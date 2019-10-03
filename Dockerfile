# Based on https://github.com/rust-lang-nursery/docker-rust/blob/f18cebe3699016a654da86212fa90afaea7431a1/1.25.0/jessie/Dockerfile
FROM ubuntu:bionic
RUN apt-get update && apt-get install -y curl bash gcc libssl-dev pkg-config git
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl -o /tmp/rustup.sh https://sh.rustup.rs && chmod u+x /tmp/rustup.sh && /tmp/rustup.sh -y --default-toolchain nightly

COPY . /app
WORKDIR /app
RUN cargo build ---release
RUN mv target/release/gtfs-server .
RUN cargo clean
ENV IN_DOCKER true
ENTRYPOINT [ "./gtfs-server" ]
