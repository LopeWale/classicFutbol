# ADR-0001: Simulation language and host stack

Status: proposed · Date: 2026-10-16 · Supersedes the C#/.NET choice in architecture v0.1

## Context

The requirements demand that an official result comes from the validated match session rather than from a score a client reports. The cheapest way to validate is for the host to replay the recorded input log with the same deterministic simulation the clients ran. Architecture v0.1 satisfied this by writing the simulation in C# (Godot's second scripting language) and the host in ASP.NET Core so the two could share one library.

That coupling was the only reason for the host stack choice, and it has costs: it forces a runtime the maintainers do not want to live in, Godot's C# builds do not export to the web, and it couples every host service to the client language even though only the replay step needs the simulation.

Online load is not a factor at this scale. Twenty participants and at most four concurrent matches produce a few hundred small packets per second and a handful of API requests per minute. Python, Go, Rust and .NET are all far beyond adequate; the decision should be made on fit and maintainability.

## Decision

1. **The simulation core is a Rust crate** (`psfc-sim`): fixed-point arithmetic, no floating point in simulation state, single-threaded, `#![forbid(unsafe_code)]`. It compiles to x86-64, aarch64 and wasm32.
2. **The client uses Godot 4 with the simulation as a GDExtension** (`psfc-godot`, via gdext). GDScript is used only for presentation glue. C# is not used.
3. **Replay verification is a standalone binary** (`psfc-replay`) built from the same crate. The host invokes it as a subprocess. This decouples the host language from the simulation language.
4. **Host services are Python 3.12**: FastAPI, SQLAlchemy 2 async with asyncpg, Pydantic v2, Alembic, httpx; the UDP relay uses asyncio datagram transports. One package, one Compose service per role.

## Alternatives considered

| Option | Why not |
|--------|---------|
| Keep C# + ASP.NET Core | Works, but ties the host to .NET for no reason once the verifier is a binary; blocks Godot web export; not a stack the maintainers want |
| Go for the host | Strong candidate: single static binary, excellent concurrency, easy cross-compile to arm64. Chosen against because the host is dominated by rules, data wrangling and admin tooling where Python iterates faster, and because the one latency-sensitive component (the relay) is tiny. **Remains the swap-in if a single compiled binary is preferred.** |
| Rust for the host too | One language everywhere and the best performance, but slow iteration on CRUD, competition rules and provider adapters, which is most of the host code |
| TypeScript/Node for the host | Fine technically; adds a fourth language with no particular advantage here |
| Cython | Solves a performance problem the host does not have; the only hot path is already in Rust |
| C++ GDExtension for the simulation | Same performance as Rust, weaker safety, no first-class wasm32 story |
| GDScript simulation | Too slow for 60 Hz with eight-frame rollback re-simulation and offers no control over numeric determinism |

## Consequences

- Three languages, each confined to one role: Rust for deterministic and real-time code, Python for records and rules, GDScript for presentation. No business logic lives in the engine.
- The simulation being C#-free and float-free keeps a browser build possible later (ADR-0002).
- gdext tracks Godot minor versions; the Godot version is pinned per release and upgraded deliberately.
- The host can optionally link the simulation in-process via PyO3 later; the subprocess boundary is sufficient and safer for now.
- CI runs `psfc-replay` on x86-64, aarch64 and wasm32 and compares hash series; a mismatch fails the build.
