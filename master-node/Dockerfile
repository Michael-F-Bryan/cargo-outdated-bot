FROM rust:1.23

RUN apt-get update && apt-get install -y --no-install-recommends cmake

COPY . /code
WORKDIR /code

RUN cargo build --release --bin master-node && \
    cp target/release/master-node /usr/bin/master-node && \
    cargo clean && \
    rm -rf $CARGO_HOME

ENTRYPOINT [ "/usr/bin/master-node" ]
