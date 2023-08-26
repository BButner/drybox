FROM rust:alpine3.18 as builder

WORKDIR .
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/environment_monitor /usr/local/bin/environment_monitor
CMD ["environment_monitor"]