"""Club catalogue, footballer attribute profiles, squad intervals, and frozen FixtureSnapshot builder."""

from __future__ import annotations

import copy
from dataclasses import dataclass, field
from typing import Any

GAMEPLAY_VERSION = "0.1.0-5v5"


def _gk_attrs(gk: int, pos: int, passing: int) -> dict[str, int]:
    return {
        "pace": 56,
        "acceleration": 58,
        "dribbling": 48,
        "passing": passing,
        "shooting": 35,
        "tackling": 42,
        "positioning": pos,
        "stamina": 78,
        "strength": 76,
        "goalkeeping": gk,
    }


def _def_attrs(pace: int, tackle: int, passing: int, strength: int) -> dict[str, int]:
    return {
        "pace": pace,
        "acceleration": max(30, pace - 2),
        "dribbling": 70,
        "passing": passing,
        "shooting": 60,
        "tackling": tackle,
        "positioning": tackle,
        "stamina": 84,
        "strength": strength,
        "goalkeeping": 15,
    }


def _mid_attrs(pace: int, drib: int, passing: int, shoot: int, tackle: int) -> dict[str, int]:
    return {
        "pace": pace,
        "acceleration": pace,
        "dribbling": drib,
        "passing": passing,
        "shooting": shoot,
        "tackling": tackle,
        "positioning": 82,
        "stamina": 88,
        "strength": 72,
        "goalkeeping": 12,
    }


def _fwd_attrs(pace: int, drib: int, passing: int, shoot: int, strength: int) -> dict[str, int]:
    return {
        "pace": pace,
        "acceleration": min(99, pace + 2),
        "dribbling": drib,
        "passing": passing,
        "shooting": shoot,
        "tackling": 44,
        "positioning": 85,
        "stamina": 82,
        "strength": strength,
        "goalkeeping": 10,
    }


def match_format_preset(format_name: str = "5v5") -> dict[str, Any]:
    """Return parameterized MatchFormat dictionary matching psfc-sim (ADR-0003)."""
    if format_name == "11v11":
        return {
            "name": "11v11",
            "players_per_side": 11,
            "pitch_length_mm": 105_000,
            "pitch_width_mm": 68_000,
            "goal_width_mm": 7_320,
            "goal_height_mm": 2_440,
            "penalty_box_depth_mm": 16_500,
            "penalty_box_width_mm": 40_320,
            "center_circle_radius_mm": 9_150,
        }
    return {
        "name": "5v5",
        "players_per_side": 5,
        "pitch_length_mm": 56_000,
        "pitch_width_mm": 36_000,
        "goal_width_mm": 4_000,
        "goal_height_mm": 2_100,
        "penalty_box_depth_mm": 9_000,
        "penalty_box_width_mm": 18_000,
        "center_circle_radius_mm": 5_500,
    }


def default_ruleset(
    format_name: str = "5v5",
    home_controller: dict[str, Any] | None = None,
    away_controller: dict[str, Any] | None = None,
    half_duration_ticks: int | None = None,
    extra_time_enabled: bool = False,
) -> dict[str, Any]:
    is_11 = format_name == "11v11"
    return {
        "half_duration_ticks": half_duration_ticks or (14_400 if is_11 else 7_200),
        "offsides_enabled": is_11,
        "fouls_enabled": True,
        "extra_time_enabled": extra_time_enabled,
        "home_controller": home_controller or {"kind": "Human"},
        "away_controller": away_controller or {"kind": "Cpu", "difficulty": "Normal"},
    }


@dataclass
class SquadMemberRecord:
    footballer_id: str
    name: str
    shirt_number: int
    role: str  # Goalkeeper | Defender | Midfielder | Forward
    attributes: dict[str, int]
    starter_5v5: bool = False
    effective_from: str = "2026-08-01T00:00:00Z"
    effective_to: str | None = None


@dataclass
class ClubRecord:
    club_id: str
    name: str
    short_name: str
    country: str
    stadium: str
    primary_color: str
    secondary_color: str
    formation_5v5: str = "1-2-1"
    formation_11v11: str = "4-3-3"
    squad: list[SquadMemberRecord] = field(default_factory=list)


class ClubCatalogue:
    """In-memory / World-scoped club and footballer catalogue with historical interval support."""

    def __init__(self) -> None:
        self.clubs: dict[str, ClubRecord] = {}
        self._load_defaults()

    def _load_defaults(self) -> None:
        self.clubs = {
            "man-utd": ClubRecord(
                club_id="man-utd",
                name="Manchester United",
                short_name="MUN",
                country="England",
                stadium="Old Trafford",
                primary_color="#DA291C",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    # 5v5 first-choice five (1 GK + 4 outfield)
                    SquadMemberRecord("mun-gk-1", "A. Onana", 24, "Goalkeeper", _gk_attrs(83, 81, 84), starter_5v5=True),
                    SquadMemberRecord("mun-def-1", "L. Martinez", 6, "Defender", _def_attrs(77, 86, 83, 82), starter_5v5=True),
                    SquadMemberRecord("mun-mid-1", "K. Mainoo", 37, "Midfielder", _mid_attrs(79, 85, 83, 74, 77), starter_5v5=True),
                    SquadMemberRecord("mun-mid-2", "B. Fernandes", 8, "Midfielder", _mid_attrs(76, 84, 90, 86, 70), starter_5v5=True),
                    SquadMemberRecord("mun-fwd-1", "M. Rashford", 10, "Forward", _fwd_attrs(90, 85, 78, 84, 76), starter_5v5=True),
                    # Additional 11v11 starters
                    SquadMemberRecord("mun-def-2", "D. Dalot", 20, "Defender", _def_attrs(85, 80, 78, 76)),
                    SquadMemberRecord("mun-def-3", "M. de Ligt", 4, "Defender", _def_attrs(74, 85, 75, 88)),
                    SquadMemberRecord("mun-def-4", "L. Shaw", 23, "Defender", _def_attrs(80, 81, 80, 79)),
                    SquadMemberRecord("mun-mid-3", "Casemiro", 18, "Midfielder", _mid_attrs(66, 74, 79, 73, 86)),
                    SquadMemberRecord("mun-fwd-2", "A. Garnacho", 17, "Forward", _fwd_attrs(89, 84, 75, 79, 67)),
                    SquadMemberRecord("mun-fwd-3", "R. Hojlund", 9, "Forward", _fwd_attrs(86, 78, 70, 81, 83)),
                ],
            ),
            "arsenal": ClubRecord(
                club_id="arsenal",
                name="Arsenal",
                short_name="ARS",
                country="England",
                stadium="Emirates Stadium",
                primary_color="#EF0107",
                secondary_color="#063672",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("ars-gk-1", "D. Raya", 22, "Goalkeeper", _gk_attrs(85, 84, 83), starter_5v5=True),
                    SquadMemberRecord("ars-def-1", "W. Saliba", 2, "Defender", _def_attrs(83, 88, 80, 86), starter_5v5=True),
                    SquadMemberRecord("ars-mid-1", "D. Rice", 41, "Midfielder", _mid_attrs(79, 82, 84, 76, 87), starter_5v5=True),
                    SquadMemberRecord("ars-mid-2", "M. Odegaard", 8, "Midfielder", _mid_attrs(78, 89, 90, 82, 67), starter_5v5=True),
                    SquadMemberRecord("ars-fwd-1", "B. Saka", 7, "Forward", _fwd_attrs(88, 88, 84, 85, 74), starter_5v5=True),
                    SquadMemberRecord("ars-def-2", "B. White", 4, "Defender", _def_attrs(80, 84, 79, 78)),
                    SquadMemberRecord("ars-def-3", "Gabriel", 6, "Defender", _def_attrs(77, 87, 75, 88)),
                    SquadMemberRecord("ars-def-4", "J. Timber", 12, "Defender", _def_attrs(83, 82, 80, 79)),
                    SquadMemberRecord("ars-mid-3", "T. Partey", 5, "Midfielder", _mid_attrs(73, 80, 83, 72, 83)),
                    SquadMemberRecord("ars-fwd-2", "G. Martinelli", 11, "Forward", _fwd_attrs(91, 86, 77, 80, 70)),
                    SquadMemberRecord("ars-fwd-3", "K. Havertz", 29, "Forward", _fwd_attrs(81, 82, 80, 81, 79)),
                ],
            ),
            "liverpool": ClubRecord(
                club_id="liverpool",
                name="Liverpool",
                short_name="LIV",
                country="England",
                stadium="Anfield",
                primary_color="#C8102E",
                secondary_color="#F6EB61",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("liv-gk-1", "Alisson", 1, "Goalkeeper", _gk_attrs(89, 88, 85), starter_5v5=True),
                    SquadMemberRecord("liv-def-1", "V. van Dijk", 4, "Defender", _def_attrs(79, 90, 81, 91), starter_5v5=True),
                    SquadMemberRecord("liv-mid-1", "A. Mac Allister", 10, "Midfielder", _mid_attrs(75, 85, 87, 82, 79), starter_5v5=True),
                    SquadMemberRecord("liv-mid-2", "D. Szoboszlai", 8, "Midfielder", _mid_attrs(84, 84, 85, 84, 70), starter_5v5=True),
                    SquadMemberRecord("liv-fwd-1", "M. Salah", 11, "Forward", _fwd_attrs(89, 88, 84, 89, 76), starter_5v5=True),
                    SquadMemberRecord("liv-def-2", "T. Alexander-Arnold", 66, "Defender", _def_attrs(78, 76, 91, 73)),
                    SquadMemberRecord("liv-def-3", "I. Konate", 5, "Defender", _def_attrs(82, 85, 73, 89)),
                    SquadMemberRecord("liv-def-4", "A. Robertson", 26, "Defender", _def_attrs(82, 81, 83, 75)),
                    SquadMemberRecord("liv-mid-3", "R. Gravenberch", 38, "Midfielder", _mid_attrs(80, 84, 82, 75, 78)),
                    SquadMemberRecord("liv-fwd-2", "L. Diaz", 7, "Forward", _fwd_attrs(90, 88, 78, 81, 72)),
                    SquadMemberRecord("liv-fwd-3", "D. Jota", 20, "Forward", _fwd_attrs(84, 84, 76, 85, 74)),
                ],
            ),
            "chelsea": ClubRecord(
                club_id="chelsea",
                name="Chelsea",
                short_name="CHE",
                country="England",
                stadium="Stamford Bridge",
                primary_color="#034694",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("che-gk-1", "R. Sanchez", 1, "Goalkeeper", _gk_attrs(80, 79, 78), starter_5v5=True),
                    SquadMemberRecord("che-def-1", "L. Colwill", 6, "Defender", _def_attrs(79, 82, 81, 82), starter_5v5=True),
                    SquadMemberRecord("che-mid-1", "M. Caicedo", 25, "Midfielder", _mid_attrs(80, 80, 82, 68, 85), starter_5v5=True),
                    SquadMemberRecord("che-mid-2", "C. Palmer", 20, "Midfielder", _mid_attrs(82, 88, 88, 87, 56), starter_5v5=True),
                    SquadMemberRecord("che-fwd-1", "N. Jackson", 15, "Forward", _fwd_attrs(88, 81, 74, 79, 78), starter_5v5=True),
                    SquadMemberRecord("che-def-2", "R. James", 24, "Defender", _def_attrs(81, 84, 84, 84)),
                    SquadMemberRecord("che-def-3", "W. Fofana", 29, "Defender", _def_attrs(82, 82, 74, 81)),
                    SquadMemberRecord("che-def-4", "M. Cucurella", 3, "Defender", _def_attrs(79, 81, 79, 72)),
                    SquadMemberRecord("che-mid-3", "E. Fernandez", 8, "Midfielder", _mid_attrs(73, 82, 86, 76, 75)),
                    SquadMemberRecord("che-fwd-2", "P. Neto", 7, "Forward", _fwd_attrs(90, 85, 79, 77, 68)),
                    SquadMemberRecord("che-fwd-3", "C. Nkunku", 18, "Forward", _fwd_attrs(83, 86, 82, 83, 71)),
                ],
            ),
        }

    def all_footballers(self) -> dict[str, dict[str, Any]]:
        out: dict[str, dict[str, Any]] = {}
        for club in self.clubs.values():
            for m in club.squad:
                if m.effective_to is None or m.footballer_id not in out:
                    out[m.footballer_id] = {
                        "footballer_id": m.footballer_id,
                        "name": m.name,
                        "club_id": club.club_id,
                        "club_short": club.short_name,
                        "shirt_number": m.shirt_number,
                        "role": m.role,
                    }
        return out

    def record_transfer(
        self,
        footballer_id: str,
        from_club_id: str,
        to_club_id: str,
        occurred_at: str,
        new_shirt: int = 99,
        starter_5v5: bool = True,
    ) -> SquadMemberRecord:
        """Close the previous club interval and open a new interval at `to_club_id` without mutating frozen snapshots."""
        src = self.clubs[from_club_id]
        dst = self.clubs[to_club_id]
        moving: SquadMemberRecord | None = None
        for m in src.squad:
            if m.footballer_id == footballer_id and m.effective_to is None:
                m.effective_to = occurred_at
                moving = m
                break
        if moving is None:
            raise ValueError(f"Active footballer '{footballer_id}' not found at '{from_club_id}'")

        if starter_5v5:
            # Demote one existing non-GK 5v5 starter on destination club so 5v5 stays 1 GK + 4 outfield
            for existing in reversed(dst.squad):
                if existing.effective_to is None and existing.starter_5v5 and existing.role != "Goalkeeper":
                    existing.starter_5v5 = False
                    break

        joined = SquadMemberRecord(
            footballer_id=moving.footballer_id,
            name=moving.name,
            shirt_number=new_shirt,
            role=moving.role,
            attributes=copy.deepcopy(moving.attributes),
            starter_5v5=starter_5v5,
            effective_from=occurred_at,
            effective_to=None,
        )
        dst.squad.append(joined)
        return joined

    def build_team_snapshot(self, club_id: str, format_name: str = "5v5") -> dict[str, Any]:
        club = self.clubs[club_id]
        active = [m for m in club.squad if m.effective_to is None]
        gks = [m for m in active if m.role == "Goalkeeper"]
        if not gks:
            raise ValueError(f"Club '{club_id}' has no active Goalkeeper")

        if format_name == "11v11":
            outfield = [m for m in active if m.role != "Goalkeeper"][:10]
            selected = [gks[0], *outfield]
            rest = [m for m in active if m not in selected]
            formation = club.formation_11v11
        else:
            preferred = [m for m in active if m.role != "Goalkeeper" and m.starter_5v5]
            if len(preferred) < 4:
                extras = [m for m in active if m.role != "Goalkeeper" and m not in preferred]
                preferred.extend(extras[: 4 - len(preferred)])
            selected = [gks[0], *preferred[:4]]
            rest = [m for m in active if m not in selected]
            formation = club.formation_5v5

        def _serialize_member(m: SquadMemberRecord) -> dict[str, Any]:
            return {
                "footballer_id": m.footballer_id,
                "name": m.name,
                "shirt_number": m.shirt_number,
                "role": m.role,
                "attributes": copy.deepcopy(m.attributes),
            }

        return {
            "club_id": club.club_id,
            "club_name": club.name,
            "short_name": club.short_name,
            "primary_color": club.primary_color,
            "secondary_color": club.secondary_color,
            "formation": formation,
            "starters": [_serialize_member(m) for m in selected],
            "substitutes": [_serialize_member(m) for m in rest],
        }

    def club_strength_ratings(self, club_id: str, format_name: str = "5v5") -> tuple[float, float]:
        """Return `(attack_rating, defense_rating)` in `[0, 100]` from active starters."""
        snap = self.build_team_snapshot(club_id, format_name=format_name)
        starters = snap["starters"]
        gk = starters[0]["attributes"]
        outfield = [p["attributes"] for p in starters[1:]]
        att = sum(
            0.45 * p["shooting"] + 0.30 * p["passing"] + 0.25 * p["dribbling"]
            for p in outfield
        ) / max(1, len(outfield))
        dfn_out = sum(
            0.50 * p["tackling"] + 0.30 * p["positioning"] + 0.20 * p["strength"]
            for p in outfield
        ) / max(1, len(outfield))
        dfn = 0.55 * dfn_out + 0.45 * (0.65 * gk["goalkeeping"] + 0.35 * gk["positioning"])
        return (att, dfn)

    def build_fixture_snapshot(
        self,
        fixture_id: str,
        home_club_id: str,
        away_club_id: str,
        seed: int,
        format_name: str = "5v5",
        home_controller: dict[str, Any] | None = None,
        away_controller: dict[str, Any] | None = None,
        half_duration_ticks: int | None = None,
        extra_time_enabled: bool = False,
    ) -> dict[str, Any]:
        return {
            "fixture_id": fixture_id,
            "gameplay_version": GAMEPLAY_VERSION,
            "seed": seed,
            "format": match_format_preset(format_name),
            "rules": default_ruleset(
                format_name=format_name,
                home_controller=home_controller,
                away_controller=away_controller,
                half_duration_ticks=half_duration_ticks,
                extra_time_enabled=extra_time_enabled,
            ),
            "home": self.build_team_snapshot(home_club_id, format_name=format_name),
            "away": self.build_team_snapshot(away_club_id, format_name=format_name),
        }
