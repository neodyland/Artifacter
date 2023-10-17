FROM debian:latest AS builder
COPY .out /out
RUN if [ "$(arch)" = "x86_64" ]; then \
        cp -r /out/x86-64 /build; \
    else \
        cp -r /out/aarch64 /build; \
    fi
RUN rm -rf /out
FROM googlefan25/tiny-rust:latest
COPY --from=builder /build/artifacter /art
CMD ["/art"]
