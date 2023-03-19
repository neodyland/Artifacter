FROM rust:latest AS builder
RUN apt-get update -y && apt-get install -y clang gcc g++ make cmake c++ tesseract-ocr libtesseract-dev
COPY . /app
WORKDIR /app
RUN sh build.sh
FROM ubuntu:latest AS runner
COPY --from=builder /app/build /app
WORKDIR /app
CMD ["./art"]