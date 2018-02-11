FROM rust:1.23

RUN apt-get update && apt-get install -y --no-install-recommends cmake

COPY . /code
WORKDIR /code

RUN cargo install --bin master-node --root /usr/bin/ && \
    cargo clean && \
    rm -rf $CARGO_HOME

ENTRYPOINT [ "/usr/bin/master-node" ]
