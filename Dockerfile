FROM rust:latest AS builder
RUN apt-get update -y && apt-get install -y clang gcc g++ cmake tesseract-ocr libtesseract-dev
WORKDIR /app
COPY . .
RUN git submodule update --init --recursive
RUN rm -rf build
RUN cargo build -r --bin artifacter
RUN mkdir -p build/assets
RUN cp target/release/artifacter build/art
RUN cp -r assets/trained build/assets/trained
FROM ubuntu:latest as img
WORKDIR /app
COPY --from=builder /app/build/* .