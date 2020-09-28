# `DOCKER_BUILDKIT=1 docker build  .`

#------------------------------------------------------------------------------
# Build Stage
#------------------------------------------------------------------------------

FROM ekidd/rust-musl-builder:latest AS builder
WORKDIR ./

# Download and compile dependencies
RUN USER=root cargo new --bin rustfm-scraper
WORKDIR ./rustfm-scraper
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Build the exe using actual source code
COPY src ./src
RUN cargo build --release

#------------------------------------------------------------------------------
# Final Stage
#------------------------------------------------------------------------------

FROM alpine:latest

# Install ca-certificates for openssl
RUN apk --no-cache add ca-certificates

#COPY --from=builder /usr/local/cargo/bin/rustfm-scraper .
COPY --from=builder \
    /home/rust/src/rustfm-scraper/target/x86_64-unknown-linux-musl/release/rustfm-scraper \
    /usr/local/bin

USER 1000
CMD /usr/local/bin/rustfm-scraper