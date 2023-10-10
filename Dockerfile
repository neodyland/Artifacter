FROM rust:latest
COPY .out /out
RUN dpkgArch="$(dpkg --print-architecture)"; \
        case "$apkArch" in \
            amd64) cp /out/x86-64/artifacter /art ;; \
            arm64) cp /out/aarch64/artifacter /art ;; \
        esac; \
RUN rm -rf /out
CMD ["./art"]
