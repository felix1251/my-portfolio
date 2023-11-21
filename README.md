# PORTFOLIO

**_Blazingly fast porfolio site_**

[Install Rust in your machine](https://www.rust-lang.org/tools/install)

Install WebAssembly target

```rust
rustup target add wasm32-unknown-unknown
```

Install Trunk to run local

```
cargo install trunk wasm-bindgen-cli
```

Build Yew

```
cargo build
```

Build tailwindcss

```
npx tailwindcss -o ./tailwind.css
```

Build and Watch tailwindcss

```
npx tailwindcss -o ./tailwind.css --watch
```

Run locally

```
trunk serve
```
