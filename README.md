# rust-adb-control

**Rust project demonstrating Android automation using ADB — taps, swipes, typing, and app launches.**

> A small Rust project to show automation on Android devices. Intended for educational demos and personal projects. Only run on devices you own or have permission to test.

---

## Features
- Connect to an Android device over TCP (IP:PORT)
- Wake device, swipe, tap coordinates, type text
- Launch apps (WhatsApp) or open URLs (LinkedIn)
- Clean Rust code demonstrating ADB automation

---

## Requirements
- Rust toolchain (stable)
- `adb` installed and in your PATH
- Android device with ADB over TCP enabled (or USB debugging)

---

## How to Run
Clone the repository:

```bash
git clone https://github.com/Mina-Noshy/rust-adb-control.git
cd rust-adb-control
cargo run

---

## Project Structure
rust-adb-control/
├─ Cargo.toml
├─ README.md
├─ src/
│  ├─ main.rs           
│  ├─ services/
│  │  └─ adb.rs        
│  └─ utilities/
│     ├─ printing_helper.rs
│     └─ io_helper.rs

