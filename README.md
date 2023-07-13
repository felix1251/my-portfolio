# PORTFOLIO UI

**_Blazingly fast porfolio site_**

## CSR (Client Side Rendering)

_Navigate to csr folder_

Install WebAssembly target

```rust
rustup target add wasm32-unknown-unknown
```

Build Yew

```rust
cargo build
```

Build tailwindcss

```rust
npx tailwindcss -o ./tailwind.css
```

Build and Watch tailwindcss

```rust
npx tailwindcss -o ./tailwind.css --watch
```

Run locally

```rust
trunk serve
```

## SSR (Server Side Rendering)

_Navigate to ssr folder_

Install cargo watcher

```rust
cargo install cargo-watch
```

Run with cargo watcher

```rust
cargo watch -x run
```

Or run without watcher

```rust
cargo r -r
```
