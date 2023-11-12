FROM rust:1.73-alpine3.18 as Builder
RUN apk add --no-cache build-base

WORKDIR /app
COPY rust-toolchain.toml .
RUN rustup component add cargo clippy rust-std rustc rustfmt


FROM builder as DependencyBuilder
WORKDIR /app

ENV CRATES="common director distributor receiver"
RUN printf "${CRATES}" | tr ' ' '\n' | xargs -I{} sh -c 'mkdir -p crates/{}/src && echo "fn main() { }" > crates/{}/src/main.rs && touch crates/{}/src/lib.rs'

# awaiting release of https://github.com/moby/buildkit/pull/3001
#COPY --parents --link ./Cargo.* ./crates/*/Cargo.toml ./
COPY --link Cargo.lock Cargo.toml ./
COPY --link crates/common/Cargo.toml crates/common/Cargo.toml
COPY --link crates/director/Cargo.toml crates/director/Cargo.toml
COPY --link crates/distributor/Cargo.toml crates/distributor/Cargo.toml
COPY --link crates/receiver/Cargo.toml crates/receiver/Cargo.toml

RUN cargo build --color always --release --locked


FROM builder as ApplicationBuilder
WORKDIR /app

COPY --from=dependencybuilder /app/target target
RUN find target -name '*saasaparilla*' -type f -delete
COPY . .
# TODO: add https://github.com/qdrant/qdrant/pull/1856/files as a just script to increase speed of multistage build
# TODO: add https://github.com/qdrant/qdrant/pull/1859/files to further increase speed of multistage builds
#       NOTE: this should only be for PR builds used for unit/integration testing
#             dev/stg/prod builds should be identical release objects
# TODO: add `--out-dir /app/bin` once `--out-dir` is stabilized to reduce size of multistage build
ARG COMPONENT
RUN cargo build --future-incompat-report --color "always" --bin "saasaparilla-notification-${COMPONENT}" --release --locked


# https://hub.docker.com/_/rust
FROM alpine:3.18 as Final
WORKDIR /app

COPY --link LICENSE /app/LICENSE

ARG COMPONENT
COPY --from=applicationbuilder --link /app/target/release/saasaparilla-notification-${COMPONENT} /app/bin
COPY --link crates/${COMPONENT}/config.toml /app/config.toml
CMD ["./bin", "config-file-path=config.toml"]