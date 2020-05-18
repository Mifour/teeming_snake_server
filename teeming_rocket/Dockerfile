FROM rust:1.40 as builder
WORKDIR /usr/src/teeming_rocket
COPY . .
RUN rustup update nightly && rustup default nightly
RUN cargo install --path .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/teeming_rocket /usr/local/bin/teeming_rocket

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
EXPOSE 8080
CMD ["teeming_rocket"]

# docker run --detach --publish 8000:8080 <image tag>
# this cmd use the optimazed compiled application, it should be available right away.
