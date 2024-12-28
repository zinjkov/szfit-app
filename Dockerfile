FROM ubuntu:latest
LABEL authors="s.zinkov"

WORKDIR /app
RUN apt-get -y update && apt-get install -y libssl-dev sudo openssl
RUN openssl genrsa -out client.key 4096
RUN openssl req -new -x509 -text -key client.key -out client.cert
RUN update-ca-certificates

COPY ./target/release/szfit-app-bin ./szfit-app-bin

ENTRYPOINT ["./szfit-app-bin"]

## Use a Rust base image with Cargo installed
#FROM rust:1.78.0 AS builder
#LABEL authors="s.zinkov"
## Set the working directory inside the container
#WORKDIR /usr/src/app
#
## Now copy the source code
#COPY ./ ./
#ENV SQLX_OFFLINE=true
## Build your application
#RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release
#
## Start a new stage to create a smaller image without unnecessary build dependencies
#FROM ubuntu:latest
#
## Set the working directory
#WORKDIR /usr/src/app
#
## Copy the built binary from the previous stage
#COPY --from=builder /usr/src/app/target/release/szfit-app-bin ./
#
## Command to run the application
#CMD ["./szfit-app-bin"]