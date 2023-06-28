FROM rust:1.70 as builder

RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev

WORKDIR /appbuild/

COPY Cargo.toml build-docker.rs ./
COPY src/ src/
COPY proto/ proto/
RUN cp build-docker.rs build.rs

# RUN cargo build --release
RUN cargo build

FROM ubuntu:22.04

RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev

COPY --from=builder /appbuild/target/debug/server /app/server
# COPY --from=builder /appbuild/target/release/server /app/server
COPY config-docker.toml ./

RUN cp config-docker.toml config.toml
CMD ["/app/server"]