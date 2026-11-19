# ADR-0002: Client platforms and where rendering happens

Status: proposed · Date: 2026-10-16

## Context

Available hardware: a DigitalOcean droplet, several Raspberry Pis, two Chromebooks, and participants' own PCs. The product's identity is console-like (PS1-inspired controllers, Game Boy-style cartridges, a living-room session). Three questions were raised: could the 3D client run on the droplet, could the Pis be stacked to act as the client, and can the Chromebooks be clients.

Facts that bound the answer:

- Rendering one frame cannot be split across machines; several Pis do not add up to one better GPU. Clustering helps throughput workloads, not real-time graphics.
- The droplet has no GPU. Software rendering of 22 skinned footballers is far below playable frame rates, and remote play through video streaming adds roughly 60–150 ms of input latency on top of encoding, which defeats the "classic FIFA feel" requirement.
- A Raspberry Pi 5 has a Vulkan- and OpenGL ES 3.1-capable GPU and Godot 4.3+ ships official Linux arm64 builds. With a low-poly art budget and the Compatibility renderer, a 22-player scene at 720p is plausible but unproven for this game.
- Chromebooks run Linux desktop applications in a container with GPU acceleration on many models, and run browser applications natively. Godot's C# builds do not export to the web; GDScript and (experimentally) Rust GDExtension builds do.

## Decision

1. **The droplet is the primary world host, not a client.** Its public address removes NAT traversal from online play and places the relay fallback where every participant can reach it. A Pi at home is the backup target and alternative primary; the same Compose file runs on both.
2. **Three client tiers**, all from one Godot project:
   - Tier A, participants' PCs: primary target, full visuals.
   - Tier B, Raspberry Pi 5 in a printed console shell: supported once a measured render gate passes in phase 1 (22 animated tier-B footballers, ball, pitch and stadium shell at a 30 fps floor at 720p on the real device). Pi 4 and earlier are not targets.
   - Tier C, Chromebooks via the Linux container: best effort.
3. **The art budget is set to tier B from the start** (footballers ≤ 3 k triangles and ≤ 30 bones, single cheap shadow or baked, 1024² atlases, no heavy post-processing). This also matches the late-2000s console aesthetic the product wants.
4. **A browser client is the recorded alternative**, kept open rather than built: the Rust simulation already targets wasm32, so a TypeScript client with Three.js or Babylon.js, WebRTC data channels for rollback transport, the Gamepad API for controllers and the File System Access API for cartridges is feasible. It would trade the Godot editor's productivity for zero-install reach on every device including Chromebooks.

## Alternatives considered

| Option | Why not now |
|--------|-------------|
| Cloud-rendered client on the droplet with video streaming | No GPU; latency incompatible with the gameplay requirement; cost of GPU droplets is disproportionate |
| Pi cluster as a client | Rendering does not distribute; a single Pi 5 is the only Pi configuration that matters |
| Browser-native client as the primary | Strongest reach; weakest tooling for animation and scene work, and Rust GDExtension on the web is experimental. Revisit if Chromebooks become the main devices or if gdext web support matures |
| Godot web export with GDScript simulation | Simulation too slow and non-deterministic for rollback and host replay |

## Consequences

- Phase 0 includes a Pi 5 rendering spike and phase 1 includes the formal gate; the console shell is designed in `/hardware` only after the gate passes.
- Every asset is authored at tier-B budget with optional tier-A detail layers, never the reverse.
- The relay and lobby live on the droplet; the ledger lives wherever the group chooses, with nightly backups crossing to the other location.
- If the gate fails, the Pi keeps its host, backup and verifier roles and nothing else in the architecture changes.
