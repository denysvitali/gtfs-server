FROM rustlang/rust:nightly
RUN mkdir /app
COPY . /app
WORKDIR /app
RUN cargo build ---release
ENV IN_DOCKER true
ENTRYPOINT [ "./target/release/gtfs-server" ]
