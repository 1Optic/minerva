FROM harbor.hendrikx-itc.nl/1optic/rust-ci:1.89.1@sha256:afa11531cec36a701d61ede01cf2499c020b8184d5c26bb5842d17c6dff15aa0 AS build

COPY . /src
WORKDIR /src

RUN cargo build --package minerva-cli --release --target=x86_64-unknown-linux-musl
RUN cargo build --package minerva-service --release --target=x86_64-unknown-linux-musl

FROM scratch

LABEL org.opencontainers.image.source="https://gitlab.1optic.io/hitc/Minerva/minerva"

COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva /usr/bin/
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/minerva-service /usr/bin/

ENTRYPOINT ["/usr/bin/minerva"]
