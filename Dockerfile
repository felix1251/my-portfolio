FROM rust:bookworm

WORKDIR /app

COPY . .
RUN cargo install trunk wasm-bindgen-cli
RUN cargo install --path .

EXPOSE 8080

CMD [ "trunk", "serve" ]
