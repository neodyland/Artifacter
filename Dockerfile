FROM googlefan25/tiny-rust:latest
COPY .out /out
RUN if [ "$(arch)" = "x86_64" ]; then \
        cp -r /out/x86-64/artifacter /art; \
    else \
        cp -r /out/aarch64/artifacter /art; \
    fi
RUN rm -rf /out
CMD ["/art"]
