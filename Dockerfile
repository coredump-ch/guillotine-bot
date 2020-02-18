# Build spaceapi
FROM rust:1.41-slim as builder
WORKDIR /source
COPY . /source
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
RUN cargo build --release && \
    cp target/release/guillotine /usr/local/bin/guillotine && \
    cd / && rm -rf /source

# Create runtime container
# Note that we need a small init process for PID 1 that forwards signals.
# See https://github.com/Yelp/dumb-init
FROM debian:stable-slim
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates dumb-init && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/guillotine /usr/local/bin/
ENV RUST_LOG=warn,guillotine=info

# Entry point
ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["/usr/local/bin/guillotine"]
