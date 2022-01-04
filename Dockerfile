FROM rust:1 as cargo-build

WORKDIR /usr/src/floodplain

# install rustfmt
RUN rustup component add rustfmt

# install clippy
RUN rustup component add clippy

# NOTE: build mock app to cache dependencies
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# cross-compiling
RUN cargo build --release
# build real apps
COPY . .
RUN cargo build --release
RUN cargo install --path .

# image to deploy
# NOTE: use alpine once openssl can be cross-compiled
FROM ubuntu:latest

RUN apt-get update -y && apt-get install -y postgresql-client

COPY --from=cargo-build /usr/local/cargo/bin/floodplain /usr/local/bin/floodplain
CMD ["/usr/local/bin/floodplain"]
