FROM rust:1.68.0 as builder
WORKDIR /usr/src/server
COPY . .
RUN cargo install --path .

FROM debian:bullseye-20230227-slim
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/src/server/.env.mongo /.env
CMD ["server"]