FROM harbor.hendrikx-itc.nl/1optic/rust-ci:1.86.2@sha256:65582634ee8782a9d5cdaeccf043721cf88c35cc41dc42a55d75cdef3ae8b865 AS build

COPY . /src
WORKDIR /src

RUN cargo build --package minerva-cli --release --target=x86_64-unknown-linux-musl
RUN cargo build --package minerva-service --release --target=x86_64-unknown-linux-musl

FROM scratch

LABEL org.opencontainers.image.source="https://gitlab.1optic.io/hitc/Minerva/minerva"

COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva /usr/bin/
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva-service /usr/bin/

ENTRYPOINT ["/usr/bin/minerva"]
