FROM rust:latest

WORKDIR /usr/src/oxidize
COPY ./src ./src
COPY ./i8n ./i8n
COPY ./tests ./tests
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY .env.dist .env
RUN cargo build
# Keep the container running by starting a shell
CMD ["sleep", "infinity"]