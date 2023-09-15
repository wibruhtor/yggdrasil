FROM messense/rust-musl-cross:x86_64-musl as builder

ENV SQLX_OFFLINE=true

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./migrations ./migrations
COPY ./src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

EXPOSE 3000

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/hastur /hastur

CMD ["/hastur"]
