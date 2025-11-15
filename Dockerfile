FROM harbor.hendrikx-itc.nl/1optic/rust-ci:1.91.1@sha256:4363248ac984c6a7e9a6f26e902cbfb910240c65f25b59491bb4a30fb42a7df8 AS build

COPY . /src
WORKDIR /src

RUN cargo build --package minerva-cli --release --target=x86_64-unknown-linux-musl
RUN cargo build --package minerva-service --release --target=x86_64-unknown-linux-musl

FROM scratch

LABEL org.opencontainers.image.source="https://gitlab.1optic.io/hitc/Minerva/minerva"

COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva /usr/bin/
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva-service /usr/bin/

ENTRYPOINT ["/usr/bin/minerva"]
