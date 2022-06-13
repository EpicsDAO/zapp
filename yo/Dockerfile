FROM rust:1.61 as build
RUN USER=root cargo new --bin yo
WORKDIR /yo
COPY entity entity
COPY migration migration
COPY Cargo.toml Cargo.toml
RUN cargo build --release
COPY ./src ./src
COPY entity/src entity/src
COPY migration/src migration/src
RUN rm -f target/release/deps/yo*
RUN cargo build --release

FROM debian:11.3
COPY --from=build /yo/target/release/yo .

CMD ["./yo"]