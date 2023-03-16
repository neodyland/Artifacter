# 何ができるの?
画像生成部分をAPIとして使用できます
# インストール方法
以下のコマンドを実行します
```sh
git clone https://github.com/neodyland/Artifacter
cd Artifacter
cargo build -r --bin api
```
実行後に生成される`target/release/api`を実行することで、apiサーバーが動作します。  
(環境変数のPORTを指定することでポートを変更可能)  
生成されたバイナリには依存関係が存在しないため、独立して動かすことが可能です。
# apiの使い方
PATH: `/v0/generate`
METHOD: `GET`
PARAMS: 
- `uid` UserId Required
- `cid` CharacterId Required
- `counter` Normal | Hp | Def | ElementalMastery | ChargeEfficiency Optional Default:Normal
- `lang` Ja | En Optional Default:Ja
- `format` Png | Jpeg | Pixel Default:Png
RETURNS:
- `400` String リクエストの形式が正しくない場合
- `404` String ユーザーデータまたはキャラクターが見つからなかった場合、enka.networkがタイムアウトした場合
- `500` String 画像生成に失敗した場合
- `200` Buffer 成功した場合
# リクエスト例
`/v0/generate?uid=827106332&cid=10000069&counter=ElementalMastery`  
`/v0/generate?uid=827106332&cid=10000069&counter=ElementalMastery&lang=En`

# ビルド中のエラーについて
もしも、ビルド中にエラーが出た場合は以下のようにビルド用ライブラリをインストールしてください。
```sh
sudo apt update
sudo apt install gcc clang g++ c++ pkg-config cmake libfontconfig-dev -y
```
これでも解消されない場合は[公式サイト](https://artifacter.neody.land/)よりDiscordサーバーに参加し、お問い合わせください。

# 対応OS
動作確認済みのOS一覧です
- Windows11(x86-64)
- MacOS13(x86-64)
- Ubuntu20.08(arm64)
- Ubuntu20.08(x86-64)