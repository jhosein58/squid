# Squid Audio Engine

**A high-performance, modular, and `#[no_std]` audio synthesis engine written in Rust.**

Squid is an experimental DSP powerhouse designed from first principles for extreme efficiency, zero-latency performance, and portability. It bridges the gap between low-level embedded audio processing and high-level DAW capabilities, utilizing modern CPU features like SIMD for massive parallel processing.

---

## ⚡ Key Features & Highlights

### 🚀 Extreme Performance & SIMD
*   **SIMD Everywhere:** Data is stored and processed in blocks using **Structure of Arrays (SoA)** layout. Every processing step is vectorized.
*   **Massive Polyphony:** Capable of rendering **7,000 concurrent sawtooth oscillators** on a single 1GHz CPU thread.
*   **Fixed-Block Processing:** Decoupled from hardware buffer sizes. The internal engine runs on fixed-block sizes for deterministic CPU load and optimal cache usage.

### 🎛️ Pristine Audio Quality
*   **Advanced Anti-Aliasing:** Oscillators use **PolyBLEP** (Polynomial Band-Limited Step) algorithms to eliminate aliasing on hard-sync and sharp waveforms, fully implemented in SIMD.
*   **Smart Gain Staging:** Built-in summing bus architecture with **Soft Clipping** ensures infinite headroom and prevents harsh digital clipping.
*   **Dual-Buffer Architecture:** Lock-free communication between the audio thread and the UI. The audio callback **never blocks**.

### 🛠️ Architecture & Design
*   **`#[no_std]` Core:** The core logic has zero dependencies and can run on bare-metal embedded systems or WebAssembly.
*   **Zero-Allocation at Runtime:** All memory is allocated upfront. The engine operates almost entirely on the stack or pre-allocated buffers during the audio callback.
*   **Universal Signal Path:** **"Everything is a Signal."** There is no distinction between audio and control signals. The output of any module can modulate any parameter of another.
*   **Modular Components:** Highly granular design. An Oscillator is composed of smaller, reusable atoms like *Phase Accumulators* and *Phase Shapers*.
*   **Compile-Time Configuration:** Critical parameters (Sample Rate, Block Size, SIMD width) are tunable via `config.rs` for maximum optimization.

### 🎨 Reactive Lua UI Framework
*   **Custom GUI Stack:** A lightweight, reactive UI framework written in Lua with minimal dependencies.
*   **Component-Based:** UI elements are modular components that react to state changes.
*   **Visualizers:** Includes custom drivers for stable oscilloscope rendering (Zero-crossing triggers) and high-performance metering.

---

## 🏗️ Architecture Overview

Squid follows a strict three-tier architecture to ensure separation of concerns:

### 1. The Core (`squid-core`) - `#[no_std]`
The mathematical heart of the engine. It knows nothing about the OS, threads, or heap allocation.
*   **Pure Math & DSP:** FFT, Filters, Oscillators (SIMD).
*   **Traits:** `Oscillator`, `Processor`, `Modulator`.
*   **Platform Agnostic:** Runs on Microcontrollers, WASM, or Desktop.

### 2. The Engine (`squid-engine`) - `std`
The glue layer that brings the core to life in a desktop environment.
*   **IO Management:** Interfaces with CPAL or custom audio drivers.
*   **Thread Safety:** Manages lock-free ring buffers and atomic state synchronization.
*   **File IO:** Includes a custom `.wav` writer (header generation + PCM streaming).

### 3. The App (`squid-app`) - Tauri + Lua
The visual layer.
*   **Scripting:** Logic and UI layout controlled via Lua.
*   **Rendering:** High-performance canvas rendering for waveforms and controls.

---

## 🎹 Core Concepts

Squid's modularity means you don't just "use" an oscillator; you build the signal flow.

*   **Oscillator**: Deconstructed into *Phase Accumulators* (tracking time/frequency) and *Shapers* (converting phase to amplitude via PolyBLEP/Wavetable).
*   **Processor**: Transformers like Filters, Bitcrushers, or Waveshapers that take a block of SIMD data and mutate it.
*   **Modulator**: LFOs and Envelopes are treated as audio signals (DC coupled), allowing for audio-rate modulation of any parameter.

---

## 🚀 Quick Example: Generating a WAV

Here’s how simple it is to generate high-quality audio using the high-level API, while the engine handles SIMD and buffering under the hood.
```rust
// Simple realtime sine-wave generator using Squid Engine
fn main() {
    let ctx = ProcessContext::default();
    let mut osc = SinOsc::new();

    // (frequency, sample rate, optional initial phase)
    osc.configure(440.0, 44100.0, None);

    // Start realtime playback
    let _pd = LivePlayback::new(move |out| {
        osc.process(&ctx, out);
    });

    loop {
        sleep(Duration::from_secs(1));
    }
}
```
---

## 🔮 Roadmap

*   [x] SIMD PolyBLEP Oscillators
*   [x] Lock-free Audio Thread
*   [x] Lua UI Framework
*   [ ] Filter Implementation (Ladder/State Variable)
*   [ ] Envelope Generators (ADSR)
*   [ ] VST/CLAP Plugin Wrapper

---
*License: MIT*

