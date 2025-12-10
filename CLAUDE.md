# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Frontend (React + TypeScript)
- `npm run dev` - Start development server (Vite)
- `npm run build` - Build frontend (TypeScript compilation + Vite build)
- `npm run preview` - Preview built frontend

### Tauri Application
- `npm run tauri dev` - Start Tauri development mode (auto-builds Rust backend and starts React frontend)
- `npm run tauri build` - Build production Tauri application
- `cargo build` - Build Rust backend only (from src-tauri directory)
- `cargo check` - Check Rust code for compilation errors (from src-tauri directory)

## Architecture Overview

This is a desktop memo/todo application built with Tauri framework:

### Tech Stack
- **Frontend**: React 19 + TypeScript + Vite
- **Backend**: Rust (Tauri 2.0)
- **UI**: CSS with React components

### Project Structure
- `src/` - React frontend code
- `src-tauri/src/` - Rust backend code with modules:
  - `lib.rs` - Main library entry point with Tauri command handlers
  - `main.rs` - Application entry point
  - `core/` - Core business logic modules:
    - `create_task.rs` - Task creation logic
    - `read_task.rs` - Task reading and listing logic
    - `update_task.rs` - Task update logic
    - `delete_task.rs` - Task deletion logic (bulk delete for DONE tasks)
    - `task.rs` - Task data structure
    - `task_util.rs` - Task utility functions
  - `fileio/` - File I/O operations:
    - `file.rs` - Basic file operations (read, write, delete)
    - `app_data_dir.rs` - Application data directory management

### Application Logic
REMは以下の仕様でTODOタスクを管理する:

1. **タスク管理**: 各タスクは.jsonファイルと1:1で紐づく
2. **ステータス管理**: ディレクトリベースでステータスを管理
   - `todo/` - TODO状態のタスク
   - `doing/` - 実行中のタスク
   - `done/` - 完了したタスク
   - `pending/` - 保留中のタスク
3. **データ構造**:
   - タイトル: .jsonファイル名
   - ステータス: ファイルの配置ディレクトリ
   - 詳細: .jsonファイルの内容
4. **一括削除機能**:
   - DONEレーンのヘッダー右側にゴミ箱アイコンボタンを表示
   - クリックするとDONEステータスの全タスクファイルを削除
   - 削除後は自動的にタスクリストをリロード

### Key Files
- `src-tauri/tauri.conf.json` - Tauri configuration
- `package.json` - Frontend dependencies and scripts
- `src-tauri/Cargo.toml` - Rust dependencies
- `SPEC.md` - Application requirements and class diagram

### Development Notes
- Tauri development server runs on port 1420
- HMR (Hot Module Replacement) uses port 1421  
- Frontend build output goes to `dist/` directory
- Rust backend uses composition over inheritance pattern