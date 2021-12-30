FROM clux/muslrust:1.57.0 AS builder
WORKDIR /opt
COPY . .
RUN cargo build --release

FROM alpine:3.15 AS runtime
COPY --from=builder /opt/target/x86_64-unknown-linux-musl/release/verbose-umbrella /usr/local/bin/verbose-umbrella
ENTRYPOINT ["/usr/local/bin/verbose-umbrella"]
