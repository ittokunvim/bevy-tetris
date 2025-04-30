# bevy-tetris

`bevy-tetris`は、ゲームエンジンBevyで作られたテトリスです。

## 始める

以下のコードを実行することで遊ぶことができます。

```sh
cargo run
```

## Wasmに変換する

ゲームをWasmに変換する場合は、以下のコマンドを実行します。

```sh
# ビルド
cargo build --release --target wasm32-unknown-unknown
# 変換
wasm-bindgen --target web --out-dir ./examples --no-typescript \
target/wasm32-unknown-unknown/release/ittoku_tetris.wasm
```

