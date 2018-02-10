FROM rust:1.23

RUN apt-get update && apt-get install -y --no-install-recommends cmake

RUN cargo install cargo-outdated --version 0.7.0

COPY . /code
WORKDIR /code

RUN cargo build --release --bin outdated-runner && \
    cp target/release/outdated-runner /usr/bin/outdated-runner && \
    cargo clean && \
    rm -rf $CARGO_HOME

ENTRYPOINT [ "/usr/bin/outdated-runner" ]