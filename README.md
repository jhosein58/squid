# Squid Audio Engine

**A modular, experimental, and performance-oriented audio synthesis and processing engine written in Rust.**

Squid is designed from the ground up to be flexible and efficient, providing the core building blocks for creating everything from simple synthesizers to complex audio applications.

---

## 🦀 Philosophy & Architecture

The project is built on a clear, three-tiered architecture, separating concerns for maximum flexibility and portability.

### 1. The Core (`squid-core`) - `#[no_std]`
This is the heart of the engine. A pure, dependency-free, and platform-agnostic library that runs anywhere—from embedded microcontrollers to WebAssembly. It knows nothing about operating systems, memory allocation, or threading.

*   **Pure Math:** All DSP algorithms are implemented here.
*   **Traits:** Defines the fundamental abstractions like `Oscillator`, `Processor`, and `Modulator`.
*   **Zero Dependencies:** Truly lightweight and portable.
*   **Static Dispatch:** Uses generics and traits for compile-time polymorphism, ensuring zero-overhead abstractions.

### 2. The Engine (`squid-engine`) - `std`
This layer builds upon `squid-core` by integrating with the standard library. It provides the necessary "glue" to make the core usable in a typical desktop or server environment.

*   **Audio I/O:** Handles communication with the operating system's audio devices.
*   **Real-time Management:** Manages audio callbacks, buffering, and thread safety.
*   **Dynamic Structures:** Introduces dynamic collections and structures where needed (e.g., managing a dynamic list of effects).

### 3. The Application Layer (Future) - The DAW
This is the ultimate goal. A user-facing application, envisioned as a Digital Audio Workstation (DAW), built on top of the Squid engine. This layer will handle the user interface, plugin management, and session control.

*   **Status:** Not yet implemented.
*   **Vision:** To provide an intuitive graphical interface for creating and manipulating sound with the Squid engine.

---

## 🎹 Core Concepts

Squid's `core` is built around a few simple but powerful traits:

*   **`Oscillator`**: The source of sound. This trait represents anything that generates a periodic waveform, such as sine, square, or sawtooth waves. Custom oscillators, like wavetable or granular synths, can be easily implemented.

*   **`Processor`**: Anything that transforms a signal. This includes effects like distortion, delay, filters, and bit-crushers. Processors can be chained together to create complex effect racks.

*   **`Modulator`**: A source of control signals. Modulators, like LFOs (Low-Frequency Oscillators) or envelopes (ADSR), don't produce audible sound themselves but are used to dynamically change the parameters of other modules (e.g., modulating an oscillator's pitch or a filter's cutoff frequency).

---

## 🚀 Quick Example

Here’s a glimpse of how you can combine modules in `squid-core`. This example creates a simple synth voice where a sine wave is processed by a soft saturator.
```rust
// This is a conceptual example of using the core library

// 1. Define the building blocks
let mut sine_osc = SinOsc::new(44100.0);
sine_osc.set_frequency(440.0); // A4 note

let mut saturator = SoftSaturator::new(5.0); // Add some warm distortion

// 2. In your audio loop, process a sample
fn get_next_sample(osc: &mut SinOsc, proc: &mut SoftSaturator) -> f32 {
let raw_sample = osc.process();
let processed_sample = proc.process(raw_sample);
processed_sample
}

// The final output is a warm, slightly distorted sine wave.

## 🗺️ Project Status & Roadmap

*   [x] **Core DSP (`squid-core`)**: `no_std` abstractions and initial set of modules are complete and functional.
*   [ ] **Engine Layer (`squid-engine`)**: Basic structure is in place, but audio I/O and real-time safety are under active development.
*   [ ] **Application (DAW)**: A long-term goal. Planning and design will begin after the engine layer is stable.

Contributions and feedback are welcome! Feel free to open an issue or submit a pull request.
