FROM rust:1.23

RUN apt-get update && apt-get install -y --no-install-recommends cmake

RUN cargo install cargo-outdated --version 0.7.0

COPY . /code
WORKDIR /code

RUN cargo install --bin outdated-runner --root /usr/bin/ && \
    cargo clean && \
    rm -rf $CARGO_HOME

ENTRYPOINT [ "/usr/bin/outdated-runner" ]