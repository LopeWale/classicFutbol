"""Competition Divergence Engine, Deterministic Poisson Generator, and Season Rollover (Phase 5)."""

from __future__ import annotations

from datetime import datetime, timedelta, timezone
import hashlib
import math
import random
import uuid
from typing import Any

from psfc_host.catalogue.clubs import ClubCatalogue
from psfc_host.ledger.projections import project_player_stats, project_standings
from psfc_host.ledger.store import WorldLedgerStore, utc_now_iso


POISSON_MODEL_VERSION = "poisson_attr_v1"


def _fixture_winner_club_id(fx: dict[str, Any]) -> str | None:
    """Return winning club_id of an accepted knockout tie (including penalty shootouts)."""
    hg = fx.get("accepted_home_goals")
    ag = fx.get("accepted_away_goals")
    if hg is None or ag is None:
        return None
    if int(hg) > int(ag):
        return str(fx["home_club_id"])
    if int(ag) > int(hg):
        return str(fx["away_club_id"])
    sh = fx.get("accepted_shootout_home")
    sa = fx.get("accepted_shootout_away")
    if sh is not None and sa is not None and int(sh) != int(sa):
        return str(fx["home_club_id"]) if int(sh) > int(sa) else str(fx["away_club_id"])
    return None


def _sample_poisson(rng: random.Random, lam: float) -> int:
    """Knuth inverse-transform Poisson sampler driven by a seeded `random.Random`."""
    threshold = math.exp(-max(0.05, min(lam, 12.0)))
    k = 0
    p = 1.0
    while True:
        k += 1
        p *= rng.random()
        if p <= threshold:
            return k - 1


class CompetitionEngine:
    """Handles knockout progression, real-vs-parallel divergence, Poisson outcomes, and season rollover."""

    def __init__(self, store: WorldLedgerStore, catalogue: ClubCatalogue) -> None:
        self.store = store
        self.catalogue = catalogue

    def _clubs_summary(self) -> dict[str, dict[str, Any]]:
        return {
            cid: {
                "club_id": c.club_id,
                "name": c.name,
                "short_name": c.short_name,
                "country": c.country,
                "stadium": c.stadium,
                "primary_color": c.primary_color,
                "secondary_color": c.secondary_color,
            }
            for cid, c in self.catalogue.clubs.items()
        }

    def advance_knockout_competition(
        self,
        *,
        world_id: str,
        competition_id: str,
        actor_id: str = "competition_engine",
    ) -> dict[str, Any]:
        """Advance a knockout competition from Semi-Finals to the Final.

        - If Parallel Semi-Final winners match `real_final_pair` (even in reversed order), links
          1:1 to the real-world Final (`has_real_equivalent=True` -> `imported` if unrepresented).
        - If Parallel Semi-Final winners DIVERGE from real football, generates a Parallel-Only Final
          (`has_real_equivalent=False` -> `played_vs_friend`, `played_vs_cpu`, or `generated`)!
        """
        comp = self.store.get_competition(competition_id)
        if comp["kind"] != "knockout":
            raise ValueError(f"Competition '{competition_id}' is not a knockout tournament")

        fixtures = self.store.list_fixtures(world_id, competition_id=competition_id)
        existing_finals = [f for f in fixtures if f.get("stage") == "cup_final"]
        if existing_finals:
            return {
                "already_generated": True,
                "final_fixture": existing_finals[0],
            }

        semis = [f for f in fixtures if f.get("stage") == "cup_sf"]
        if len(semis) < 2:
            raise ValueError("At least 2 Semi-Final fixtures ('cup_sf') are required")

        winners: list[str] = []
        for sf in semis[:2]:
            w = _fixture_winner_club_id(sf)
            if w is None:
                raise ValueError(
                    f"Semi-Final '{sf['id']}' does not yet have a decisive winner (play or resolve it first)"
                )
            winners.append(w)

        home_finalist, away_finalist = winners[0], winners[1]
        real_pair = comp.get("real_final_pair") or []
        matches_real = len(real_pair) == 2 and set(winners) == set(real_pair)
        diverged = not matches_real

        result_source = self.store.compute_advance_result_source(
            world_id,
            home_finalist,
            away_finalist,
            has_real_equivalent=matches_real,
        )

        final_id = f"fix-cup-final-{home_finalist[:3]}-{away_finalist[:3]}"
        self.store.append_event(
            world_id=world_id,
            event_type="draw_generated",
            aggregate_type="competition",
            aggregate_id=competition_id,
            payload={
                "competition_id": competition_id,
                "stage": "cup_final",
                "home_club_id": home_finalist,
                "away_club_id": away_finalist,
                "real_final_pair": real_pair,
                "diverged_from_real_world": diverged,
                "assigned_result_source": result_source,
            },
            caused_by=actor_id,
        )

        latest_sf_ko = max(str(sf["kickoff_utc"]) for sf in semis)
        sf_dt = datetime.fromisoformat(latest_sf_ko.replace("Z", "+00:00")).astimezone(timezone.utc)
        final_ko_dt = (sf_dt + timedelta(days=26)).replace(hour=16, minute=30, second=0)
        final_close_dt = (final_ko_dt + timedelta(days=14)).replace(hour=23, minute=59, second=59)
        final_open_utc = min(str(sf["window_open_utc"]) for sf in semis)

        final_fx = self.store.create_fixture(
            fixture_id=final_id,
            world_id=world_id,
            competition_id=competition_id,
            stage="cup_final",
            round_no=max(int(sf["round"]) for sf in semis) + 1,
            home_club_id=home_finalist,
            away_club_id=away_finalist,
            kickoff_utc=final_ko_dt.strftime("%Y-%m-%dT%H:%M:%SZ"),
            window_open_utc=final_open_utc,
            window_close_utc=final_close_dt.strftime("%Y-%m-%dT%H:%M:%SZ"),
            result_source=result_source,
            state="open",
            extra_time_enabled=True,
            real_home_goals=1 if matches_real else None,
            real_away_goals=2 if matches_real else None,
            caused_by=actor_id,
        )
        return {
            "already_generated": False,
            "diverged_from_real_world": diverged,
            "final_fixture": final_fx,
        }

    def generate_poisson_fixture_result(
        self,
        *,
        fixture_id: str,
        seed: int | None = None,
        actor_id: str = "poisson_generator",
    ) -> dict[str, Any]:
        """Deterministically generate a match outcome for an unrepresented diverging fixture (`result_source == 'generated'`)."""
        fx = self.store.get_fixture(fixture_id)
        if fx["state"] == "accepted":
            return {
                "idempotent_duplicate": True,
                "accepted_result": self.store.get_accepted_result(fixture_id),
            }
        if fx["result_source"] != "generated":
            raise ValueError(
                f"Fixture '{fixture_id}' has result_source='{fx['result_source']}', expected 'generated'"
            )

        world_id = fx["world_id"]
        world = self.store.get_world(world_id)
        fmt = world.get("default_match_format", "5v5")

        if seed is None:
            digest = hashlib.sha256(f"poisson:{fixture_id}:{POISSON_MODEL_VERSION}".encode("utf-8")).digest()
            seed = int.from_bytes(digest[:8], "little")

        rng = random.Random(seed)
        world_cat = self.catalogue.for_world(self.store, world_id)
        h_att, h_def = world_cat.club_strength_ratings(fx["home_club_id"], format_name=fmt)
        a_att, a_def = world_cat.club_strength_ratings(fx["away_club_id"], format_name=fmt)

        # Expected goals scaled by attack vs opponent defense + mild home advantage
        lam_home = max(0.35, 1.45 * math.exp((h_att - a_def) / 42.0) * 1.08)
        lam_away = max(0.30, 1.35 * math.exp((a_att - h_def) / 42.0))

        home_goals = _sample_poisson(rng, lam_home)
        away_goals = _sample_poisson(rng, lam_away)
        went_aet = False
        sh_home: int | None = None
        sh_away: int | None = None

        if bool(fx.get("extra_time_enabled", 0)) and home_goals == away_goals:
            went_aet = True
            home_goals += _sample_poisson(rng, lam_home * 0.33)
            away_goals += _sample_poisson(rng, lam_away * 0.33)
            if home_goals == away_goals:
                # Deterministic ABAB 5-kick + sudden-death shootout
                hp, ap = 0, 0
                for r in range(20):
                    if rng.random() < 0.74:
                        hp += 1
                    if r < 5 and (hp > ap + (5 - r) or ap > hp + (4 - r)):
                        break
                    if rng.random() < 0.72:
                        ap += 1
                    if r < 5:
                        rem = 4 - r
                        if hp > ap + rem or ap > hp + rem:
                            break
                    elif hp != ap:
                        break
                if hp == ap:
                    hp += 1
                sh_home, sh_away = hp, ap

        # Synthesize goal events attributed to active starters weighted by shooting attribute
        h_snap = world_cat.build_team_snapshot(fx["home_club_id"], format_name=fmt)
        a_snap = world_cat.build_team_snapshot(fx["away_club_id"], format_name=fmt)

        def _pick_scorer_and_assist(starters: list[dict[str, Any]]) -> tuple[str, str | None]:
            outfield = starters[1:] or starters
            weights = [max(10, int(p["attributes"]["shooting"]) ** 2) for p in outfield]
            scorer = rng.choices(outfield, weights=weights, k=1)[0]
            mates = [p for p in outfield if p["footballer_id"] != scorer["footballer_id"]]
            assist = (
                rng.choices(
                    mates,
                    weights=[max(10, int(m["attributes"]["passing"])) for m in mates],
                    k=1,
                )[0]["footballer_id"]
                if mates and rng.random() < 0.68
                else None
            )
            return (scorer["footballer_id"], assist)

        max_min = 120 if went_aet else 90
        gen_events: list[dict[str, Any]] = []
        for _ in range(home_goals):
            sc, asst = _pick_scorer_and_assist(h_snap["starters"])
            gen_events.append(
                {
                    "event_type": "goal",
                    "footballer_id": sc,
                    "assist_footballer_id": asst,
                    "minute": rng.randint(4, max_min),
                }
            )
        for _ in range(away_goals):
            sc, asst = _pick_scorer_and_assist(a_snap["starters"])
            gen_events.append(
                {
                    "event_type": "goal",
                    "footballer_id": sc,
                    "assist_footballer_id": asst,
                    "minute": rng.randint(4, max_min),
                }
            )
        gen_events.sort(key=lambda e: int(e["minute"]))

        ts = utc_now_iso()
        res_ev = self.store.append_event(
            world_id=world_id,
            event_type="generated_result_accepted",
            aggregate_type="fixture",
            aggregate_id=fixture_id,
            payload={
                "fixture_id": fixture_id,
                "competition_id": fx.get("competition_id"),
                "stage": fx.get("stage", "league"),
                "home_club_id": fx["home_club_id"],
                "away_club_id": fx["away_club_id"],
                "home_goals": home_goals,
                "away_goals": away_goals,
                "went_to_extra_time": went_aet,
                "shootout_score": [sh_home, sh_away] if sh_home is not None else None,
                "source": "generated",
                "seed": seed,
                "model_version": POISSON_MODEL_VERSION,
                "expected_goals": [round(lam_home, 3), round(lam_away, 3)],
            },
            caused_by=actor_id,
            occurred_at=ts,
            commit=False,
        )
        if gen_events:
            self.store.append_event(
                world_id=world_id,
                event_type="match_events_recorded",
                aggregate_type="fixture",
                aggregate_id=fixture_id,
                payload={
                    "fixture_id": fixture_id,
                    "source": "generated",
                    "events": gen_events,
                },
                caused_by=actor_id,
                occurred_at=ts,
                commit=False,
            )

        self.store.conn.execute(
            """
            INSERT INTO accepted_result (
                fixture_id, world_id, session_id, source,
                home_goals, away_goals, went_to_extra_time, shootout_home, shootout_away,
                final_hash_hex, accepted_at, event_seq
            )
            VALUES (?, ?, NULL, 'generated', ?, ?, ?, ?, ?, NULL, ?, ?)
            """,
            (
                fixture_id,
                world_id,
                home_goals,
                away_goals,
                int(went_aet),
                sh_home,
                sh_away,
                ts,
                res_ev["seq"],
            ),
        )
        self.store.conn.execute(
            "UPDATE fixture SET state = 'accepted' WHERE id = ?",
            (fixture_id,),
        )
        self.store.conn.commit()
        return {
            "idempotent_duplicate": False,
            "seed": seed,
            "model_version": POISSON_MODEL_VERSION,
            "accepted_result": self.store.get_accepted_result(fixture_id),
            "events": gen_events,
        }

    def complete_season_and_rollover(
        self,
        *,
        world_id: str,
        next_season_label: str = "2027/28",
        actor_id: str = "commissioner",
    ) -> dict[str, Any]:
        """Award end-of-season honours, determine qualification, and roll over to a new Parallel Season."""
        world = self.store.get_world(world_id)
        events = self.store.list_events(world_id)
        world_cat = self.catalogue.for_world(self.store, world_id)
        clubs = {
            cid: {
                "club_id": c.club_id,
                "name": c.name,
                "short_name": c.short_name,
                "country": c.country,
                "stadium": c.stadium,
                "primary_color": c.primary_color,
                "secondary_color": c.secondary_color,
            }
            for cid, c in world_cat.clubs.items()
        }
        rep = set(self.store.represented_clubs(world_id).keys())
        standings = project_standings(events, clubs, represented_club_ids=rep)["standings"]
        players = project_player_stats(
            events,
            world_cat.active_footballers(),
            historical_footballers=world_cat.all_footballers(),
        )
        ts = utc_now_iso()

        honours: list[dict[str, Any]] = []
        if standings:
            champ = standings[0]
            honours.append(
                self.store.record_honour(
                    world_id=world_id,
                    season_label=world["season_label"],
                    category="league_champion",
                    winner_id=champ["club_id"],
                    winner_name=champ["club_name"],
                    detail={"points": champ["points"], "gd": champ["gd"], "gf": champ["gf"]},
                    awarded_at=ts,
                )
            )

        # Check Knockout Cup winner if a `cup_final` fixture was accepted
        all_fx = self.store.list_fixtures(world_id)
        cup_finals = [f for f in all_fx if f.get("stage") == "cup_final"]
        cup_winner_id: str | None = None
        if cup_finals:
            cw = _fixture_winner_club_id(cup_finals[0])
            if cw and cw in clubs:
                cup_winner_id = cw
                honours.append(
                    self.store.record_honour(
                        world_id=world_id,
                        season_label=world["season_label"],
                        category="cup_winner",
                        winner_id=cw,
                        winner_name=clubs[cw]["name"],
                        detail={"fixture_id": cup_finals[0]["id"]},
                        awarded_at=ts,
                    )
                )

        if players:
            top_scorer = max(players, key=lambda p: (p["combined"]["goals"], p["combined"]["assists"]))
            honours.append(
                self.store.record_honour(
                    world_id=world_id,
                    season_label=world["season_label"],
                    category="golden_boot",
                    winner_id=top_scorer["footballer_id"],
                    winner_name=top_scorer["name"],
                    detail={"goals": top_scorer["combined"]["goals"], "club_short": top_scorer["club_short"]},
                    awarded_at=ts,
                )
            )
            keepers = [p for p in players if p.get("role") == "Goalkeeper"] or players
            top_gk = max(keepers, key=lambda p: p["combined"]["saves"])
            honours.append(
                self.store.record_honour(
                    world_id=world_id,
                    season_label=world["season_label"],
                    category="golden_glove",
                    winner_id=top_gk["footballer_id"],
                    winner_name=top_gk["name"],
                    detail={"saves": top_gk["combined"]["saves"], "club_short": top_gk["club_short"]},
                    awarded_at=ts,
                )
            )

        # Parallel Continental Qualification: Top 2 in League -> Champions Cup; 3rd or Cup Winner -> Europa Cup
        champions_cup_qualifiers = [r["club_id"] for r in standings[:2]]
        europa_qualifier = (
            cup_winner_id
            if cup_winner_id and cup_winner_id not in champions_cup_qualifiers
            else (standings[2]["club_id"] if len(standings) >= 3 else None)
        )
        qualification = {
            "parallel_champions_cup": champions_cup_qualifiers,
            "parallel_europa_cup": [europa_qualifier] if europa_qualifier else [],
        }

        self.store.append_event(
            world_id=world_id,
            event_type="honours_awarded",
            aggregate_type="world",
            aggregate_id=world_id,
            payload={"season_label": world["season_label"], "honours": honours},
            caused_by=actor_id,
            occurred_at=ts,
            commit=False,
        )
        self.store.append_event(
            world_id=world_id,
            event_type="qualification_determined",
            aggregate_type="world",
            aggregate_id=world_id,
            payload=qualification,
            caused_by=actor_id,
            occurred_at=ts,
            commit=False,
        )
        self.store.append_event(
            world_id=world_id,
            event_type="season_completed",
            aggregate_type="world",
            aggregate_id=world_id,
            payload={"season_label": world["season_label"]},
            caused_by=actor_id,
            occurred_at=ts,
            commit=False,
        )
        self.store.conn.execute("UPDATE world SET status = 'completed' WHERE id = ?", (world_id,))
        self.store.conn.execute("UPDATE competition SET status = 'completed' WHERE world_id = ?", (world_id,))
        self.store.conn.commit()

        # Roll over into next Parallel World preserving participant club assignments!
        slug = next_season_label.replace("/", "-")
        next_world_id = f"{world_id}-next-{slug}-{uuid.uuid4().hex[:4]}"
        self.store.create_world(
            world_id=next_world_id,
            name=world["name"],
            competition_name=world["competition_name"],
            season_label=next_season_label,
            season_timezone=world["season_timezone"],
            default_match_format=world["default_match_format"],
            parent_world_id=world_id,
            caused_by=actor_id,
        )
        next_league_id = f"comp-league-{next_world_id}"
        self.store.create_competition(
            competition_id=next_league_id,
            world_id=next_world_id,
            name=world["competition_name"],
            kind="league",
            format_name=world["default_match_format"],
            caused_by=actor_id,
        )

        # Carry over active club assignments from round 1 of new season
        prior_assignments = self.store.list_assignments(world_id)
        for a in prior_assignments:
            self.store.assign_club(
                assignment_id=f"asgn-{next_world_id}-{a['club_id']}",
                world_id=next_world_id,
                club_id=a["club_id"],
                participant_id=a["participant_id"],
                effective_from_round=1,
                baseline_summary={"rolled_over_from_world": world_id},
            )

        # Seed opening Matchweek 1 fixtures across all clubs in the rolled-over 2027/28 season
        club_ids = list(clubs.keys())
        pairings = [
            (club_ids[i], club_ids[i + 1]) for i in range(0, len(club_ids) - 1, 2)
        ]
        for idx, (hc, ac) in enumerate(pairings, start=1):
            src = self.store.compute_advance_result_source(next_world_id, hc, ac, has_real_equivalent=True)
            self.store.create_fixture(
                fixture_id=f"fix-{next_world_id}-r1-{idx}",
                world_id=next_world_id,
                competition_id=next_league_id,
                stage="league",
                round_no=1,
                home_club_id=hc,
                away_club_id=ac,
                kickoff_utc="2027-08-13T19:00:00Z",
                window_open_utc=ts,
                window_close_utc="2027-12-31T23:59:59Z",
                result_source=src,
                state="open",
                caused_by=actor_id,
            )

        self.store.append_event(
            world_id=world_id,
            event_type="season_rolled_over",
            aggregate_type="world",
            aggregate_id=world_id,
            payload={
                "completed_world_id": world_id,
                "next_world_id": next_world_id,
                "next_season_label": next_season_label,
                "preserved_assignments": len(prior_assignments),
                "qualification": qualification,
            },
            caused_by=actor_id,
        )

        return {
            "completed_world_id": world_id,
            "next_world": self.store.get_world(next_world_id),
            "honours": honours,
            "qualification": qualification,
        }
