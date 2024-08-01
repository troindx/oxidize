FROM rust:latest as builder

WORKDIR /usr/src/oxidize
COPY ./src .
COPY ./i8n .
COPY ./tests .
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY .env.dist .env
RUN cargo build
# Keep the container running by starting a shell
CMD ["sleep", "infinity"]