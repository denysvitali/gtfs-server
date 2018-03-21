FROM rustlang/rust:nightly
RUN mkdir /app
RUN git clone https://github.com/denysvitali/gtfs-server.git /app
WORKDIR /app
RUN cargo build ---release
RUN mv target/release/gtfs-server .
RUN cargo clean
ENV IN_DOCKER true
ENTRYPOINT [ "./gtfs-server" ]
