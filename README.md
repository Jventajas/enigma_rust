# Enigma Machine Simulator

<p align="center">
  <img src="https://img.shields.io/badge/release-v1.0-blue" alt="release version">
  <img src="https://img.shields.io/badge/license-MIT-green" alt="license">
  <img src="https://img.shields.io/badge/made%20with-Rust-orange" alt="made with rust">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white&style=for-the-badge" alt="Rust">
  <img src="https://img.shields.io/badge/Rocket%20Framework-FF0000?logo=rocket&logoColor=white&style=for-the-badge" alt="Rocket">
  <img src="https://img.shields.io/badge/HTML5-E34F26?logo=html5&logoColor=white&style=for-the-badge" alt="HTML5">
  <img src="https://img.shields.io/badge/CSS3-1572B6?logo=css3&logoColor=white&style=for-the-badge" alt="CSS3">
</p>

## 🚀 Overview

This project is a **web-based Enigma Machine Simulator**, built in **Rust** using the [Rocket framework](https://rocket.rs). It is a historically accurate recreation of the World War II-era German encryption device, the **Enigma Machine**.

By visiting the web interface of this project, users can simulate the encryption and decryption of messages, configure rotor settings, reflectors, and plugboards just like the original Enigma Machine.

---

## ✨ Features

- **Fully functional Enigma Machine Simulation**: Includes support for 3 rotors, configurable reflectors, and a dynamic plugboard.
- **Highly Customizable**: Rotors, initial positions, ring settings, and plugboard configurations can all be adjusted to mimic complex encryption scenarios.
- **Modern Web Interface**: A lightweight HTML/CSS-driven UI enables easy message encryption/decryption online.
- **Project Designed for Performance**: Built in Rust for reliability and high-speed performance, ensuring the simulator is secure and efficient.
- **Clean and Well-Structured Codebase**: Written using modular Rust concepts, making it extendable and easy to maintain.

---

## 🛠 Technologies Used

- **Programming Language**: Rust
- **Web Framework**: [Rocket 0.5.1](https://rocket.rs)
- **Template Engine**: [rocket_dyn_templates 0.2.0](https://docs.rs/rocket_dyn_templates/latest/rocket_dyn_templates/)
- **Frontend**: HTML5, CSS3

---

## 📜 How It Works

The Enigma Machine Simulator is broken down into the following primary components:

1. **Rotors** - Simulate the core encryption logic by scrambling characters. Each rotor rotates for added complexity.

2. **Reflector** - Enables bi-directional scrambling by reflecting characters post-rotor configuration.

3. **Plugboard** - Provides additional character mappings to enhance encryption.

4. **Stepping Logic** - Implements historically accurate rotor stepping movements (double-step mechanism).

5. **Rocket Web Server** - Hosts the front-end interface and processes encryption requests.

---

## 🚧 Usage

### Local Setup

To run the Enigma Machine Simulator locally, ensure you have Rust installed. Then follow these steps:

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/Jventajas/enigma_rust.git
   cd enigma_rust
   ```

2. **Install Dependencies**:

   Since the project uses the Rocket framework, install all necessary dependencies and tools for Rust.

   ```bash
   cargo build
   ```

3. **Run the Application**:

   Start the Rocket web server by running:

   ```bash
   cargo run
   ```

4. **Access the Simulator**:

   Visit the application in your browser at `http://localhost:8000`.

---

## 🌐 Web Interface ⚡

1. **Homepage**:

   Enter the text to encrypt or decrypt, configure the rotors, plugboard settings, and hit "Encrypt" to process the input.

2. **Encryption**:

    - After submitting the message, the encrypted or decrypted message will be displayed on the result page.
    - Dynamic rotors and plugboard ensure that configurations are consistent with the WWII Enigma Machine.

---

## 🧩 Enigma Components

The core of this repository revolves around the implementation of realistic Enigma Machine operations:

- **Rotors (rotor.rs)**:
    - Each rotor has specific wiring and features a notch for the stepping mechanism.
    - The rotors encode characters both forward and in reverse directions.

- **Plugboard (plugboard.rs)**:
    - Enables manual character swaps before and after the rotor processing.

- **Reflector (reflector.rs)**:
    - Reflects the signal, ensuring reversibility for decryption.

- **Enigma Machine (enigma_machine.rs)**:
    - Combines all components together as a cohesive simulation of the historic device.

---
