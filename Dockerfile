FROM debian:latest AS builder
RUN apt-get update -y && apt-get install -y cmake tesseract-ocr libtesseract-dev clang gcc g++
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN source $HOME/.cargo/env
WORKDIR /build
COPY . .
RUN git submodule update --init --recursive
RUN rm -rf build
RUN cargo build -r --bin artifacter
RUN mkdir -p build/assets
RUN cp target/release/artifacter build/art
RUN cp -r assets/trained build/assets/trained
FROM debian:latest AS runtime
WORKDIR /app
COPY --from=builder /build/build .
RUN apt-get update -y && apt-get install -y tesseract-ocr libtesseract-dev libssl-dev
CMD ["./art"]
