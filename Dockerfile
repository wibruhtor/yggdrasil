FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && \
  apt-get install -y ca-certificates
COPY --from=planner /app/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Set sqlx to offline
ENV SQLX_OFFLINE=true
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /yggdrasil
EXPOSE 8000
ENTRYPOINT ["/yggdrasil"]
