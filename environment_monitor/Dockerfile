FROM rust:1.72

WORKDIR .
COPY . .
RUN cargo install --path .

CMD ["environment_monitor"]