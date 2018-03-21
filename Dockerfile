FROM rustlang/rust:nightly
RUN mkdir /app
COPY . /app
WORKDIR /app
RUN cargo build ---release
ENTRYPOINT [ "./target/release/gtfs-server" ]
