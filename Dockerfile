FROM rust:1.60 AS builder
WORKDIR /usr/src/myapp
ADD . .
RUN cargo build --release -bin controller

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /usr/src/myapp/target/release/controller .
RUN chmod u+x controller
ENTRYPOINT ["./controller"]
