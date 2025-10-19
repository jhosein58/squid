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
## 🚀 Quick Example: Generating a WAV file

Here’s a practical example demonstrating how to use the engine to generate a 3-second, 440 Hz sine wave and save it as a `.wav` file. This showcases the simplicity of the API.
```rust
// main.rs

fn main() {
    // --- 1. Setup the Synthesis Environment ---

    // Define the sample rate for our audio context.
    let sample_rate = 44100;

    // --- 2. Create the Sound Source ---

    // Instantiate a Sine Oscillator.
    // This will be our basic sound generator.
    let mut sine_osc = SinOsc(sample_rate);

    // Set the oscillator's frequency to 440 Hz (the note 'A4').
    sine_osc.set_frequency(440.0);

    // --- 3. Prepare the Output ---

    // Create a WAV file container with CD-quality mono specs
    // (16-bit, 44100 Hz, 1 channel).
    let mut wav_file = Wav(WavSpec::cd_mono());

    // --- 4. Generate the Audio Samples ---

    // Run a loop to generate 3 seconds of audio.
    // For each step, get the next sample from our oscillator.
    println!("Generating 3 seconds of a 440 Hz sine wave...");
    for _ in 0..(sample_rate * 3) {
        wav_file.push_sample(sine_osc.next_sample());
    }

    // --- 5. Save the Result ---

    // Write all the generated samples to a file named "output.wav".
    wav_file.write_to_path("output.wav").unwrap();

    println!("Successfully saved to output.wav!");
}
```
This code is self-contained and demonstrates a complete workflow: **Setup -> Generate -> Save**. It highlights how easy it is to get started with the fundamental building blocks of the Squid engine.
