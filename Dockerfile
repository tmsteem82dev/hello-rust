# ----------------------------
# Cargo Build Stage
# ----------------------------

#FROM rustlang/rust:nightly as cargo-build
FROM clux/muslrust as cargo-build

RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install musl-tools libssl-dev -y
RUN rustup default nightly

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/hello-rust

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# ENV OPENSSL_DIR /usr
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/hello-rust*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ----------------------------
# Final Stage
# ----------------------------

FROM alpine:latest

RUN addgroup -g 1000 hello-rust
RUN adduser -D -s /bin/sh -u 1000 -G hello-rust hello-rust

WORKDIR /home/hello-rust/bin/

COPY --from=cargo-build /usr/src/hello-rust/target/x86_64-unknown-linux-musl/release/hello-rust .

RUN chown hello-rust:hello-rust hello-rust

USER hello-rust

CMD ["./hello-rust"]