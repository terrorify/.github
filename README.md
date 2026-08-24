<div align="center">

# ⚡ TERRORIFY

### High-Performance Multiplayer Game Development Studio

[![Rust](https://img.shields.io/badge/Rust-Native_Performance-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Java 21](https://img.shields.io/badge/Java_21-Virtual_Threads-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white)](https://openjdk.org/projects/jdk/21/)
[![Netty](https://img.shields.io/badge/Netty-Async_I/O-00599C?style=for-the-badge&logo=cplusplus&logoColor=white)](https://netty.io/)
[![GeyserMC](https://img.shields.io/badge/GeyserMC-Bedrock_Bridge-00D26A?style=for-the-badge&logo=minecraft&logoColor=white)](https://geysermc.org/)
[![Badlion Client](https://img.shields.io/badge/Badlion-ModAPI-FFA000?style=for-the-badge&logo=lion&logoColor=white)](https://github.com/BadlionClient/BadlionClientModAPI)
[![LabyMod](https://img.shields.io/badge/LabyMod-Server_API-1E88E5?style=for-the-badge&logo=wolfpack&logoColor=white)](https://dev.labymod.net/)
[![Adventure](https://img.shields.io/badge/Kyori-Adventure_UI-FF4081?style=for-the-badge&logo=starship&logoColor=white)](https://github.com/KyoriPowered/adventure)
[![SpacetimeDB](https://img.shields.io/badge/SpacetimeDB-Real--Time_State-7B2CBF?style=for-the-badge&logo=databricks&logoColor=white)](https://spacetimedb.com/)

<br/>

<p align="center">
  <b>Terrorify</b> is an independent game development studio building ultra-responsive, high-frequency multiplayer worlds.<br/>
  We focus on server-authoritative physics, low-latency networking, and high-performance game tech.
</p>

---

[🎮 Player Experience](#-the-player-experience) • [🛡️ Server Authority](#-server-authority-stopping-impossible-actions) • [👁️ GameSense](#-gamesense-catching-unnatural-perfection) • [🔬 Open Gists](#-open-algorithms--gists) • [🛠️ Toolchain](#-our-engineering-toolchain) • [🌐 Public Projects](#-public-projects)

---

</div>

<br/>

## 🎮 The Player Experience

We engineer our games around a single standard: **smooth gameplay, zero lag spikes, and fair competition.**

<div align="center">

| 🚀 Zero Rubberbanding | 🎯 Precise Hit Registration |
|:---|:---|
| Smooth, uninterrupted movement with zero annoying snap-backs, even during high-speed parkour and combat. | Attacks and projectiles register exactly where you aim—no ghost hits, phantom swings, or desyncs. |

| ⚡ Unshakable 20.0 TPS | 📉 50–70% Lower Memory |
|:---|:---|
| Consistent sub-millisecond server ticks that stay fast and responsive, even during massive player battles. | Efficient off-heap data structures that eliminate garbage collection lag spikes and keep servers light. |

</div>

<br/>

---

## 🛡️ Server Authority: Stopping Impossible Actions

> **Server Authority happens in real time.** If a move is physically impossible, the server blocks it instantly on that exact tick.

Instead of trusting what a player's computer claims happened, our server calculates the physical world itself:

```
[ Player Action ] ──▶ [ Server Physics Engine ] ──▶ [ Verified Game World ]
                              │
                              ├── Strictly checks block collisions (No walking through walls)
                              ├── Validates true distance & line of sight (No impossible reach)
                              └── Enforces natural gravity & velocity (No flying or super-speed)
```

<br/>

<div align="center">

| Exploit Type | What Cheaters Try To Do | How Server Authority Blocks It |
|:---|:---|:---|
| **Speed & Flying** | Moving faster than game limits or hovering in air. | The server calculates legal speed and gravity; illegal positions are rejected immediately. |
| **Wall-Clips (Phase/NoClip)** | Glitching or teleporting through solid blocks. | 3D voxel collision prevents bounding boxes from entering solid obstacles. |
| **Impossible Reach** | Hitting targets beyond normal player range. | The server casts a 3D ray to the target; hits that exceed distance or go through walls fail. |
| **Timer / Speeding Ticks** | Sending packet bursts to run or mine faster. | The server enforces a strict time budget of 1 tick per 50ms. |
| **Knockback Cancel** | Ignoring velocity when hit by attacks. | Knockback trajectories are calculated server-side; players must follow the applied force. |

</div>

<br/>

---

## 👁️ GameSense: Catching Unnatural Perfection

> **GameSense is a delayed detection engine.** While Server Authority blocks *impossible* physics, GameSense catches the **unnatural perfection** of subtle closet cheats.

Closet cheaters try to hide by using subtle tools like soft aim-assist or auto-clickers that stay barely within physical limits. Human hands naturally make tiny mistakes and tire over time—machines do not.

```
       [ Server Gameplay Telemetry ]
                     │
                     ▼
       [ GameSense Evidence Engine ]
      (Tracks statistical consistency)
                     │
                     ▼
         [ Case File Assembled ]
                     │
         ┌───────────┴───────────┐
         ▼                       ▼
  [ Staff Review Tool ]      [ Scheduled Ban Wave ]
(Review logs & replays)   (Keeps cheat makers blind)
```

### 🔬 The Detection-to-Case Lifecycle
1. **Telemetry Ingest**: Records fine-grained movement and combat data generated during normal play.
2. **Statistical Analysis**: Compares player consistency, angle precision, and click patterns against normal human ranges.
3. **Evidence Dossier**: Builds a detailed timeline report with logs and replay clips whenever unnatural perfection is identified.
4. **Staff Review & Ban Waves**: Moderators review compiled cases, and confirmed cheaters are banned in delayed waves so cheat authors cannot test workarounds.

> [!NOTE]
> **Research Lineage**: Our data collection formats reference historical competitive research from [FrozenOrb/HAL_Prediction](https://github.com/FrozenOrb/HAL_Prediction) and [FrozenOrb/HALData](https://github.com/FrozenOrb/HALData).

<br/>

---

## 🔬 Open Algorithms & Gists

Explore standalone open-source implementations of our perception math, collision algorithms, and spatial memory engines:

<div align="center">

| Research Algorithm & Gist | Focus Area | Plain English Summary |
|:---|:---|:---|
| [**👁️ Volumetric Perception Authority (3D DDA)**](https://gist.github.com/McMmax/bb7a4e81f14e69b10336c35a96f9ceab) | *Anti-ESP / Line of Sight* | A fast 3D raycaster that checks if an entity is hidden behind walls. If hidden, the server never sends its data to the client, making wallhacks and ESP impossible. |
| [**⚡ SIMD Swept AABB Collision Solver**](https://gist.github.com/McMmax/b38449bcd2b16d8e142b1ad030f235e6) | *Continuous Collision (CCD)* | A branchless vector collision solver that sweeps 3D bounding boxes along velocity paths to prevent high-speed tunneling and calculate exact impact timing. |
| [**🧱 3D Voxel Grid Math & Traversal**](https://gist.github.com/McMmax/a6f236db9c96caefe576abe167dfe774) | *Discrete Voxel Footprints* | Fast discrete grid utilities that determine the exact block footprint of continuous physical volumes and iterate through intersecting block shapes with zero allocations. |
| [**🧊 3D Morton Z-Order Spatial Indexing**](https://gist.github.com/McMmax/2a24c020ffd5b9834df80cc9c9eb3b7c) | *Fast Voxel Memory* | A bit-interleaving technique (using modern CPU BMI2 instructions) that keeps neighboring 3D blocks close together in RAM for instant, cache-friendly lookups. |

</div>

<br/>

---

## 🛠️ Our Engineering Toolchain

We build with modern, high-performance open-source tools and industry standards:

<div align="center">

| Layer | Tools & Standards | What We Use It For |
|:---|:---|:---|
| **Core Systems** | [**`Rust`**](https://www.rust-lang.org/) • [**`Java 21`**](https://openjdk.org/projects/jdk/21/) | Memory-safe native code for physics, paired with modern Java virtual threads for game logic. |
| **Networking & I/O** | [**`Netty`**](https://netty.io/) • `Direct ByteBufs` | High-speed, non-blocking network pipelines that handle thousands of packets per second. |
| **Cross-Platform Play** | [**`GeyserMC`**](https://geysermc.org/) • [**`Floodgate`**](https://geysermc.org/wiki/floodgate) | Bridges Minecraft Bedrock (consoles, mobile, Windows 10) so everyone can play together seamlessly. |
| **Client Integrations** | [**`Apollo`**](https://apollo.lunarclient.dev/) • [**`Badlion`**](https://github.com/BadlionClient/BadlionClientModAPI) • [**`LabyMod`**](https://dev.labymod.net/) • [**`WhatsMyClient`**](https://github.com/terrorify/WhatsMyClient) | Client brand identification, Lunar HUD/waypoint hooks, Badlion ModAPI enforcement, and LabyMod displays. |
| **Proximity Voice Chat** | [**`Simple Voice Chat`**](https://modrinth.com/plugin/simple-voice-chat) • [**`Plasmo Voice`**](https://plasmovoice.com/) | Real-time 3D spatial audio where voices get louder as players walk closer. |
| **Native Interop** | `Zero-Copy C-ABI` • `Native FFM` | Connects Java game logic to native Rust calculations in microseconds with zero memory overhead. |
| **State & Caching** | [**`SpacetimeDB`**](https://spacetimedb.com/) • [**`Caffeine Cache`**](https://github.com/ben-manes/caffeine) | Real-time relational database sync and lightning-fast in-memory caching. |
| **Math & Collisions** | [**`SIMD Vector Math`**](https://gist.github.com/McMmax/b38449bcd2b16d8e142b1ad030f235e6) • [**`Voxel Math`**](https://gist.github.com/McMmax/a6f236db9c96caefe576abe167dfe774) | Fast vector calculations for bounding box intersections and discrete grid raycasting checks. |
| **UI & Player Messaging** | [**`Kyori Adventure`**](https://github.com/KyoriPowered/adventure) • [**`MiniMessage`**](https://docs.advntr.dev/minimessage) | Clean, rich text formatting and responsive menus across all player screens. |

</div>

<br/>

---

## 🌐 Public Projects

Explore our open-source tools and connectors:

<div align="center">

| Project | Description | Stack |
|:---|:---|:---|
| [**`WhatsMyClient`**](https://github.com/terrorify/WhatsMyClient) | Accurate Minecraft client brand and handshake identification library with Lunar, Badlion, and LabyMod support. | `Kotlin` • `Java` • `Netty` • `Network Protocol` |
| [**`FOST`**](https://github.com/terrorify/fost) | **Fabric of Spacetime**: Real-time Fabric server state synchronization adapter connecting worlds directly to SpacetimeDB. | `Java` • `Fabric` • `SpacetimeDB` |
| [**`Folia4dB`**](https://github.com/terrorify/folia4db) | High-concurrency SpacetimeDB relational state connector tailored for multi-threaded Folia server platforms. | `Java` • `Folia` • `SpacetimeDB` |
| [**`polar4d`**](https://github.com/terrorify/polar4d) | High-performance spatial-temporal voxel store designed for lightning-fast spatial queries and chunk memory compaction. | `Rust` • `Native` |
| [**`mcpclient`**](https://github.com/terrorify/mcpclient) | Developer tooling and client SDK for the Model Context Protocol (MCP) ecosystem. | `TypeScript` • `MCP` |

</div>

<br/>

---

<div align="center">

### 🤝 Connect with Terrorify

Building high-frequency virtual worlds and ultra-optimized game infrastructure.

[![Website](https://img.shields.io/badge/Website-terrorify.dev-000000?style=for-the-badge&logo=googlechrome&logoColor=white)](https://terrorify.dev)
[![GitHub](https://img.shields.io/badge/GitHub-terrorify-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/terrorify)

<sub>© 2026 Terrorify. All rights reserved.</sub>

</div>