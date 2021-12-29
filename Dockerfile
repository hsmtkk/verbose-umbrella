FROM rust:1.57 AS builder
WORKDIR /opt
COPY . .
RUN cargo build --release

FROM ubuntu:20.04 AS runtime
COPY --from=builder /opt/target/release/verbose-umbrella /usr/local/bin/verbose-umbrella
ENTRYPOINT ["/usr/local/bin/verbose-umbrella"]
