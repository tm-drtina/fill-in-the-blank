# Build server
FROM rust:1.61 as build-server
WORKDIR /app

COPY ./server /app
RUN cargo build --release --locked


# Build client
FROM node:18 as build-client
WORKDIR /client

COPY ./client/package.json ./client/yarn.lock /client/
RUN yarn install --frozen-lockfile

COPY ./client/ /client/
RUN yarn build


# Final release
FROM gcr.io/distroless/cc
WORKDIR /app

ENV PORT=80
ENV LISTEN_ADDR=0.0.0.0

COPY --from=build-server /app/target/release/fitb /app/
COPY --from=build-client /client/build/ /app/static/

CMD ["/app/fitb"]
