FROM rust:1.73-alpine as Builder
RUN apk add --no-cache build-base

WORKDIR /app
COPY rust-toolchain.toml .
RUN rustup component add cargo clippy rust-std rustc rustfmt

FROM builder as DependencyBuilder
WORKDIR /app

RUN mkdir -p crates/common/src crates/director/src crates/distributor/src crates/receiver/src
RUN touch crates/common/src/lib.rs crates/director/src/lib.rs crates/distributor/src/lib.rs crates/receiver/src/lib.rs
RUN touch crates/common/src/main.rs crates/director/src/main.rs crates/distributor/src/main.rs crates/receiver/src/main.rs
RUN echo "fn main() {}" | tee crates/common/src/main.rs crates/director/src/main.rs crates/distributor/src/main.rs crates/receiver/src/main.rs > /dev/null

COPY Cargo.lock .
COPY Cargo.toml .
COPY crates/common/Cargo.toml crates/common/Cargo.toml
COPY crates/director/Cargo.toml crates/director/Cargo.toml
COPY crates/distributor/Cargo.toml crates/distributor/Cargo.toml
COPY crates/receiver/Cargo.toml crates/receiver/Cargo.toml

RUN cargo install --color always --bin saasparilla-notification-receiver --locked --path crates/receiver

FROM builder as ApplicationBuilder
WORKDIR /app

COPY --from=dependencybuilder /app/target target
RUN find target -name '*saasparilla*' -type f -delete
COPY . .

# TODO: add https://github.com/qdrant/qdrant/pull/1856/files as a just script to increase speed of multistage build
# TODO: add https://github.com/qdrant/qdrant/pull/1859/files to further increase speed of multistage builds
#       NOTE: this should only be for PR builds used for unit/integration testing
#             dev/stg/prod builds should be identical release objects
# TODO: add `--out-dir /app/bin` once `--out-dir` is stabilized to reduce size of multistage build
# TODO: could probably do some pre-building based on some copying of `Cargo.*` files
RUN cargo build --future-incompat-report --color always --bin saasparilla-notification-receiver --release --locked

FROM rust:1.73-alpine as Final

COPY --from=applicationbuilder /app/target/release/saasparilla-notification-receiver /usr/local/bin/saasparilla-notification-receiver

CMD ["saasparilla-notification-receiver"]