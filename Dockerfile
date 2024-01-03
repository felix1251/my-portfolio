FROM rust:bookworm

WORKDIR /app

COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
RUN cargo build

EXPOSE 8080

CMD [ "trunk", "serve" ]
