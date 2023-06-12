FROM rust:latest AS builder
RUN apt-get update -y && apt-get install -y cmake tesseract-ocr libtesseract-dev clang gcc g++
WORKDIR /build
COPY . .
RUN git submodule update --init --recursive
RUN rm -rf build
RUN cargo build -r --bin artifacter
RUN mkdir -p build/assets
RUN cp target/release/artifacter build/art
RUN cp -r assets/trained build/assets/trained
FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /build/build .
CMD ["./art"]