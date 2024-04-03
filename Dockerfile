FROM rust:latest as build

WORKDIR /usr/src/connect-backend
COPY . .

RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get update -y
RUN apt-get install libpq-dev -y

COPY --from=build /usr/src/connect-backend/target/release/connect-backend /usr/local/bin/connect-backend

WORKDIR /usr/local/bin

CMD ["connect-backend"]