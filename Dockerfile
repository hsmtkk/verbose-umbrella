FROM rust:1.57 AS builder
WORKDIR /opt
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11 AS runtime
COPY --from=builder /opt/target/release/verbose-umbrella /usr/local/bin/verbose-umbrella
ENTRYPOINT ["/usr/local/bin/verbose-umbrella"]
