FROM rust:latest

WORKDIR /app
COPY ./target/release/hackerman /app/hackerman
COPY ./cmdmap.json ./cmdmap.json
RUN cp /app/cmdmap.json /cmdmap.json

COPY ./docker-entrypoint.sh ./docker-entrypoint.sh

CMD ["/app/docker-entrypoint.sh"]
