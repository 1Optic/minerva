FROM harbor.hendrikx-itc.nl/1optic/rust-ci:1.85.1@sha256:15557c8c33bac9e2729e3063db4dde8d113add1c9cd57043b3c5f074d67b65b3 AS build

COPY . /src
WORKDIR /src

RUN cargo build --package minerva-cli --release --target=x86_64-unknown-linux-musl
RUN cargo build --package minerva-service --release --target=x86_64-unknown-linux-musl

FROM scratch

LABEL org.opencontainers.image.source="https://gitlab.1optic.io/hitc/Minerva/minerva"

COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva /usr/bin/
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva-service /usr/bin/

ENTRYPOINT ["/usr/bin/minerva"]
