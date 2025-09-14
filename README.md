# REM
REM is a memo application. This app is desktop application built with [TAURI](https://v2.tauri.app/).
- Frontend: React.js
- Backend: Rust

# Installation
Download a binary file(.dmg) from [https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_aarch64.dmg](https://github.com/tttol/rem/releases/download/1.0.0/rem_1.0.0_aarch64.dmg)

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

