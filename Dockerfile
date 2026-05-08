FROM rust:trixie AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY src ./src

RUN cargo build --release

FROM gcr.io/distroless/cc-debian13

WORKDIR /app

COPY --from=builder /app/target/release/dat-bank /app/dat-bank

ENV PORT=80

EXPOSE 80
ENTRYPOINT ["/app/dat-bank"]
