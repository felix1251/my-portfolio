# portfolio-ui

**_Blazingly fast porfolio site_**

## CSR

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

## SSR

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
