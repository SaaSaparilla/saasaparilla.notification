FROM rust:1.76-alpine AS builder
RUN apk add --no-cache build-base perl

WORKDIR /app
COPY rust-toolchain.toml .
RUN rustup component add clippy rustfmt


FROM builder AS dependency_builder
WORKDIR /app

ENV CRATES="common director distributor receiver"
RUN printf "${CRATES}" | tr ' ' '\n' | xargs -I{} sh -c 'mkdir -p crates/{}/src && echo "fn main() { }" > crates/{}/src/main.rs && touch crates/{}/src/lib.rs'

# awaiting release of https://github.com/moby/buildkit/pull/3001
#COPY --parents --link ./Cargo.* ./crates/*/Cargo.toml ./
COPY --link "Cargo.lock" "Cargo.toml" ./
COPY --link "crates/common/Cargo.toml" crates/common/Cargo.toml
COPY --link "crates/director/Cargo.toml" crates/director/Cargo.toml
COPY --link "crates/distributor/Cargo.toml" crates/distributor/Cargo.toml
COPY --link "crates/receiver/Cargo.toml" crates/receiver/Cargo.toml

RUN cargo build --color "auto" --release --locked


FROM builder AS application_builder
WORKDIR /app

COPY --from=dependency_builder /app/target target
RUN find target -name '*saasaparilla*' -type f -delete
COPY . .
# TODO: add https://github.com/qdrant/qdrant/pull/1856/files as a just script to increase speed of multistage build
# TODO: add https://github.com/qdrant/qdrant/pull/1859/files to further increase speed of multistage builds
#       NOTE: this should only be for PR builds used for unit/integration testing
#             dev/stg/prod builds should be identical release objects
# TODO: add `--out-dir /app/bin` once `--out-dir` is stabilized to reduce size of multistage build
ARG COMPONENT
ENV COMPONENT_BIN="saasaparilla-notification-${COMPONENT}"
RUN cargo build --future-incompat-report --color "auto" --bin "${COMPONENT_BIN}" --release --locked


# https://hub.docker.com/_/rust
FROM alpine:3.19 AS final
WORKDIR /app

COPY --link LICENSE /app/LICENSE

ARG COMPONENT
ENV COMPONENT_BIN="saasaparilla-notification-${COMPONENT}"
COPY --from=application_builder --link "/app/target/release/${COMPONENT_BIN}" "/app/bin"
COPY --link "crates/${COMPONENT}/config.toml" "/app/config.toml"
CMD ["/app/bin", "config-file-path=config.toml"]


FROM scratch AS minimal
WORKDIR /app

# generate the list of required libs with `ldd /app/bin`
COPY --from=final --link "/lib/ld-musl-x86_64.so.1" /lib/

COPY --from=final --link "/app/" /app/
CMD ["/app/bin", "config-file-path=config.toml"]
