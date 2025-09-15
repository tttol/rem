# REM
<img width="1996" height="1602" alt="image" src="https://github.com/tttol/rem/blob/main/docs/app.png?raw=true" />
<br/>
REM is a desktop application for task management. REM's data stays completely local, stored as JSON files on your machine. Nothing is sent to cloud storage or external servers.

# Installation

## Download Links

| Platform | Architecture | Download |
|----------|-------------|----------|
| **macOS** | Apple Silicon (M1/M2/M3) | [rem_1.0.0_aarch64.dmg](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_aarch64.dmg) |
| **macOS** | Intel | [rem_1.0.0_x64.dmg](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_x64.dmg) |
| **Windows** | 64-bit (exe) | [rem_1.0.0_x64-setup.exe](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_x64-setup.exe) |
| **Windows** | 64-bit (msi) | [rem_1.0.0_x64_en-US.msi](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_x64_en-US.msi) |
| **Windows** | 32-bit (exe) | [rem_1.0.0_x86-setup.exe](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_x86-setup.exe) |
| **Windows** | 32-bit (msi) | [rem_1.0.0_x86_en-US.msi](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_x86_en-US.msi) |
| **Windows** | ARM64 (exe) | [rem_1.0.0_arm64-setup.exe](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_arm64-setup.exe) |
| **Windows** | ARM64 (msi) | [rem_1.0.0_arm64_en-US.msi](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_arm64_en-US.msi) |

Or visit the [latest release page](https://github.com/tttol/rem/releases/latest) to download the appropriate version for your system.

# Usage
<img width="1996" height="1602" alt="image" src="https://github.com/tttol/rem/blob/main/docs/demo2.gif?raw=true" />



# Tech Stack
This app is desktop application built with [TAURI](https://v2.tauri.app/).
- Frontend: React.js
- Backend: Rust

# Develop build
```sh
cd src-tauri
npm run tauri dev
```

# Release build
```bash
npm run tauri build -- --bundles dmg
```
ref: https://tauri.app/distribute/dmg/

