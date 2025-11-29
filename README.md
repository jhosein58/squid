# Squid Audio Engine

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![no_std](https://img.shields.io/badge/stdlib-no__std-green.svg)]()

**A hyper-performant, modular, and `#[no_std]` audio synthesis framework written in Rust.**

Squid is an experimental DSP powerhouse designed from first principles for extreme efficiency, deterministic low-latency performance, and portability. It bridges the gap between bare-metal embedded audio processing and high-level modular synthesis, leveraging modern CPU features like SIMD for massive parallel throughput.

[demo](https://github.com/user-attachments/assets/cd046c75-c085-434a-b218-a6e6dccdb039)

---

## ⚡ Performance Benchmarks

Squid is engineered for raw speed. Benchmarks performed on a consumer-grade **Intel Core i5-1035G1 (1.00 GHz)** single thread:

| Scenario | Buffer Size | Oscillators | Status |
| :--- | :---: | :---: | :--- |
| **Raw Throughput** | 1024 | **60,000+** | Stable (Sawtooth, Naive) |
| **Low Latency** | 64 | **13,000+** | Stable (Sawtooth, **PolyBLEP Anti-aliased**) |

*Note: Performance scales linearly with SIMD lane width (AVX2/AVX-512).*

---

## 🚀 Key Features

### 🧠 High-Performance DSP Architecture
*   **SIMD-First Design:** Data is stored and processed in **Structure of Arrays (SoA)** blocks. All internal processing is auto-vectorized/explicitly vectorized.
*   **Branchless Core:** The DSP kernel uses integer math logic and bitwise operations to eliminate branching penalties in critical loops.
*   **Fixed-Block Processing:** The internal engine runs on fixed block sizes (configurable), completely decoupled from the audio hardware buffer request size.
*   **Integer Phase Accumulation:** Precision-guaranteed phase tracking using `u32` wrapping arithmetic, eliminating floating-point errors and jitter.

### 🎛️ Pristine Audio Quality
*   **PolyBLEP Anti-Aliasing:** Oscillators utilize Polynomial Band-Limited Step algorithms to suppress aliasing artifacts above the Nyquist frequency without the overhead of oversampling.
*   **Decoupled Audio Thread:** Uses a **Lock-Free SPSC Ring Buffer** pipeline. The audio callback *never* blocks or waits for the render thread, ensuring glitch-free playback even under heavy load.
*   **Infinite Headroom:** Internal summing bus architecture with Soft Clipping prevents harsh digital distortion.

### 🛠️ System & Engineering
*   **`#[no_std]` & Dependency-Free:** The core library runs on bare metal, embedded microcontrollers, or WASM.
*   **Zero Runtime Allocation:** All memory is allocated upfront. The engine operates almost entirely on the stack or pre-allocated arenas, guaranteeing **zero GC pauses** and cache locality.
*   **"Everything is a Signal":** Unified modulation system. Audio and Control signals (LFOs, Envelopes) are treated identically, allowing for audio-rate modulation of *any* parameter.
*   **Modular Atomicity:** Oscillators are deconstructed into atomic components (Phase Accumulators, Phase Shapers), allowing for deep customization.

### 🎨 Reactive UI Ecosystem
*   **Lua-Powered Interface:** A custom, lightweight, and reactive UI framework built on Lua.
*   **Component-Based:** UI elements are modular and react instantly to engine state changes.
*   **Custom Drivers:** Includes a high-performance `.wav` writer and custom rendering drivers for oscilloscopes and metering.

---

## 🏗️ Architecture Hierarchy

1.  **`squid-core` (`no_std`):**
    The mathematical heart. Contains pure DSP algorithms, SIMD implementations, and traits (`Oscillator`, `Processor`). Platform agnostic.

2.  **`squid-engine` (`std`):**
    The desktop integration layer. Handles Threading (Park/Unpark), Audio I/O (CPAL), File I/O, and manages the `LivePlayback` pipeline.

3.  **`squid-app` (Lua):**
    The visual layer. Handles user interaction, script loading, and rendering via a high-performance 2D canvas.

---

## 💻 Quick Start

Generating high-performance audio with Squid is simple, despite the low-level optimizations under the hood:
```rust
use squid_core::{
    AudioNode,
    oscillators::{Oscillator, saw_osc::SawOsc},
    process_context::ProcessContext,
};
use squid_engine::LivePlayback;


fn main() {
    // 1. Create the context (holds sample rate, block size info)
    let ctx = ProcessContext::default();

    // 2. Initialize a PolyBLEP Sawtooth Oscillator
    let mut osc = SawOsc::new();
    osc.configure(440.0, 44100.0, None); // Freq, SampleRate, InitialPhase

    // 3. Initialize the Audio Backend (Low Latency)
    let mut pb = LivePlayback::init();

    // 4. Start the non-blocking render loop
    // The closure is moved to the dedicated render thread
    pb.start(move |out| {
        osc.process(&ctx, out);
    });
    
    // Keep the main thread alive
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
```
---

## 🔮 Roadmap

*   [x] SIMD PolyBLEP Oscillators
*   [x] Lock-free / Wait-free Audio Architecture
*   [x] Reactive Lua UI Framework
*   [ ] State Variable Filters (SIMD Optimized)
*   [ ] Vectorized Envelope Generators (ADSR)
*   [ ] Plugin Wrapper (VST3/CLAP)

---

*Licensed under the MIT License.*
