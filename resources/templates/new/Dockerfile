FROM rust:1.64 as builder

# create a new empty bin project such that caching dependencies works
RUN cargo new --bin /usr/src/{{ app_name }}

WORKDIR /usr/src/{{ app_name }}

# cache dependencies
COPY entity entity
COPY migration migration
COPY Cargo.toml Cargo.toml
RUN cargo build --release

# copy source code
COPY src src

# release build
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get upgrade \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/{{ app_name }}/target/release/{{ app_name }} .

CMD ["./{{ app_name }}"]
