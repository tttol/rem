# REM
<img width="1996" height="1602" alt="image" src="https://github.com/tttol/rem/blob/main/docs/app.png?raw=true" />
<br/>
REM is a desktop application for task management. REM's data stays completely local, stored as JSON files on your machine. Nothing is sent to cloud storage or external servers.

# Installation
```sh
git clone https://github.com/tttol/rem.git
cd rem
npm i
cd src-tauri
npm run tauri dev
```

# Features
- **Task Management**: Create, update, and organize tasks across different status lanes (TODO, DOING, DONE, PENDING)
- **Drag & Drop**: Move tasks between status lanes
- **Bulk Delete**: Delete all completed tasks at once using the trash icon in the DONE lane
- **Local Storage**: All data is stored locally as JSON files on your machine
- **Auto-reload**: Task list automatically refreshes after operations

# Usage
<img width="1996" height="1602" alt="image" src="https://github.com/tttol/rem/blob/main/docs/demo2.gif?raw=true" />

# Tech Stack
This app is desktop application built with [TAURI](https://v2.tauri.app/).
- Frontend: React.js
- Backend: Rust

# Release build
### Windows
GitHub Actions automatically creates Windows release binaries whenever a git tag is created.

### Mac
Mac release binaries must be created manually. The explanation for why GitHub CI isn't used for Mac builds can be found in [this PR](https://github.com/tttol/rem/pull/5).
```bash
npm run tauri build -- --target universal-apple-darwin --bundles dmg
```
- `universal-apple-darwin` is an option of the universal binary that is compatible for both Aplle Silicon and Intel.
- ref: https://tauri.app/distribute/dmg/

