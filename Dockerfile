FROM rust:1.72 as builder

ENV SQLX_OFFLINE=true

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./.sqlx ./.sqlx

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM scratch

EXPOSE 3000

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/yggdrasil /yggdrasil

CMD ["/yggdrasil"]
