FROM harbor.hendrikx-itc.nl/1optic/rust-ci:1.94.2@sha256:720fd20d0093d964ab7bbd0d16bfdf4cd486278f07cbe4bbde813c8946fbc0a9 AS build

COPY . /src
WORKDIR /src

RUN cargo build --package minerva-cli --release --target=x86_64-unknown-linux-musl
RUN cargo build --package minerva-service --release --target=x86_64-unknown-linux-musl

FROM scratch

LABEL org.opencontainers.image.source="https://gitlab.1optic.io/hitc/Minerva/minerva"

COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva /usr/bin/
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva-service /usr/bin/

ENTRYPOINT ["/usr/bin/minerva"]
