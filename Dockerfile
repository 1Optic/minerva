FROM scratch

LABEL org.opencontainers.image.source="https://gitlab.1optic.io/hitc/Minerva/minerva"

COPY target/x86_64-unknown-linux-musl/release/minerva-service /usr/bin/

ENTRYPOINT ["/usr/bin/minerva-service"]
