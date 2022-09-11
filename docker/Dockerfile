FROM rust:1.63.0-slim-buster as build
WORKDIR /src
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runtime
WORKDIR /app
EXPOSE 8080
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/docker /app
CMD ["./docker"]