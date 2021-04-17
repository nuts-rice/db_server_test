FROM rustlang/rust:nightly as builder

RUN USER=root cargo new --bin db_server_test_rc
WORKDIR ./db_server_test_rc
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
RUN rm ./target/debug/deps/db_server_test_rc*
RUN cargo build

FROM buildpack-deps:stretch

COPY --from=builder /db_server_test_rc/target/debug/db_server_test_rc /app/

ENTRYPOINT ["/app/db_server_test_rc"]
