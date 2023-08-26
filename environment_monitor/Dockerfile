FROM rust:1.72

RUN --mount=type=bind,source=src,target=src \
	--mount=type=bind,source=Cargo.toml,target=Cargo.toml \
	--mount=type=bind,source=Cargo.lock,target=Cargo.lock \
	--mount=type=cache,target=/app/target/ \
	--mount=type=cache,target=/usr/local/cargo/registry/ \
	<<EOF
set -e
cargo build --locked --release
cp ./target/release/environment_monitor /bin/server
EOF

WORKDIR .
COPY . .
RUN cargo install --path .

CMD ["environment_monitor"]