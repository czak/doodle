FROM rust:1.42.0-alpine3.11 as builder
WORKDIR /usr/src/scribble
COPY scribble .
RUN cargo install --path .

FROM alpine:3.11
WORKDIR /checkout
RUN apk add openssh git
RUN mkdir -p ~/.ssh
RUN ssh-keyscan github.com >> ~/.ssh/known_hosts
COPY --from=builder /usr/local/cargo/bin/scribble /usr/local/bin/scribble
COPY scripts/commit.sh /bin/commit.sh
