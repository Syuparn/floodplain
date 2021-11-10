FROM rust:1 as cargo-build

WORKDIR /usr/src/floodplain

# install rustfmt
RUN rustup component add rustfmt

# install clippy
RUN rustup component add clippy

# add target for cross-compiling
RUN rustup target add x86_64-unknown-linux-musl

# NOTE: build mock app to cache dependencies
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# cross-compiling
RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/floodplain*

# build real apps
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN cargo install --path . --target=x86_64-unknown-linux-musl

# image to deploy
FROM alpine:latest
COPY --from=cargo-build /usr/local/cargo/bin/floodplain /usr/local/bin/floodplain
CMD ["/usr/local/bin/floodplain"]
