# Cache Data Operation Tool

Redis/Valkeyキャッシュサーバのデータ操作（CRUD）を簡単に行うためのCLIツールです。

## 機能

- ✅ CLIインタラクティブメニュー
- ✅ キー名による検索（部分一致）
- ✅ キー・バリューの取得/設定
- ✅ Redis/Valkey対応

## 必要環境

- Rust 1.70以上
- Redis または Valkey サーバー

## インストール・実行

```bash
# リポジトリをクローン
git clone <repository-url>
cd cache-data-operation

# 依存関係のインストールとビルド
cargo build

# 実行
cargo run
```

## 使い方

### 1. サーバー接続

実行すると、まず接続先URLの入力を求められます：

```
Cache Data Operation Tool
=========================
Redis/Valkey URL (default: redis://127.0.0.1/): 
```

- Enterのみ: デフォルトの `redis://127.0.0.1/` に接続
- カスタムURL: `redis://localhost:6379/1` など任意のURLを入力

### 2. メニュー操作

接続が成功すると、以下のメニューが表示されます：

```
--- Menu ---
1. Search keys
2. Get value by key
3. Set key-value
4. Exit
Select option: 
```

### 3. 各機能の使い方

#### キー検索（1番）
キー名の一部を入力して、該当するキーを検索します。

```
Select option: 1
Enter search pattern: user
Found 3 keys:
  - user:123
  - user:456  
  - user_settings
```

#### 値の取得（2番）
指定したキーの値を取得します。

```
Select option: 2
Enter key name: user:123
Value: {"name": "John", "age": 30}
```

#### 値の設定（3番）
新しいキー・バリューのペアを設定します。

```
Select option: 3
Enter key name: user:789
Enter value: {"name": "Alice", "age": 25}
Successfully set 'user:789' = '{"name": "Alice", "age": 25}'
```

#### 終了（4番）
アプリケーションを終了します。

## 技術仕様

### 使用ライブラリ

- `redis`: Redis/Valkey接続
- `tokio`: 非同期ランタイム
- `anyhow`: エラーハンドリング
- `clap`: CLI引数解析（将来の拡張用）

### 対応データ型

- 文字列（String）
- JSON文字列
- その他のRedis文字列型データ

### 検索仕様

- パターン検索: `*{入力文字列}*` 形式のワイルドカード検索
- 大文字小文字区別あり
- 部分一致対応

## ライセンス

MIT License
