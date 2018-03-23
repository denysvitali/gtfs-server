FROM rustlang/rust:nightly
COPY . /app
WORKDIR /app
RUN cargo build ---release
RUN mv target/release/gtfs-server .
RUN cargo clean
ENV IN_DOCKER true
ENTRYPOINT [ "./gtfs-server" ]
