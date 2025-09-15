# REM
<img width="1996" height="1602" alt="image" src="https://github.com/tttol/rem/blob/main/docs/app.png?raw=true" />
<br/>
REM is a desktop application for task management. REM's data stays completely local, stored as JSON files on your machine. Nothing is sent to cloud storage or external servers.

# Installation
Download a binary file from [https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_aarch64.dmg](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_aarch64.dmg)

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

