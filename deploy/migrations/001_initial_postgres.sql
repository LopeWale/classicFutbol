-- Parallel Season FC — PostgreSQL 16 Append-Only Ledger & Domain Schema (Phase 4 Production Target)
-- Mirrors `host/psfc_host/ledger/store.py` with PostgreSQL triggers forbidding UPDATE/DELETE on `world_event`.

BEGIN;

CREATE TABLE IF NOT EXISTS world (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    competition_name TEXT NOT NULL,
    season_label TEXT NOT NULL,
    season_timezone TEXT NOT NULL,
    default_match_format TEXT NOT NULL DEFAULT '5v5',
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed')),
    parent_world_id TEXT NULL REFERENCES world(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS competition (
    id TEXT PRIMARY KEY,
    world_id TEXT NOT NULL REFERENCES world(id),
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('league', 'knockout')),
    format_name TEXT NOT NULL DEFAULT '5v5',
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed')),
    real_final_pair JSONB NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS participant (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS device (
    id TEXT PRIMARY KEY,
    participant_id TEXT NOT NULL REFERENCES participant(id),
    label TEXT NOT NULL,
    secret_hash TEXT NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ NULL
);

CREATE TABLE IF NOT EXISTS club_assignment (
    id TEXT PRIMARY KEY,
    world_id TEXT NOT NULL REFERENCES world(id),
    club_id TEXT NOT NULL,
    participant_id TEXT NOT NULL REFERENCES participant(id),
    effective_from_round INTEGER NOT NULL,
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_active_club_assignment
    ON club_assignment (world_id, club_id)
    WHERE effective_to IS NULL;

CREATE TABLE IF NOT EXISTS fixture (
    id TEXT PRIMARY KEY,
    world_id TEXT NOT NULL REFERENCES world(id),
    competition_id TEXT NULL REFERENCES competition(id),
    stage TEXT NOT NULL DEFAULT 'league',
    round INTEGER NOT NULL,
    home_club_id TEXT NOT NULL,
    away_club_id TEXT NOT NULL,
    kickoff_utc TIMESTAMPTZ NOT NULL,
    window_open_utc TIMESTAMPTZ NOT NULL,
    window_close_utc TIMESTAMPTZ NOT NULL,
    result_source TEXT NOT NULL CHECK (
        result_source IN ('played_vs_friend', 'played_vs_cpu', 'imported', 'generated', 'admin')
    ),
    state TEXT NOT NULL CHECK (
        state IN ('scheduled', 'open', 'in_session', 'played', 'accepted', 'void', 'postponed', 'awaiting_admin')
    ),
    extra_time_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    real_home_goals INTEGER NULL,
    real_away_goals INTEGER NULL
);

CREATE TABLE IF NOT EXISTS world_event (
    seq BIGSERIAL PRIMARY KEY,
    world_id TEXT NOT NULL REFERENCES world(id),
    type TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    payload JSONB NOT NULL,
    caused_by TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    supersedes_seq BIGINT NULL REFERENCES world_event(seq)
);

CREATE OR REPLACE FUNCTION forbid_world_event_mutation()
RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'world_event ledger is strictly append-only; emit a superseding event instead';
END;
$$;

DROP TRIGGER IF EXISTS trg_world_event_immutable ON world_event;
CREATE TRIGGER trg_world_event_immutable
BEFORE UPDATE OR DELETE ON world_event
FOR EACH ROW EXECUTE FUNCTION forbid_world_event_mutation();

COMMIT;
