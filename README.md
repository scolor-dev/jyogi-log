# Jyogi-OAuth

## 起動方法
0. 環境変数の設定
1. DBの起動＆マイグレーション
```
docker compose up -d
```
2. APIサーバーの起動
```
cd api
cargo run
```
3. Reactの起動
```
cd web
npm run
```