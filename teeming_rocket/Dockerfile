FROM rust:1.40 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

EXPOSE 8000

CMD ["/teeming_rocket"]
