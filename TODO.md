# kaggle-mcp-rs TODO リスト

Kaggle MCPのRust実装プロジェクトのTODOリスト。

## 🎯 プロジェクト概要

Kaggle APIをMCP (Model Context Protocol)経由でClaude AIに接続するRustサーバーの実装。全44ツールを提供予定。

## 📋 実装タスク

### 1. プロジェクト基盤 (高優先度)

- [x] **プロジェクト構造の作成**
  - [x] Cargo.tomlの設定 (rmcp依存関係)
  - [x] 基本的なディレクトリ構造の作成
  - [ ] ビルド設定とCI/CDパイプライン

- [x] **データ構造の設計**
  - [x] Kaggle APIレスポンスのRust型定義
  - [x] エラー型の定義
  - [x] 共通ユーティリティの実装

### 2. 認証機能 (高優先度)

- [x] **認証ツール (1ツール)**
  - [x] `authenticate`: Kaggle API認証の実装
  - [x] 認証情報の永続化 (~/.kaggle/kaggle.json)
  - [x] 環境変数サポート

### 3. コンペティション機能 (中優先度)

- [ ] **コンペティションツール (8ツール)**
  - [x] `competitions_list`: コンペティション一覧 ✅ (2025-01-06)
  - [ ] `competition_details`: 詳細情報取得
  - [ ] `competition_download_files`: ファイルダウンロード
  - [ ] `competition_list_files`: ファイル一覧
  - [ ] `competition_submissions`: 提出履歴
  - [ ] `competition_leaderboard`: リーダーボード
  - [ ] `competition_submit`: 新規提出
  - [x] 共通HTTP通信機能の実装 ✅ (KaggleClient内に実装済み)

### 4. データセット機能 (中優先度)

- [ ] **データセットツール (10ツール)**
  - [ ] `datasets_list`: データセット検索
  - [ ] `dataset_list_files`: ファイル一覧
  - [ ] `dataset_download_files`: ファイルダウンロード
  - [ ] `dataset_metadata`: メタデータ取得
  - [ ] `dataset_create_new`: 新規作成
  - [ ] `dataset_create_version`: バージョン作成
  - [ ] `dataset_status`: ステータス確認
  - [ ] `dataset_initialize_metadata`: メタデータ初期化
  - [ ] `dataset_update_metadata`: メタデータ更新
  - [ ] ファイルアップロード機能

### 5. カーネル機能 (中優先度)

- [ ] **カーネルツール (8ツール)**
  - [ ] `kernels_list`: カーネル検索
  - [ ] `kernel_list_files`: ファイル一覧
  - [ ] `kernel_output`: 出力ダウンロード
  - [ ] `kernel_pull`: コード取得
  - [ ] `kernel_status`: 実行ステータス
  - [ ] `kernel_initialize_metadata`: メタデータ初期化
  - [ ] `kernel_push`: カーネルアップロード
  - [ ] ノートブック形式のサポート

### 6. モデル機能 (中優先度)

- [ ] **モデルツール (16ツール)**
  - [ ] `models_list`: モデル一覧
  - [ ] `model_get`: モデル詳細
  - [ ] `model_initialize_metadata`: メタデータ初期化
  - [ ] `model_create_new`: 新規作成
  - [ ] `model_update`: 更新
  - [ ] `model_delete`: 削除
  - [ ] **モデルインスタンス管理 (10ツール)**
    - [ ] インスタンスCRUD操作
    - [ ] バージョン管理
    - [ ] ファイル操作

### 7. 設定機能 (中優先度)

- [ ] **設定ツール (4ツール)**
  - [ ] `config_view`: 設定表示
  - [ ] `config_set`: 設定値の設定
  - [ ] `config_unset`: 設定値のクリア
  - [ ] `config_path`: ダウンロードパス管理

### 8. ドキュメントと例 (低優先度)

- [x] **ドキュメント作成**
  - [x] README.mdの作成
  - [x] APIリファレンス
  - [ ] インストールガイド

- [ ] **サンプルコード**
  - [x] 基本的な使用例
  - [ ] Claude Desktop設定例
  - [x] 各機能の統合テスト

## 🛠️ 技術スタック

- **言語**: Rust
- **MCPフレームワーク**: rmcp (rust-sdk)
- **HTTPクライアント**: reqwest
- **JSONシリアライズ**: serde_json
- **非同期ランタイム**: tokio

## 📦 依存関係

```toml
[dependencies]
rmcp = { git = "https://github.com/modelcontextprotocol/rust-sdk" }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🚀 実装順序

1. **Phase 1**: プロジェクト基盤と認証 (必須)
2. **Phase 2**: コンペティション機能 (基本機能)
3. **Phase 3**: データセット機能 (データ操作)
4. **Phase 4**: カーネル・モデル機能 (高度な機能)
5. **Phase 5**: ドキュメントとリリース準備

## 📝 メモ

- Kaggle APIの仕様: https://github.com/Kaggle/kaggle-api
- Rustのイディオムに従う
- エラーハンドリングは`Result<T, E>`型を適切に使用
- 非同期処理を活用して効率的な実装を目指す
