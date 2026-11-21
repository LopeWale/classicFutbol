"""Club catalogue, footballer attribute profiles, squad intervals, and frozen FixtureSnapshot builder."""

from __future__ import annotations

import copy
from dataclasses import dataclass, field
from typing import Any

GAMEPLAY_VERSION = "0.1.0-5v5"
SUPPORTED_5V5_FORMATIONS = ("1-2-1", "2-2", "1-1-2")
_ROLE_ORDER = {"Goalkeeper": 0, "Defender": 1, "Midfielder": 2, "Forward": 3}


def _member_overall(m: SquadMemberRecord) -> int:
    a = m.attributes
    if m.role == "Goalkeeper":
        return round(0.62 * a["goalkeeping"] + 0.26 * a["positioning"] + 0.12 * a["passing"])
    if m.role == "Defender":
        return round(0.40 * a["tackling"] + 0.25 * a["positioning"] + 0.20 * a["strength"] + 0.15 * a["pace"])
    if m.role == "Midfielder":
        return round(0.36 * a["passing"] + 0.30 * a["dribbling"] + 0.18 * a["shooting"] + 0.16 * a["stamina"])
    return round(0.40 * a["shooting"] + 0.26 * a["pace"] + 0.22 * a["dribbling"] + 0.12 * a["passing"])


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
            "arsenal": ClubRecord(
                club_id="arsenal",
                name="Arsenal",
                short_name="ARS",
                country="England",
                stadium="Emirates Stadium",
                primary_color="#EF0107",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("ars-gk-1", "D. Raya", 1, "Goalkeeper", _gk_attrs(88, 87, 85), starter_5v5=True),
                    SquadMemberRecord("ars-def-1", "W. Saliba", 2, "Defender", _def_attrs(84, 90, 82, 87), starter_5v5=True),
                    SquadMemberRecord("ars-mid-1", "D. Rice", 41, "Midfielder", _mid_attrs(80, 83, 86, 78, 89), starter_5v5=True),
                    SquadMemberRecord("ars-mid-2", "M. Odegaard", 8, "Midfielder", _mid_attrs(79, 90, 91, 84, 68), starter_5v5=True),
                    SquadMemberRecord("ars-fwd-1", "B. Saka", 7, "Forward", _fwd_attrs(89, 90, 86, 87, 76), starter_5v5=True),
                    SquadMemberRecord("ars-def-2", "Gabriel", 6, "Defender", _def_attrs(78, 88, 76, 89)),
                    SquadMemberRecord("ars-def-3", "R. Calafiori", 33, "Defender", _def_attrs(81, 85, 83, 83)),
                    SquadMemberRecord("ars-def-4", "J. Timber", 12, "Defender", _def_attrs(84, 84, 82, 80)),
                    SquadMemberRecord("ars-mid-3", "B. Guimaraes", 39, "Midfielder", _mid_attrs(77, 85, 87, 79, 84)),
                    SquadMemberRecord("ars-fwd-2", "E. Eze", 10, "Forward", _fwd_attrs(85, 90, 84, 83, 74)),
                    SquadMemberRecord("ars-fwd-3", "V. Gyokeres", 14, "Forward", _fwd_attrs(87, 83, 77, 88, 88)),
                ],
            ),
            "brighton": ClubRecord(
                club_id="brighton",
                name="Brighton & Hove Albion",
                short_name="BHA",
                country="England",
                stadium="Falmer Stadium",
                primary_color="#0057B8",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("bha-gk-1", "B. Verbruggen", 1, "Goalkeeper", _gk_attrs(84, 83, 84), starter_5v5=True),
                    SquadMemberRecord("bha-def-1", "L. Dunk", 5, "Defender", _def_attrs(70, 84, 80, 87), starter_5v5=True),
                    SquadMemberRecord("bha-mid-1", "P. Gross", 13, "Midfielder", _mid_attrs(72, 83, 88, 80, 74), starter_5v5=True),
                    SquadMemberRecord("bha-mid-2", "J. Hinshelwood", 8, "Midfielder", _mid_attrs(79, 81, 82, 77, 79), starter_5v5=True),
                    SquadMemberRecord("bha-fwd-1", "K. Mitoma", 7, "Forward", _fwd_attrs(89, 90, 80, 81, 68), starter_5v5=True),
                    SquadMemberRecord("bha-def-2", "F. Kadioglu", 24, "Defender", _def_attrs(85, 79, 81, 73)),
                    SquadMemberRecord("bha-def-3", "M. De Cuyper", 29, "Defender", _def_attrs(81, 78, 83, 75)),
                    SquadMemberRecord("bha-def-4", "O. Boscagli", 21, "Defender", _def_attrs(76, 82, 82, 79)),
                    SquadMemberRecord("bha-mid-3", "M. Wieffer", 27, "Midfielder", _mid_attrs(75, 79, 81, 72, 83)),
                    SquadMemberRecord("bha-fwd-2", "G. Rutter", 10, "Forward", _fwd_attrs(84, 86, 80, 80, 78)),
                    SquadMemberRecord("bha-fwd-3", "Y. Minteh", 11, "Forward", _fwd_attrs(92, 84, 75, 79, 71)),
                ],
            ),
            "man-city": ClubRecord(
                club_id="man-city",
                name="Manchester City",
                short_name="MCI",
                country="England",
                stadium="Etihad Stadium",
                primary_color="#6CABDD",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("mci-gk-1", "G. Donnarumma", 1, "Goalkeeper", _gk_attrs(89, 86, 79), starter_5v5=True),
                    SquadMemberRecord("mci-def-1", "R. Dias", 3, "Defender", _def_attrs(76, 89, 82, 89), starter_5v5=True),
                    SquadMemberRecord("mci-mid-1", "E. Fernandez", 17, "Midfielder", _mid_attrs(75, 84, 88, 79, 78), starter_5v5=True),
                    SquadMemberRecord("mci-mid-2", "R. Cherki", 10, "Midfielder", _mid_attrs(82, 91, 87, 82, 55), starter_5v5=True),
                    SquadMemberRecord("mci-fwd-1", "E. Haaland", 9, "Forward", _fwd_attrs(89, 81, 73, 94, 93), starter_5v5=True),
                    SquadMemberRecord("mci-def-2", "J. Gvardiol", 24, "Defender", _def_attrs(82, 86, 83, 85)),
                    SquadMemberRecord("mci-def-3", "M. Guehi", 6, "Defender", _def_attrs(80, 85, 80, 83)),
                    SquadMemberRecord("mci-def-4", "A. Khusanov", 45, "Defender", _def_attrs(85, 82, 74, 84)),
                    SquadMemberRecord("mci-mid-3", "P. Foden", 47, "Midfielder", _mid_attrs(85, 90, 86, 86, 62)),
                    SquadMemberRecord("mci-fwd-2", "J. Doku", 11, "Forward", _fwd_attrs(93, 92, 77, 78, 69)),
                    SquadMemberRecord("mci-fwd-3", "A. Semenyo", 42, "Forward", _fwd_attrs(88, 84, 76, 82, 83)),
                ],
            ),
            "bournemouth": ClubRecord(
                club_id="bournemouth",
                name="AFC Bournemouth",
                short_name="BOU",
                country="England",
                stadium="Vitality Stadium",
                primary_color="#DA291C",
                secondary_color="#111827",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("bou-gk-1", "D. Petrovic", 1, "Goalkeeper", _gk_attrs(82, 81, 77), starter_5v5=True),
                    SquadMemberRecord("bou-def-1", "A. Silva", 14, "Defender", _def_attrs(79, 83, 78, 82), starter_5v5=True),
                    SquadMemberRecord("bou-mid-1", "A. Scott", 8, "Midfielder", _mid_attrs(79, 83, 82, 74, 75), starter_5v5=True),
                    SquadMemberRecord("bou-mid-2", "M. Tavernier", 16, "Midfielder", _mid_attrs(84, 81, 80, 80, 70), starter_5v5=True),
                    SquadMemberRecord("bou-fwd-1", "Evanilson", 9, "Forward", _fwd_attrs(83, 82, 75, 84, 81), starter_5v5=True),
                    SquadMemberRecord("bou-def-2", "J. Araujo", 2, "Defender", _def_attrs(85, 77, 75, 75)),
                    SquadMemberRecord("bou-def-3", "B. Diakite", 18, "Defender", _def_attrs(80, 82, 75, 83)),
                    SquadMemberRecord("bou-def-4", "A. Truffert", 3, "Defender", _def_attrs(82, 78, 79, 73)),
                    SquadMemberRecord("bou-mid-3", "T. Adams", 12, "Midfielder", _mid_attrs(79, 76, 78, 64, 83)),
                    SquadMemberRecord("bou-fwd-2", "J. Kluivert", 19, "Forward", _fwd_attrs(88, 84, 77, 80, 67)),
                    SquadMemberRecord("bou-fwd-3", "A. Adli", 21, "Forward", _fwd_attrs(86, 84, 79, 78, 70)),
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
                    SquadMemberRecord("liv-def-1", "V. van Dijk", 4, "Defender", _def_attrs(78, 90, 82, 91), starter_5v5=True),
                    SquadMemberRecord("liv-mid-1", "F. Wirtz", 7, "Midfielder", _mid_attrs(83, 91, 89, 84, 62), starter_5v5=True),
                    SquadMemberRecord("liv-mid-2", "D. Szoboszlai", 8, "Midfielder", _mid_attrs(84, 85, 86, 84, 72), starter_5v5=True),
                    SquadMemberRecord("liv-fwd-1", "A. Isak", 9, "Forward", _fwd_attrs(88, 88, 80, 90, 79), starter_5v5=True),
                    SquadMemberRecord("liv-def-2", "J. Frimpong", 30, "Defender", _def_attrs(93, 77, 81, 70)),
                    SquadMemberRecord("liv-def-3", "R. Araujo", 33, "Defender", _def_attrs(84, 87, 75, 88)),
                    SquadMemberRecord("liv-def-4", "M. Kerkez", 6, "Defender", _def_attrs(86, 80, 79, 77)),
                    SquadMemberRecord("liv-mid-3", "A. Mac Allister", 10, "Midfielder", _mid_attrs(75, 85, 87, 82, 80)),
                    SquadMemberRecord("liv-fwd-2", "B. Barcola", 29, "Forward", _fwd_attrs(92, 88, 80, 82, 72)),
                    SquadMemberRecord("liv-fwd-3", "H. Ekitike", 22, "Forward", _fwd_attrs(86, 84, 76, 83, 78)),
                ],
            ),
            "leeds": ClubRecord(
                club_id="leeds",
                name="Leeds United",
                short_name="LEE",
                country="England",
                stadium="Elland Road",
                primary_color="#FFFFFF",
                secondary_color="#1D428A",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("lee-gk-1", "J. Trafford", 1, "Goalkeeper", _gk_attrs(82, 81, 80), starter_5v5=True),
                    SquadMemberRecord("lee-def-1", "E. Ampadu", 4, "Defender", _def_attrs(77, 82, 80, 81), starter_5v5=True),
                    SquadMemberRecord("lee-mid-1", "A. Tanaka", 22, "Midfielder", _mid_attrs(78, 81, 83, 74, 80), starter_5v5=True),
                    SquadMemberRecord("lee-mid-2", "A. Stach", 18, "Midfielder", _mid_attrs(76, 79, 81, 76, 82), starter_5v5=True),
                    SquadMemberRecord("lee-fwd-1", "D. Calvert-Lewin", 9, "Forward", _fwd_attrs(81, 77, 72, 82, 85), starter_5v5=True),
                    SquadMemberRecord("lee-def-2", "J. Bogle", 2, "Defender", _def_attrs(83, 77, 76, 75)),
                    SquadMemberRecord("lee-def-3", "J. Rodon", 6, "Defender", _def_attrs(75, 81, 74, 84)),
                    SquadMemberRecord("lee-def-4", "N. Elvedi", 30, "Defender", _def_attrs(76, 82, 77, 82)),
                    SquadMemberRecord("lee-mid-3", "B. Aaronson", 11, "Midfielder", _mid_attrs(83, 82, 78, 73, 68)),
                    SquadMemberRecord("lee-fwd-2", "N. Okafor", 19, "Forward", _fwd_attrs(89, 84, 74, 79, 76)),
                    SquadMemberRecord("lee-fwd-3", "H. Wilson", 10, "Forward", _fwd_attrs(79, 81, 82, 81, 68)),
                ],
            ),
            "everton": ClubRecord(
                club_id="everton",
                name="Everton",
                short_name="EVE",
                country="England",
                stadium="Hill Dickinson Stadium",
                primary_color="#003399",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("eve-gk-1", "J. Pickford", 1, "Goalkeeper", _gk_attrs(85, 84, 84), starter_5v5=True),
                    SquadMemberRecord("eve-def-1", "J. Branthwaite", 4, "Defender", _def_attrs(81, 85, 77, 86), starter_5v5=True),
                    SquadMemberRecord("eve-mid-1", "K. Dewsbury-Hall", 8, "Midfielder", _mid_attrs(78, 82, 83, 77, 74), starter_5v5=True),
                    SquadMemberRecord("eve-mid-2", "J. Grealish", 10, "Midfielder", _mid_attrs(78, 88, 85, 77, 58), starter_5v5=True),
                    SquadMemberRecord("eve-fwd-1", "T. Barry", 11, "Forward", _fwd_attrs(85, 80, 72, 83, 84), starter_5v5=True),
                    SquadMemberRecord("eve-def-2", "J. Tarkowski", 6, "Defender", _def_attrs(68, 84, 72, 88)),
                    SquadMemberRecord("eve-def-3", "V. Mykolenko", 16, "Defender", _def_attrs(80, 80, 75, 76)),
                    SquadMemberRecord("eve-def-4", "A. Maitland-Niles", 2, "Defender", _def_attrs(83, 78, 77, 75)),
                    SquadMemberRecord("eve-mid-3", "C. Norgaard", 23, "Midfielder", _mid_attrs(70, 75, 79, 70, 84)),
                    SquadMemberRecord("eve-fwd-2", "B. Johnson", 22, "Forward", _fwd_attrs(90, 81, 75, 80, 72)),
                    SquadMemberRecord("eve-fwd-3", "T. Dibling", 20, "Forward", _fwd_attrs(84, 85, 78, 76, 71)),
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
                    SquadMemberRecord("che-gk-1", "E. Martinez", 1, "Goalkeeper", _gk_attrs(87, 86, 83), starter_5v5=True),
                    SquadMemberRecord("che-def-1", "L. Colwill", 6, "Defender", _def_attrs(80, 84, 82, 83), starter_5v5=True),
                    SquadMemberRecord("che-mid-1", "M. Caicedo", 25, "Midfielder", _mid_attrs(81, 82, 84, 70, 87), starter_5v5=True),
                    SquadMemberRecord("che-mid-2", "C. Palmer", 10, "Midfielder", _mid_attrs(82, 89, 89, 88, 56), starter_5v5=True),
                    SquadMemberRecord("che-fwd-1", "Joao Pedro", 9, "Forward", _fwd_attrs(86, 87, 81, 85, 78), starter_5v5=True),
                    SquadMemberRecord("che-def-2", "R. James", 24, "Defender", _def_attrs(81, 85, 85, 84)),
                    SquadMemberRecord("che-def-3", "W. Fofana", 3, "Defender", _def_attrs(82, 83, 75, 82)),
                    SquadMemberRecord("che-def-4", "J. Hato", 21, "Defender", _def_attrs(83, 82, 81, 79)),
                    SquadMemberRecord("che-mid-3", "M. Rogers", 17, "Midfielder", _mid_attrs(85, 87, 83, 83, 68)),
                    SquadMemberRecord("che-fwd-2", "P. Neto", 7, "Forward", _fwd_attrs(91, 86, 80, 79, 69)),
                    SquadMemberRecord("che-fwd-3", "Estevao", 41, "Forward", _fwd_attrs(88, 90, 81, 80, 64)),
                ],
            ),
            "tottenham": ClubRecord(
                club_id="tottenham",
                name="Tottenham Hotspur",
                short_name="TOT",
                country="England",
                stadium="Tottenham Hotspur Stadium",
                primary_color="#FFFFFF",
                secondary_color="#132257",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("tot-gk-1", "A. Kinsky", 31, "Goalkeeper", _gk_attrs(83, 82, 83), starter_5v5=True),
                    SquadMemberRecord("tot-def-1", "M. van de Ven", 37, "Defender", _def_attrs(93, 85, 78, 84), starter_5v5=True),
                    SquadMemberRecord("tot-mid-1", "S. Tonali", 16, "Midfielder", _mid_attrs(81, 83, 85, 77, 84), starter_5v5=True),
                    SquadMemberRecord("tot-mid-2", "X. Simons", 7, "Midfielder", _mid_attrs(85, 89, 85, 81, 61), starter_5v5=True),
                    SquadMemberRecord("tot-fwd-1", "O. Marmoush", 22, "Forward", _fwd_attrs(90, 86, 79, 85, 76), starter_5v5=True),
                    SquadMemberRecord("tot-def-2", "P. Porro", 23, "Defender", _def_attrs(84, 79, 84, 75)),
                    SquadMemberRecord("tot-def-3", "M. Senesi", 5, "Defender", _def_attrs(76, 83, 78, 83)),
                    SquadMemberRecord("tot-def-4", "D. Udogie", 13, "Defender", _def_attrs(86, 81, 78, 80)),
                    SquadMemberRecord("tot-mid-3", "J. Maddison", 10, "Midfielder", _mid_attrs(76, 86, 87, 81, 60)),
                    SquadMemberRecord("tot-fwd-2", "M. Kudus", 20, "Forward", _fwd_attrs(89, 89, 78, 81, 80)),
                    SquadMemberRecord("tot-fwd-3", "D. Solanke", 19, "Forward", _fwd_attrs(82, 80, 74, 83, 84)),
                ],
            ),
            "brentford": ClubRecord(
                club_id="brentford",
                name="Brentford",
                short_name="BRE",
                country="England",
                stadium="Brentford Community Stadium",
                primary_color="#E30613",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("bre-gk-1", "C. Kelleher", 1, "Goalkeeper", _gk_attrs(82, 82, 80), starter_5v5=True),
                    SquadMemberRecord("bre-def-1", "N. Collins", 22, "Defender", _def_attrs(77, 83, 76, 86), starter_5v5=True),
                    SquadMemberRecord("bre-mid-1", "M. Damsgaard", 24, "Midfielder", _mid_attrs(79, 84, 84, 75, 68), starter_5v5=True),
                    SquadMemberRecord("bre-mid-2", "V. Janelt", 27, "Midfielder", _mid_attrs(76, 78, 80, 72, 82), starter_5v5=True),
                    SquadMemberRecord("bre-fwd-1", "I. Thiago", 9, "Forward", _fwd_attrs(83, 79, 72, 82, 86), starter_5v5=True),
                    SquadMemberRecord("bre-def-2", "M. Kayode", 33, "Defender", _def_attrs(85, 79, 76, 79)),
                    SquadMemberRecord("bre-def-3", "K. Ajer", 20, "Defender", _def_attrs(76, 81, 76, 85)),
                    SquadMemberRecord("bre-def-4", "R. Henry", 3, "Defender", _def_attrs(86, 79, 76, 74)),
                    SquadMemberRecord("bre-mid-3", "F. Carvalho", 14, "Midfielder", _mid_attrs(81, 84, 80, 78, 60)),
                    SquadMemberRecord("bre-fwd-2", "K. Schade", 7, "Forward", _fwd_attrs(92, 81, 73, 79, 77)),
                    SquadMemberRecord("bre-fwd-3", "D. Ouattara", 11, "Forward", _fwd_attrs(89, 82, 74, 77, 75)),
                ],
            ),
            "aston-villa": ClubRecord(
                club_id="aston-villa",
                name="Aston Villa",
                short_name="AVL",
                country="England",
                stadium="Villa Park",
                primary_color="#670E36",
                secondary_color="#95BFE5",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("avl-gk-1", "Z. Suzuki", 1, "Goalkeeper", _gk_attrs(82, 80, 81), starter_5v5=True),
                    SquadMemberRecord("avl-def-1", "P. Torres", 14, "Defender", _def_attrs(76, 84, 85, 81), starter_5v5=True),
                    SquadMemberRecord("avl-mid-1", "A. Onana", 24, "Midfielder", _mid_attrs(81, 79, 80, 72, 85), starter_5v5=True),
                    SquadMemberRecord("avl-mid-2", "J. McGinn", 7, "Midfielder", _mid_attrs(76, 82, 83, 78, 80), starter_5v5=True),
                    SquadMemberRecord("avl-fwd-1", "N. Jackson", 11, "Forward", _fwd_attrs(89, 83, 76, 83, 80), starter_5v5=True),
                    SquadMemberRecord("avl-def-2", "M. Cash", 2, "Defender", _def_attrs(83, 79, 77, 77)),
                    SquadMemberRecord("avl-def-3", "I. Maatsen", 22, "Defender", _def_attrs(88, 77, 79, 70)),
                    SquadMemberRecord("avl-def-4", "T. Mings", 5, "Defender", _def_attrs(75, 82, 74, 87)),
                    SquadMemberRecord("avl-mid-3", "B. Kamara", 8, "Midfielder", _mid_attrs(75, 80, 82, 68, 85)),
                    SquadMemberRecord("avl-fwd-2", "A. Garnacho", 17, "Forward", _fwd_attrs(90, 85, 76, 80, 68)),
                    SquadMemberRecord("avl-fwd-3", "T. Abraham", 18, "Forward", _fwd_attrs(81, 78, 72, 81, 83)),
                ],
            ),
            "newcastle": ClubRecord(
                club_id="newcastle",
                name="Newcastle United",
                short_name="NEW",
                country="England",
                stadium="St James' Park",
                primary_color="#241F20",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("new-gk-1", "N. Pope", 1, "Goalkeeper", _gk_attrs(84, 84, 73), starter_5v5=True),
                    SquadMemberRecord("new-def-1", "S. Botman", 4, "Defender", _def_attrs(77, 86, 79, 87), starter_5v5=True),
                    SquadMemberRecord("new-mid-1", "Joelinton", 7, "Midfielder", _mid_attrs(80, 82, 80, 77, 85), starter_5v5=True),
                    SquadMemberRecord("new-mid-2", "N. Gonzalez", 6, "Midfielder", _mid_attrs(77, 82, 84, 75, 81), starter_5v5=True),
                    SquadMemberRecord("new-fwd-1", "Y. Wissa", 9, "Forward", _fwd_attrs(86, 83, 76, 83, 78), starter_5v5=True),
                    SquadMemberRecord("new-def-2", "T. Livramento", 2, "Defender", _def_attrs(87, 81, 78, 76)),
                    SquadMemberRecord("new-def-3", "F. Schar", 5, "Defender", _def_attrs(72, 83, 82, 83)),
                    SquadMemberRecord("new-def-4", "L. Hall", 3, "Defender", _def_attrs(82, 79, 81, 75)),
                    SquadMemberRecord("new-mid-3", "J. Ramsey", 41, "Midfielder", _mid_attrs(82, 83, 80, 77, 73)),
                    SquadMemberRecord("new-fwd-2", "A. Elanga", 19, "Forward", _fwd_attrs(93, 83, 76, 78, 72)),
                    SquadMemberRecord("new-fwd-3", "H. Barnes", 11, "Forward", _fwd_attrs(85, 83, 77, 81, 72)),
                ],
            ),
            "coventry": ClubRecord(
                club_id="coventry",
                name="Coventry City",
                short_name="COV",
                country="England",
                stadium="Coventry Building Society Arena",
                primary_color="#0593D0",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("cov-gk-1", "C. Rushworth", 19, "Goalkeeper", _gk_attrs(78, 78, 76), starter_5v5=True),
                    SquadMemberRecord("cov-def-1", "M. van Ewijk", 27, "Defender", _def_attrs(88, 77, 75, 75), starter_5v5=True),
                    SquadMemberRecord("cov-mid-1", "J. Rudoni", 5, "Midfielder", _mid_attrs(78, 80, 81, 76, 72), starter_5v5=True),
                    SquadMemberRecord("cov-mid-2", "G. Hamer", 38, "Midfielder", _mid_attrs(76, 81, 84, 80, 76), starter_5v5=True),
                    SquadMemberRecord("cov-fwd-1", "H. Wright", 11, "Forward", _fwd_attrs(86, 79, 73, 80, 83), starter_5v5=True),
                    SquadMemberRecord("cov-def-2", "B. Thomas", 4, "Defender", _def_attrs(74, 78, 72, 82)),
                    SquadMemberRecord("cov-def-3", "E. Pinnock", 2, "Defender", _def_attrs(71, 81, 73, 86)),
                    SquadMemberRecord("cov-def-4", "J. Dasilva", 3, "Defender", _def_attrs(82, 75, 76, 68)),
                    SquadMemberRecord("cov-mid-3", "M. Grimes", 6, "Midfielder", _mid_attrs(72, 78, 82, 70, 77)),
                    SquadMemberRecord("cov-fwd-2", "T. Sakamoto", 7, "Forward", _fwd_attrs(83, 83, 77, 76, 66)),
                    SquadMemberRecord("cov-fwd-3", "T. Awoniyi", 14, "Forward", _fwd_attrs(83, 76, 69, 79, 86)),
                ],
            ),
            "hull": ClubRecord(
                club_id="hull",
                name="Hull City",
                short_name="HUL",
                country="England",
                stadium="MKM Stadium",
                primary_color="#F5A12D",
                secondary_color="#111827",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("hul-gk-1", "K. Tzolakis", 19, "Goalkeeper", _gk_attrs(81, 80, 76), starter_5v5=True),
                    SquadMemberRecord("hul-def-1", "C. Hughes", 4, "Defender", _def_attrs(75, 79, 75, 81), starter_5v5=True),
                    SquadMemberRecord("hul-mid-1", "H. Morita", 5, "Midfielder", _mid_attrs(76, 81, 82, 72, 81), starter_5v5=True),
                    SquadMemberRecord("hul-mid-2", "T. Iroegbunam", 42, "Midfielder", _mid_attrs(79, 78, 77, 71, 79), starter_5v5=True),
                    SquadMemberRecord("hul-fwd-1", "M. Belloumi", 10, "Forward", _fwd_attrs(87, 84, 77, 78, 67), starter_5v5=True),
                    SquadMemberRecord("hul-def-2", "L. Coyle", 2, "Defender", _def_attrs(78, 76, 74, 75)),
                    SquadMemberRecord("hul-def-3", "J. Egan", 15, "Defender", _def_attrs(70, 79, 73, 81)),
                    SquadMemberRecord("hul-def-4", "R. Giles", 3, "Defender", _def_attrs(82, 74, 79, 72)),
                    SquadMemberRecord("hul-mid-3", "R. Slater", 27, "Midfielder", _mid_attrs(76, 76, 77, 71, 76)),
                    SquadMemberRecord("hul-fwd-2", "O. McBurnie", 9, "Forward", _fwd_attrs(72, 74, 71, 78, 84)),
                    SquadMemberRecord("hul-fwd-3", "M. Cho", 50, "Forward", _fwd_attrs(88, 82, 73, 76, 71)),
                ],
            ),
            "fulham": ClubRecord(
                club_id="fulham",
                name="Fulham",
                short_name="FUL",
                country="England",
                stadium="Craven Cottage",
                primary_color="#FFFFFF",
                secondary_color="#111827",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("ful-gk-1", "B. Leno", 1, "Goalkeeper", _gk_attrs(84, 84, 78), starter_5v5=True),
                    SquadMemberRecord("ful-def-1", "A. Robinson", 33, "Defender", _def_attrs(90, 82, 79, 78), starter_5v5=True),
                    SquadMemberRecord("ful-mid-1", "S. Berge", 16, "Midfielder", _mid_attrs(76, 80, 81, 71, 82), starter_5v5=True),
                    SquadMemberRecord("ful-mid-2", "E. Smith Rowe", 32, "Midfielder", _mid_attrs(81, 85, 82, 78, 63), starter_5v5=True),
                    SquadMemberRecord("ful-fwd-1", "R. Muniz", 9, "Forward", _fwd_attrs(81, 79, 72, 82, 84), starter_5v5=True),
                    SquadMemberRecord("ful-def-2", "K. Tete", 2, "Defender", _def_attrs(80, 81, 75, 78)),
                    SquadMemberRecord("ful-def-3", "J. Andersen", 5, "Defender", _def_attrs(70, 83, 82, 86)),
                    SquadMemberRecord("ful-def-4", "C. Bassey", 3, "Defender", _def_attrs(81, 81, 75, 86)),
                    SquadMemberRecord("ful-mid-3", "H. Larsson", 15, "Midfielder", _mid_attrs(79, 82, 82, 74, 77)),
                    SquadMemberRecord("ful-fwd-2", "O. Bobb", 14, "Forward", _fwd_attrs(85, 88, 81, 78, 66)),
                    SquadMemberRecord("ful-fwd-3", "A. Iwobi", 17, "Forward", _fwd_attrs(83, 84, 80, 75, 74)),
                ],
            ),
            "ipswich": ClubRecord(
                club_id="ipswich",
                name="Ipswich Town",
                short_name="IPS",
                country="England",
                stadium="Portman Road",
                primary_color="#3A64A3",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("ips-gk-1", "A. Palmer", 1, "Goalkeeper", _gk_attrs(78, 78, 74), starter_5v5=True),
                    SquadMemberRecord("ips-def-1", "L. Davis", 3, "Defender", _def_attrs(85, 76, 82, 73), starter_5v5=True),
                    SquadMemberRecord("ips-mid-1", "Florentino", 12, "Midfielder", _mid_attrs(76, 78, 80, 66, 84), starter_5v5=True),
                    SquadMemberRecord("ips-mid-2", "E. Palacios", 25, "Midfielder", _mid_attrs(78, 83, 84, 76, 81), starter_5v5=True),
                    SquadMemberRecord("ips-fwd-1", "J. Enciso", 10, "Forward", _fwd_attrs(86, 85, 78, 80, 68), starter_5v5=True),
                    SquadMemberRecord("ips-def-2", "D. O'Shea", 26, "Defender", _def_attrs(76, 79, 75, 82)),
                    SquadMemberRecord("ips-def-3", "J. Greaves", 24, "Defender", _def_attrs(75, 79, 74, 83)),
                    SquadMemberRecord("ips-def-4", "D. Furlong", 2, "Defender", _def_attrs(78, 77, 73, 79)),
                    SquadMemberRecord("ips-mid-3", "S. Lukic", 23, "Midfielder", _mid_attrs(74, 79, 81, 72, 78)),
                    SquadMemberRecord("ips-fwd-2", "A. Fatawu", 7, "Forward", _fwd_attrs(91, 84, 75, 77, 69)),
                    SquadMemberRecord("ips-fwd-3", "Z. Flemming", 9, "Forward", _fwd_attrs(77, 78, 75, 79, 81)),
                ],
            ),
            "nottm-forest": ClubRecord(
                club_id="nottm-forest",
                name="Nottingham Forest",
                short_name="NFO",
                country="England",
                stadium="City Ground",
                primary_color="#DD0000",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("nfo-gk-1", "M. Sels", 26, "Goalkeeper", _gk_attrs(83, 83, 76), starter_5v5=True),
                    SquadMemberRecord("nfo-def-1", "Murillo", 5, "Defender", _def_attrs(82, 84, 79, 85), starter_5v5=True),
                    SquadMemberRecord("nfo-mid-1", "M. Gibbs-White", 10, "Midfielder", _mid_attrs(83, 86, 85, 79, 68), starter_5v5=True),
                    SquadMemberRecord("nfo-mid-2", "I. Sangare", 6, "Midfielder", _mid_attrs(74, 77, 79, 70, 84), starter_5v5=True),
                    SquadMemberRecord("nfo-fwd-1", "L. Delap", 19, "Forward", _fwd_attrs(85, 80, 73, 82, 84), starter_5v5=True),
                    SquadMemberRecord("nfo-def-2", "O. Diomande", 2, "Defender", _def_attrs(83, 83, 78, 86)),
                    SquadMemberRecord("nfo-def-3", "N. Milenkovic", 31, "Defender", _def_attrs(72, 84, 72, 88)),
                    SquadMemberRecord("nfo-def-4", "O. Aina", 34, "Defender", _def_attrs(86, 80, 77, 78)),
                    SquadMemberRecord("nfo-mid-3", "J. McAtee", 24, "Midfielder", _mid_attrs(80, 84, 81, 77, 62)),
                    SquadMemberRecord("nfo-fwd-2", "C. Hudson-Odoi", 7, "Forward", _fwd_attrs(88, 85, 78, 78, 69)),
                    SquadMemberRecord("nfo-fwd-3", "I. Jesus", 11, "Forward", _fwd_attrs(83, 81, 74, 80, 82)),
                ],
            ),
            "sunderland": ClubRecord(
                club_id="sunderland",
                name="Sunderland",
                short_name="SUN",
                country="England",
                stadium="Stadium of Light",
                primary_color="#EB172B",
                secondary_color="#FFFFFF",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("sun-gk-1", "R. Roefs", 22, "Goalkeeper", _gk_attrs(80, 79, 78), starter_5v5=True),
                    SquadMemberRecord("sun-def-1", "K. Danso", 4, "Defender", _def_attrs(80, 82, 74, 87), starter_5v5=True),
                    SquadMemberRecord("sun-mid-1", "G. Xhaka", 34, "Midfielder", _mid_attrs(68, 79, 87, 79, 82), starter_5v5=True),
                    SquadMemberRecord("sun-mid-2", "E. Le Fee", 28, "Midfielder", _mid_attrs(80, 85, 83, 76, 68), starter_5v5=True),
                    SquadMemberRecord("sun-fwd-1", "B. Brobbey", 9, "Forward", _fwd_attrs(85, 79, 72, 81, 89), starter_5v5=True),
                    SquadMemberRecord("sun-def-2", "N. Mukiele", 20, "Defender", _def_attrs(82, 80, 76, 83)),
                    SquadMemberRecord("sun-def-3", "D. Ballard", 5, "Defender", _def_attrs(75, 80, 72, 84)),
                    SquadMemberRecord("sun-def-4", "R. Mandava", 17, "Defender", _def_attrs(83, 81, 75, 80)),
                    SquadMemberRecord("sun-mid-3", "C. Rigg", 11, "Midfielder", _mid_attrs(79, 81, 80, 74, 74)),
                    SquadMemberRecord("sun-fwd-2", "M. Fofana", 39, "Forward", _fwd_attrs(91, 86, 76, 77, 65)),
                    SquadMemberRecord("sun-fwd-3", "W. Isidor", 18, "Forward", _fwd_attrs(87, 80, 71, 78, 77)),
                ],
            ),
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
                    SquadMemberRecord("mun-gk-1", "S. Lammens", 1, "Goalkeeper", _gk_attrs(82, 81, 80), starter_5v5=True),
                    SquadMemberRecord("mun-def-1", "L. Martinez", 6, "Defender", _def_attrs(77, 86, 83, 82), starter_5v5=True),
                    SquadMemberRecord("mun-mid-1", "K. Mainoo", 37, "Midfielder", _mid_attrs(80, 86, 84, 75, 78), starter_5v5=True),
                    SquadMemberRecord("mun-mid-2", "B. Fernandes", 8, "Midfielder", _mid_attrs(76, 85, 90, 86, 70), starter_5v5=True),
                    SquadMemberRecord("mun-fwd-1", "M. Cunha", 10, "Forward", _fwd_attrs(86, 87, 82, 84, 79), starter_5v5=True),
                    SquadMemberRecord("mun-def-2", "D. Dalot", 2, "Defender", _def_attrs(85, 80, 79, 77)),
                    SquadMemberRecord("mun-def-3", "M. de Ligt", 4, "Defender", _def_attrs(74, 85, 75, 88)),
                    SquadMemberRecord("mun-def-4", "L. Yoro", 15, "Defender", _def_attrs(83, 84, 78, 82)),
                    SquadMemberRecord("mun-mid-3", "Y. Tielemans", 18, "Midfielder", _mid_attrs(71, 82, 86, 80, 75)),
                    SquadMemberRecord("mun-fwd-2", "B. Mbeumo", 19, "Forward", _fwd_attrs(89, 85, 80, 85, 77)),
                    SquadMemberRecord("mun-fwd-3", "B. Sesko", 30, "Forward", _fwd_attrs(88, 80, 73, 84, 86)),
                ],
            ),
            "crystal-palace": ClubRecord(
                club_id="crystal-palace",
                name="Crystal Palace",
                short_name="CRY",
                country="England",
                stadium="Selhurst Park",
                primary_color="#1B458F",
                secondary_color="#C4122E",
                formation_5v5="1-2-1",
                formation_11v11="4-3-3",
                squad=[
                    SquadMemberRecord("cry-gk-1", "D. Henderson", 1, "Goalkeeper", _gk_attrs(82, 82, 76), starter_5v5=True),
                    SquadMemberRecord("cry-def-1", "T. Mitchell", 3, "Defender", _def_attrs(83, 81, 75, 76), starter_5v5=True),
                    SquadMemberRecord("cry-mid-1", "A. Wharton", 20, "Midfielder", _mid_attrs(75, 83, 86, 71, 82), starter_5v5=True),
                    SquadMemberRecord("cry-mid-2", "D. Kamada", 18, "Midfielder", _mid_attrs(78, 83, 82, 77, 68), starter_5v5=True),
                    SquadMemberRecord("cry-fwd-1", "J. Mateta", 14, "Forward", _fwd_attrs(82, 79, 73, 83, 87), starter_5v5=True),
                    SquadMemberRecord("cry-def-2", "C. Richards", 26, "Defender", _def_attrs(79, 80, 74, 82)),
                    SquadMemberRecord("cry-def-3", "A. Disasi", 5, "Defender", _def_attrs(74, 81, 74, 86)),
                    SquadMemberRecord("cry-def-4", "O. Mingueza", 30, "Defender", _def_attrs(81, 79, 81, 76)),
                    SquadMemberRecord("cry-mid-3", "Y. Pino", 10, "Midfielder", _mid_attrs(84, 84, 79, 77, 62)),
                    SquadMemberRecord("cry-fwd-2", "I. Sarr", 7, "Forward", _fwd_attrs(92, 82, 75, 78, 75)),
                    SquadMemberRecord("cry-fwd-3", "J. Strand Larsen", 22, "Forward", _fwd_attrs(80, 77, 72, 81, 86)),
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

    def replay_transfer_events(self, events: list[dict[str, Any]]) -> int:
        """Replay persisted `imported_squad_membership_changed` ledger events onto this catalogue."""
        applied = 0
        for ev in events:
            if ev.get("type") != "imported_squad_membership_changed":
                continue
            p = ev.get("payload") or {}
            fid = p.get("footballer_id")
            from_cid = p.get("from_club_id")
            to_cid = p.get("to_club_id")
            if not fid or not from_cid or not to_cid:
                continue
            if from_cid not in self.clubs or to_cid not in self.clubs:
                continue
            already_at_dest = any(
                m.footballer_id == fid and m.effective_to is None
                for m in self.clubs[to_cid].squad
            )
            still_at_src = any(
                m.footballer_id == fid and m.effective_to is None
                for m in self.clubs[from_cid].squad
            )
            if already_at_dest and not still_at_src:
                continue
            if still_at_src:
                self.record_transfer(
                    footballer_id=str(fid),
                    from_club_id=str(from_cid),
                    to_club_id=str(to_cid),
                    occurred_at=str(p.get("effective_from") or ev.get("occurred_at") or "2026-08-01T00:00:00Z"),
                    new_shirt=int(p.get("new_shirt", 99)),
                )
                applied += 1
        return applied

    def for_world(self, store: Any, world_id: str) -> ClubCatalogue:
        """Build a world-scoped catalogue from default squads + transfer events of the whole season lineage.

        A rolled-over child world inherits its parent's squads, so transfers are replayed
        root-ancestor first, then each descendant down to `world_id`.
        """
        lineage: list[str] = []
        seen: set[str] = set()
        current: str | None = world_id
        while current and current not in seen:
            seen.add(current)
            lineage.append(current)
            try:
                current = store.get_world(current).get("parent_world_id")
            except KeyError:
                break
        cat = ClubCatalogue()
        ordered = list(reversed(lineage))
        for idx, wid in enumerate(ordered):
            events = store.list_events(wid)
            if idx + 1 < len(ordered):
                child_wid = ordered[idx + 1]
                bounded: list[dict[str, Any]] = []
                for ev in events:
                    bounded.append(ev)
                    if (
                        ev.get("type") == "season_rolled_over"
                        and (ev.get("payload") or {}).get("next_world_id") == child_wid
                    ):
                        break
                events = bounded
            cat.replay_transfer_events(events)
        return cat

    def resolve_saved_5v5_selection(
        self,
        club_id: str,
        saved: dict[str, Any] | None,
    ) -> tuple[list[str] | None, str | None]:
        """Resolve a persisted 5v5 lineup against the club's *current* active squad.

        Returns `(starter_ids, formation)`. If a saved starter has since transferred away
        (or the saved lineup is otherwise no longer valid), starters fall back to `None`
        (club defaults) — matching what `get_club_roster` displays — while a still-valid
        saved formation is kept. Explicit per-match overrides are NOT routed through here
        and continue to be strictly validated.
        """
        if not saved:
            return (None, None)
        fmt = saved.get("formation_5v5")
        fmt = fmt if fmt in SUPPORTED_5V5_FORMATIONS else None
        ids = saved.get("starter_ids")
        if not ids:
            return (None, fmt)
        try:
            self.validate_5v5_selection(club_id, list(ids), fmt)
        except (ValueError, KeyError):
            return (None, fmt)
        return (list(ids), fmt)

    def validate_5v5_selection(
        self,
        club_id: str,
        starter_ids: list[str],
        formation_5v5: str | None = None,
    ) -> tuple[list[SquadMemberRecord], str]:
        """Validate a 5v5 selection (1 GK + 4 outfielders) from the club's active 11-player squad."""
        if club_id not in self.clubs:
            raise KeyError(f"Unknown club '{club_id}'")
        club = self.clubs[club_id]
        formation = formation_5v5 or club.formation_5v5
        if formation not in SUPPORTED_5V5_FORMATIONS:
            raise ValueError(
                f"Unsupported 5v5 formation '{formation}'; choose one of {list(SUPPORTED_5V5_FORMATIONS)}"
            )

        if len(starter_ids) != 5 or len(set(starter_ids)) != 5:
            raise ValueError(
                f"5v5 lineup for '{club_id}' must select exactly 5 unique players from the official squad (got {len(starter_ids)})"
            )

        active_by_id = {
            m.footballer_id: m for m in club.squad if m.effective_to is None
        }
        chosen: list[SquadMemberRecord] = []
        for fid in starter_ids:
            member = active_by_id.get(fid)
            if member is None:
                raise ValueError(
                    f"Footballer '{fid}' is not in active official squad of '{club_id}'"
                )
            chosen.append(member)

        gks = [m for m in chosen if m.role == "Goalkeeper"]
        if len(gks) != 1:
            raise ValueError(
                f"5v5 lineup for '{club_id}' must include exactly 1 Goalkeeper + 4 outfield players (got {len(gks)} Goalkeepers)"
            )

        outfield = [m for m in chosen if m.role != "Goalkeeper"]
        order_index = {fid: i for i, fid in enumerate(starter_ids)}
        outfield.sort(key=lambda m: (_ROLE_ORDER.get(m.role, 9), order_index[m.footballer_id]))
        return ([gks[0], *outfield], formation)

    def default_5v5_starter_ids(self, club_id: str) -> list[str]:
        club = self.clubs[club_id]
        active = [m for m in club.squad if m.effective_to is None]
        gks = [m for m in active if m.role == "Goalkeeper"]
        if not gks:
            raise ValueError(f"Club '{club_id}' has no active Goalkeeper")
        preferred = [m for m in active if m.role != "Goalkeeper" and m.starter_5v5]
        if len(preferred) < 4:
            extras = [m for m in active if m.role != "Goalkeeper" and m not in preferred]
            preferred.extend(extras[: 4 - len(preferred)])
        return [gks[0].footballer_id, *[m.footballer_id for m in preferred[:4]]]

    def get_club_roster(
        self,
        club_id: str,
        selected_starter_ids: list[str] | None = None,
        formation_5v5: str | None = None,
    ) -> dict[str, Any]:
        """Return full 11-player official squad + active 5v5 selection and formation."""
        if club_id not in self.clubs:
            raise KeyError(f"Unknown club '{club_id}'")
        club = self.clubs[club_id]
        active = [m for m in club.squad if m.effective_to is None]

        effective_ids = self.default_5v5_starter_ids(club_id)
        effective_formation = club.formation_5v5
        custom_applied = False
        if selected_starter_ids:
            try:
                validated, fmt = self.validate_5v5_selection(
                    club_id, selected_starter_ids, formation_5v5
                )
                effective_ids = [m.footballer_id for m in validated]
                effective_formation = fmt
                custom_applied = True
            except ValueError:
                # If a player transferred away since saving custom lineup, gracefully fall back to valid defaults
                effective_ids = self.default_5v5_starter_ids(club_id)
                if formation_5v5 in SUPPORTED_5V5_FORMATIONS:
                    effective_formation = formation_5v5
        elif formation_5v5 in SUPPORTED_5V5_FORMATIONS:
            effective_formation = str(formation_5v5)

        selected_set = set(effective_ids)
        return {
            "club_id": club.club_id,
            "name": club.name,
            "short_name": club.short_name,
            "country": club.country,
            "stadium": club.stadium,
            "primary_color": club.primary_color,
            "secondary_color": club.secondary_color,
            "formation_5v5": effective_formation,
            "formation_11v11": club.formation_11v11,
            "supported_formations_5v5": list(SUPPORTED_5V5_FORMATIONS),
            "custom_lineup_active": custom_applied,
            "starter_ids_5v5": effective_ids,
            "squad": [
                {
                    "footballer_id": m.footballer_id,
                    "name": m.name,
                    "shirt_number": m.shirt_number,
                    "role": m.role,
                    "overall": _member_overall(m),
                    "default_starter_5v5": m.starter_5v5,
                    "selected_5v5": m.footballer_id in selected_set,
                    "attributes": copy.deepcopy(m.attributes),
                }
                for m in active
            ],
        }

    def all_rosters(
        self, world_lineups: dict[str, dict[str, Any]] | None = None
    ) -> dict[str, dict[str, Any]]:
        lineups = world_lineups or {}
        out: dict[str, dict[str, Any]] = {}
        for cid in self.clubs:
            saved = lineups.get(cid)
            out[cid] = self.get_club_roster(
                cid,
                selected_starter_ids=saved["starter_ids"] if saved else None,
                formation_5v5=saved["formation_5v5"] if saved else None,
            )
        return out

    def build_team_snapshot(
        self,
        club_id: str,
        format_name: str = "5v5",
        starter_ids: list[str] | None = None,
        formation_override: str | None = None,
    ) -> dict[str, Any]:
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
            if starter_ids is not None:
                selected, formation = self.validate_5v5_selection(
                    club_id, starter_ids, formation_override
                )
            else:
                default_ids = self.default_5v5_starter_ids(club_id)
                selected, formation = self.validate_5v5_selection(
                    club_id, default_ids, formation_override
                )
            rest = [m for m in active if m not in selected]

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

    def club_strength_ratings(
        self,
        club_id: str,
        format_name: str = "5v5",
        starter_ids: list[str] | None = None,
    ) -> tuple[float, float]:
        """Return `(attack_rating, defense_rating)` in `[0, 100]` from active starters."""
        snap = self.build_team_snapshot(
            club_id, format_name=format_name, starter_ids=starter_ids
        )
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
        home_starters: list[str] | None = None,
        away_starters: list[str] | None = None,
        home_formation: str | None = None,
        away_formation: str | None = None,
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
            "home": self.build_team_snapshot(
                home_club_id,
                format_name=format_name,
                starter_ids=home_starters,
                formation_override=home_formation,
            ),
            "away": self.build_team_snapshot(
                away_club_id,
                format_name=format_name,
                starter_ids=away_starters,
                formation_override=away_formation,
            ),
        }
