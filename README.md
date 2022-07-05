# Rust Wasm Sample

## Install

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack
```

## 実行

```sh
wasm-pack build --target web
python3 -m http.server 8080
```

http://localhost:8080/hello.html

## 参考

- https://dev.classmethod.jp/articles/rust-webassembly-javascript/
- https://rheotommy.hatenablog.com/entry/2020/07/18/205343
