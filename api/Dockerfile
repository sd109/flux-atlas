ARG WORKDIR=/usr/src/flux-atlas-backend

FROM rust:1.75 AS builder

ARG WORKDIR

WORKDIR ${WORKDIR}

COPY Cargo.* .
COPY src/ src/

RUN cargo install --path . --locked

FROM ubuntu:22.04

ARG WORKDIR

# RUN apt-get update && apt-get install -y libc6-dev && rm -rf /var/lib/apt/lists/*

# Install Helm or other CLI tools here?

COPY --from=builder /usr/local/cargo/bin/flux-atlas-backend /usr/local/bin/flux-atlas-backend
COPY --from=builder ${WORKDIR}/target/release/flux-atlas-backend /usr/local/bin/flux-atlas-backend

CMD ["flux-atlas-backend"]
