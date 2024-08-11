FROM rust:alpine
WORKDIR /axum_example

RUN apk add --no-cache musl-dev

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=.env,target=.env\
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
EOF

CMD ["/axum_example/target/release/axum_example"]
EXPOSE 3000