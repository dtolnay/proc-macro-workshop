from docker.io/library/rust

RUN apt update && apt upgrade
RUN cargo install cargo-expand
RUN rustup default nightly
