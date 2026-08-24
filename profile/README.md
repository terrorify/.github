<div align="center">

# ⚡ TERRORIFY

### Game Development Studio & High-Performance Multiplayer Technology

[![Rust](https://img.shields.io/badge/Rust-Native_Performance-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Java 21](https://img.shields.io/badge/Java_21-Virtual_Threads-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white)](https://openjdk.org/)
[![Netty](https://img.shields.io/badge/Netty-Async_epoll_I/O-00599C?style=for-the-badge&logo=cplusplus&logoColor=white)](https://netty.io/)
[![GeyserMC](https://img.shields.io/badge/GeyserMC-Bedrock_Bridge-00D26A?style=for-the-badge&logo=minecraft&logoColor=white)](https://geysermc.org/)
[![Adventure](https://img.shields.io/badge/Kyori-Adventure_UI-FF4081?style=for-the-badge&logo=starship&logoColor=white)](https://github.com/KyoriPowered/adventure)
[![SpacetimeDB](https://img.shields.io/badge/SpacetimeDB-Real--Time_State-7B2CBF?style=for-the-badge&logo=databricks&logoColor=white)](https://spacetimedb.com/)

<p align="center">
  <b>Terrorify</b> is an independent game development studio engineering the next generation of high-frequency, ultra-responsive multiplayer worlds. We combine native compute, zero-copy foreign function bridging, and server-authoritative simulation to deliver unparalleled performance and competitive integrity.
</p>

---

[🎮 What Players Feel](#-the-player-experience--performance) • [🛡️ Server Authority](#-server-authority-preventing-the-impossible) • [👁️ GameSense](#-gamesense-catching-the-probability-of-perfection) • [🛠️ Our Toolchain](#-our-engineering-toolchain) • [🌐 Public Projects](#-public-projects)

---

</div>

<br/>

## 🎮 The Player Experience & Performance

Our technology stack is engineered around one standard: **flawless responsiveness and absolute reliability**.

```
┌───────────────────────────────┐     ┌───────────────────────────────┐
│     50–70% RAM Reduction      │     │     Sub-Millisecond Ticks     │
│   Off-heap zero-alloc paths   │     │    Consistent 20.0 TPS rate   │
└──────────────┬────────────────┘     └──────────────┬────────────────┘
               │                                     │
               ▼                                     ▼
      ╔═════════════════════════════════════════════════════╗
      ║              THE TERRORIFY EXPERIENCE               ║
      ║     Zero Rubberbanding • Flawless Hit Registration  ║
      ╚═════════════════════════════════════════════════════╝
```

### ⚡ Measurable Outcomes
* **🚀 Zero Movement Rubberbanding**: Fluid, instant motion synchronization with zero positional snap-backs during intense combat or parkour.
* **🎯 Flawless Hit Registration**: Exact server-calculated line-of-sight and 3D raycasting ensure every projectile and melee strike connects exactly where seen.
* **📉 Massive Memory & CPU Reduction**: Up to **50–70% lower RAM consumption** and drastically reduced CPU overhead through cache-friendly structures, off-heap native memory allocation, and eliminated garbage collection pauses.
* **🛡️ Real-Time Exploit Prevention**: Traditional impossible physical feats are invalidated instantly at the server tick level before reaching the game state.

---

## 🛡️ Server Authority: Preventing the Impossible

> **Server Authority is Real-Time.** Its objective is simple: **make impossible physics mechanically impossible to execute.**

```
[ Client Input ] ──▶ [ Real-Time Server Authority ] ──▶ [ Verified World State ]
                             │
                             ├─▶ 3D AABB Volumetric Boundary Enforcement
                             ├─▶ Server-Calculated Line-of-Sight & Raycasting
                             └─▶ Deterministic Knockback & Trajectory Physics
```

Rather than trusting client-reported positions, our server authority architecture computes and validates all physical interactions on the fly:
* ❌ **Speed & Flight**: Movement vectors, acceleration limits, and air-time envelopes are verified tick-by-tick.
* ❌ **Phase & V-Clip (NoClip)**: Volumetric spatial voxel collision prevents block penetration.
* ❌ **Impossible Reach & Angle Exploits**: 3D geometric raycasting rejects all out-of-range or occluded interactions.
* ❌ **Timer & Blink Exploits**: Motion budgets and strict client packet pacing prevent time manipulation.
* ❌ **Knockback & Velocity Spoofing**: Momentum and trajectory vectors are computed purely by the server.

---

## 👁️ GameSense: Catching the Probability of Perfection

> **GameSense is a Slow-Burn, Delayed Evidence Engine.** While Server Authority prevents the *impossible*, GameSense is built to catch the **probability of perfection** in closet cheating.

```
       [ Server Authority Telemetry ]
                     │
                     ▼
       [ GameSense Evidence Engine ]
         (Statistical Accumulation)
                     │
                     ▼
        [ Case File Dossier Generated ]
                     │
         ┌───────────┴───────────┐
         ▼                       ▼
  [ Staff Review Portal ]    [ Ban Wave Queue ]
  (Behavioral Deep Dive)     (Delayed Action Wave)
```

Closet cheaters do not fly or speed across maps; they subtly manipulate micro-mechanics (subtle aim smoothing, micro-reach, humanly improbable consistency over time). 

### 🔬 The Detection-to-Case Lifecycle:
1. **Telemetry Ingestion**: Gathers continuous fine-grained telemetry data generated from Server Authority validation.
2. **Statistical Anomaly Scoring**: Evaluates movement distributions, angle deltas, and consistency curves over extended play sessions.
3. **Case Dossier Assembly**: When suspicious perfection is detected, GameSense builds an evidence case file containing timeline logs and statistical proof.
4. **Staff Review & Wave Execution**: Integrated with moderator dashboards for behavioral review and scheduled delayed ban waves, keeping cheat developers completely in the dark.

> [!NOTE]
> **Foundational Telemetry Research**: Our underlying telemetry schema and baseline feature tracking reference historical competitive research from [FrozenOrb/HAL_Prediction](https://github.com/FrozenOrb/HAL_Prediction) and [FrozenOrb/HALData](https://github.com/FrozenOrb/HALData).

---

## 🛠️ Our Engineering Toolchain

We build on top of battle-tested, high-performance open-source foundations and modern systems engineering standards:

<div align="center">

| Layer | Technologies | Core Purpose & Implementation |
|:---|:---|:---|
| **Systems & Execution** | `Rust`, `Java 21 (LTS)` | Memory-safe low-level compute, concurrent virtual thread scheduling & zero-cost abstractions. |
| **Networking & I/O** | `Netty`, `Native epoll`, `Direct ByteBufs` | Event-driven non-blocking packet pipelines, zero-copy socket buffers & kernel event multiplexing. |
| **Cross-Play & Ingress** | `GeyserMC`, `Floodgate` | Universal Bedrock protocol bridging, seamless console/mobile cross-play & bedrock authentication. |
| **Client Integrations** | `Apollo` | Native Lunar Client visual integration, custom HUD elements, waypoints & rich mod communication. |
| **Proximity Voice & Audio** | `Simple Voice Chat`, `Plasmo Voice` | High-definition 3D positional audio, spatial sound attenuation & real-time voice streaming. |
| **Native Interop** | `Zero-Copy C-ABI`, `Native FFM` | Direct off-heap memory bindings bridging high-level game logic to native Rust physics in microseconds. |
| **State & Persistence** | `SpacetimeDB`, `Caffeine Cache` | Real-time distributed relational database synchronization & high-hitrate concurrency caches. |
| **Math & Spatial Queries** | `SIMD Vector Operations`, `Voxel Math` | Hardware-accelerated swept AABB intersection calculations & volumetric raycasting. |
| **UI & Text Rendering** | `Kyori Adventure`, `MiniMessage` | Rich, responsive component-based UI rendering pipelines and clean player messaging. |

</div>

---

## 🌐 Public Projects

Explore our open-source tools, protocol bridges, and spatial libraries:

| Project | Description | Stack |
|:---|:---|:---|
| [**`FOST`**](https://github.com/terrorify/fost) | **Fabric of Spacetime**: Real-time Fabric server state synchronization adapter connecting worlds directly to SpacetimeDB. | `Java` • `Fabric` • `SpacetimeDB` |
| [**`Folia4dB`**](https://github.com/terrorify/folia4db) | High-concurrency SpacetimeDB relational state connector tailored for multi-threaded Folia server platforms. | `Java` • `Folia` • `SpacetimeDB` |
| [**`polar4d`**](https://github.com/terrorify/polar4d) | High-performance spatial-temporal voxel store designed for lightning-fast spatial queries and chunk memory compaction. | `Rust` • `Native` |
| [**`mcpclient`**](https://github.com/terrorify/mcpclient) | Developer tooling and client SDK for the Model Context Protocol (MCP) ecosystem. | `TypeScript` • `MCP` |

---

<div align="center">

### 🤝 Connect with Terrorify

Building high-frequency virtual worlds and ultra-optimized game infrastructure.

[![Website](https://img.shields.io/badge/Website-terrorify.dev-000000?style=for-the-badge&logo=googlechrome&logoColor=white)](https://terrorify.dev)
[![GitHub](https://img.shields.io/badge/GitHub-terrorify-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/terrorify)

<sub>© 2026 Terrorify. All rights reserved.</sub>

</div>
