FROM rust:latest

WORKDIR /app
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./cmdmap.json ./cmdmap.json

# Install
RUN cargo install --path .
RUN cp /app/cmdmap.json /cmdmap.json

COPY ./docker-entrypoint.sh ./docker-entrypoint.sh

CMD ["/app/docker-entrypoint.sh"]
