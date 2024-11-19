# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.78.0
ARG APP_NAME=RfeedMultibot
FROM rust:${RUST_VERSION}-slim-bookworm AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config perl libsqlite3-dev


FROM build AS build-app
WORKDIR /app
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/rfeed-bot
EOF

FROM debian:bookworm-slim AS final-prepare

RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates


FROM final-prepare AS final

# Copy the executable from the "build" stage.
COPY --from=build-app /bin/rfeed-bot /bin/

# What the container should run when it is started.
CMD ["/bin/rfeed-bot"]
