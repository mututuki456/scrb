# Scrapbox Scrabbler - scrb

Scrapbox のページを取得・閲覧するCLIツール

## Install

```
cargo build --release
```

`target/release/scrb.exe` を任意のディレクトリに配置してください。

## Setting

`scrb.exe` と同じディレクトリに `scrb-config.toml` を作成します。

```toml
sid = "your-connect.sid-value"
```

`sid` はブラウザの <F12>DevTools → Application → Cookies → `scrapbox.io` の `connect.sid` の値です。
パブリックプロジェクトのみで `scrb` を使用する場合は不要です。

## Commands

### `md` — MarkdownとしてページをSTDOUTに出力

```
scrb md <project> <page>
```

ページ名は部分一致で検索できます。
候補が複数ある場合は選択肢が表示されるので自由に選択できます。

### `raw` — Scrapbox記法のままSTDOUTに出力

```
scrb raw <project> <page>
```

### `ls` — ページ一覧を表示

```
scrb ls <project> [keyword]
```

キーワードを指定するとページ名（部分一致）で絞り込みます。

## Usage

```
scrb raw <YOUR_PROJECT_NAME> <PAGE_TITLE> > pagetitle.txt
```

## Cache

取得したページ一覧の情報（`ls` および `md`/`raw` のページ名補完）は `cache/<project>/titles.json` にキャッシュされます。
更新は呼び出す際に1日おきで行われます。
（キャッシュの手動更新は未実装）

## 注意事項

- 本ツールは非公式のScrapbox APIを使用しています。
  APIの仕様変更により予告なく動作しなくなる可能性があります。
- 使用前にソースコードを確認の上、自己責任でご利用ください。
- 本ツールの使用によって生じたいかなる損害についても、作者は責任を負いません。

