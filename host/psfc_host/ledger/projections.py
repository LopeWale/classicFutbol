"""Pure derived projections from the append-only `world_event` log.

Standings and footballer statistics (`inherited`, `earned`, `combined`) are NEVER
mutated directly or overwritten by external season totals. Corrections (`result_corrected`
with `supersedes_seq`) replace superseded events during projection rebuilds without
double-counting.
"""

from __future__ import annotations

from typing import Any


RESULT_EVENT_TYPES = {
    "result_accepted",
    "imported_result_accepted",
    "generated_result_accepted",
    "result_corrected",
}

MATCH_STAT_EVENT_TYPES = {
    "match_events_recorded",
    "imported_match_events_recorded",
}


def _resolve_active_fixture_outcomes(events: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    """Replay the event log in `seq` order, applying `supersedes_seq` corrections cleanly."""
    superseded_seqs: set[int] = {
        int(ev["supersedes_seq"])
        for ev in events
        if ev.get("supersedes_seq") is not None
    }

    fixtures_meta: dict[str, dict[str, Any]] = {}
    outcomes: dict[str, dict[str, Any]] = {}

    for ev in events:
        seq = int(ev["seq"])
        etype = ev["type"]
        payload = ev["payload"]

        if etype == "fixture_created":
            fid = payload["fixture_id"]
            fixtures_meta[fid] = {
                "fixture_id": fid,
                "round": payload.get("round", 1),
                "home_club_id": payload["home_club_id"],
                "away_club_id": payload["away_club_id"],
                "real_reference_score": payload.get("real_reference_score"),
            }
            continue

        if seq in superseded_seqs:
            continue

        if etype in RESULT_EVENT_TYPES:
            fid = payload["fixture_id"]
            meta = fixtures_meta.get(fid, {})
            source = payload.get("source") or (
                "imported" if etype == "imported_result_accepted" else "played_vs_cpu"
            )
            outcomes[fid] = {
                "seq": seq,
                "fixture_id": fid,
                "home_club_id": payload.get("home_club_id") or meta.get("home_club_id"),
                "away_club_id": payload.get("away_club_id") or meta.get("away_club_id"),
                "home_goals": int(payload["home_goals"]),
                "away_goals": int(payload["away_goals"]),
                "source": source,
                "events": list(payload.get("events", [])),
            }
        elif etype in MATCH_STAT_EVENT_TYPES:
            fid = payload["fixture_id"]
            # If the parent result event for this fixture was superseded by a `result_corrected`
            # with a higher seq, ignore the older match_events_recorded!
            cur = outcomes.get(fid)
            if cur is not None and cur["seq"] > seq:
                continue
            if cur is not None:
                cur["events"] = list(payload.get("events", []))

    return outcomes


def project_standings(
    events: list[dict[str, Any]],
    clubs: dict[str, dict[str, Any]],
    represented_club_ids: set[str] | None = None,
) -> dict[str, Any]:
    """Rebuild Parallel Season league standings + Real-World reference standings from `world_event`."""
    rep = represented_club_ids or set()
    outcomes = _resolve_active_fixture_outcomes(events)

    table: dict[str, dict[str, Any]] = {}
    real_pts: dict[str, int] = {cid: 0 for cid in clubs}

    for cid, cinfo in clubs.items():
        table[cid] = {
            "club_id": cid,
            "club_name": cinfo["name"],
            "short_name": cinfo["short_name"],
            "primary_color": cinfo["primary_color"],
            "represented": cid in rep,
            "played": 0,
            "won": 0,
            "drawn": 0,
            "lost": 0,
            "gf": 0,
            "ga": 0,
            "gd": 0,
            "points": 0,
            "inherited_points": 0,
            "earned_points": 0,
            "form": [],
        }

    # Compute Real-World reference points from `fixture_created` real_reference_score when available
    for ev in events:
        if ev["type"] == "fixture_created":
            pl = ev["payload"]
            ref = pl.get("real_reference_score")
            if ref and len(ref) == 2 and ref[0] is not None and ref[1] is not None:
                hg, ag = int(ref[0]), int(ref[1])
                hc, ac = pl["home_club_id"], pl["away_club_id"]
                if hc in real_pts and ac in real_pts:
                    if hg > ag:
                        real_pts[hc] += 3
                    elif ag > hg:
                        real_pts[ac] += 3
                    else:
                        real_pts[hc] += 1
                        real_pts[ac] += 1

    # Accumulate Parallel Season outcomes
    for out in outcomes.values():
        hc = out["home_club_id"]
        ac = out["away_club_id"]
        if hc not in table or ac not in table:
            continue
        hg = out["home_goals"]
        ag = out["away_goals"]
        is_inherited = out["source"] == "imported"

        h_row = table[hc]
        a_row = table[ac]
        h_row["played"] += 1
        a_row["played"] += 1
        h_row["gf"] += hg
        h_row["ga"] += ag
        a_row["gf"] += ag
        a_row["ga"] += hg

        if hg > ag:
            h_pts, a_pts = 3, 0
            h_res, a_res = "W", "L"
            h_row["won"] += 1
            a_row["lost"] += 1
        elif ag > hg:
            h_pts, a_pts = 0, 3
            h_res, a_res = "L", "W"
            a_row["won"] += 1
            h_row["lost"] += 1
        else:
            h_pts, a_pts = 1, 1
            h_res, a_res = "D", "D"
            h_row["drawn"] += 1
            a_row["drawn"] += 1

        h_row["points"] += h_pts
        a_row["points"] += a_pts
        if is_inherited:
            h_row["inherited_points"] += h_pts
            a_row["inherited_points"] += a_pts
        else:
            h_row["earned_points"] += h_pts
            a_row["earned_points"] += a_pts

        h_row["form"].append(h_res)
        a_row["form"].append(a_res)

    rows = list(table.values())
    for r in rows:
        r["gd"] = r["gf"] - r["ga"]
        r["form"] = r["form"][-5:]
        r["real_reference_points"] = real_pts.get(r["club_id"], 0)
        r["delta_vs_real"] = r["points"] - r["real_reference_points"]

    rows.sort(key=lambda r: (-r["points"], -r["gd"], -r["gf"], r["club_name"]))
    for idx, r in enumerate(rows, start=1):
        r["position"] = idx

    return {
        "standings": rows,
        "accepted_fixture_count": len(outcomes),
    }


def _empty_stat_bucket() -> dict[str, int]:
    return {
        "goals": 0,
        "assists": 0,
        "shots": 0,
        "saves": 0,
        "tackles": 0,
        "yellow_cards": 0,
        "red_cards": 0,
    }


def project_player_stats(
    events: list[dict[str, Any]],
    footballers: dict[str, dict[str, Any]],
) -> list[dict[str, Any]]:
    """Project `inherited`, `earned`, and `combined` footballer statistics strictly from match events."""
    outcomes = _resolve_active_fixture_outcomes(events)
    stats_by_player: dict[str, dict[str, Any]] = {}

    def _ensure_player(fid: str) -> dict[str, Any]:
        if fid not in stats_by_player:
            meta = footballers.get(
                fid,
                {
                    "footballer_id": fid,
                    "name": fid,
                    "club_id": "unknown",
                    "club_short": "UNK",
                    "shirt_number": 0,
                    "role": "Outfield",
                },
            )
            stats_by_player[fid] = {
                **meta,
                "inherited": _empty_stat_bucket(),
                "earned": _empty_stat_bucket(),
                "combined": _empty_stat_bucket(),
            }
        return stats_by_player[fid]

    for fid in footballers:
        _ensure_player(fid)

    for out in outcomes.values():
        bucket_key = "inherited" if out["source"] == "imported" else "earned"
        for mev in out["events"]:
            etype = str(mev.get("event_type") or mev.get("type") or "")
            fid = mev.get("footballer_id")
            assist_id = mev.get("assist_footballer_id")

            if fid:
                p_entry = _ensure_player(fid)
                b = p_entry[bucket_key]
                if etype == "Goal":
                    b["goals"] += 1
                    b["shots"] += 1
                elif etype == "Shot":
                    b["shots"] += 1
                elif etype == "Save":
                    b["saves"] += 1
                elif etype == "Tackle":
                    b["tackles"] += 1
                elif etype == "YellowCard":
                    b["yellow_cards"] += 1
                elif etype == "RedCard":
                    b["red_cards"] += 1

            if etype == "Goal" and assist_id:
                a_entry = _ensure_player(assist_id)
                a_entry[bucket_key]["assists"] += 1

    result_list: list[dict[str, Any]] = []
    for entry in stats_by_player.values():
        inh = entry["inherited"]
        ear = entry["earned"]
        comb = entry["combined"]
        for k in comb:
            comb[k] = inh[k] + ear[k]
        result_list.append(entry)

    result_list.sort(
        key=lambda r: (
            -r["combined"]["goals"],
            -r["combined"]["assists"],
            -r["combined"]["saves"],
            r["name"],
        )
    )
    return result_list
