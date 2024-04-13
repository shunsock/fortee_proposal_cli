# Fortee Proposal CLI
## Overview
[Fortee](https://fortee.jp)に投稿されたプロポーザルデータをのURLを使ってダウンロード+表示するためのコマンドラインツールです．
URLを指定すると，プロポーザルデータをダウンロードします．以下のように動作します．

![command example](command_example.png)

ダウンロードされるプロポーザルデータは以下のようなJSON形式です．

```json
{
  "title": "Readable 正規表現",
  "schedule": "2024/03/07 17:30〜",
  "track": "Track A",
  "speaker": "shunsock",
  "proposal_page_url": "https://fortee.jp/phperkaigi-2024/proposal/eff9589b-b603-4b23-aa35-42948443a80b",
  "og_image_url": "https://fortee.jp/phperkaigi-2024/proposal/og-image/eff9589b-b603-4b23-aa35-42948443a80b.png"
}
```

また，OGイメージもダウンロードされます．保存されたプロポーザルのJSONデータやOGイメージは以下の場所に保存されます．
```bash
$HOME/.fortee/html/proposal.json
$HOME/.fortee/image/og_image.{file_extension}
```

以下のようにコマンドラインで表示されるので，ここをコピー&ペーストすると楽に取得できます．

```bash
[notice] you can get data by running: cp /Users/shunsuke.tsuchiya/.fortee/json/proposal.json path/your/directory
[notice] you can get data by running: cp /Users/shunsuke.tsuchiya/.fortee/image/og_image.png path/your/directory
```

## Warning
このツールは，fortee公式にはサポートされていません．自己責任で使用してください．
また，これはテストバージョンです．正常に動作しない可能性があります．

## Requirements
以下のソフトウェアのインストールを前提としています．
- `Git`
- `Make`

以下のソフトウェアは開発者向けのものです．
- `Cargo`

## Getting Started
fortee community cliのインストールはマニュアルで行う必要があります．

```bash
git clone https://github.com/shunsock/fortee_cli_test.git 
cd fortee-proposal-downloader
make install
fortee-cli https://fortee.jp/phperkaigi-2024/proposal/eff9589b-b603-4b23-aa35-42948443a80b
```

## Usage
### Download
```bash
git clone https://github.com/shunsock/fortee_cli_test.git
```

### Install & Uninstall
```bash
make install
make uninstall
```

### Help
```bash
fortee-cli --help
```

### Download proposal data
```bash
fortee-cli -u {your_proposal_url_page}
```

### (For Developer) Run All Rust Pre-Commit Checks
```bash
make watcher
```

### (For Developer) Install Rust Pre-Commit Components (rustfmt, clippy, etc.)
```bash
make install-dev
```

### (For Developer) Build Release Binary and Copy to `bin` Directory
```bash
make publish
```
