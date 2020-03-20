FROM golang:1.14.1-alpine3.11 as builder
WORKDIR /usr/src/tools
COPY tools-go .
RUN go install ./...

FROM alpine:3.11
WORKDIR /checkout
RUN apk add openssh git
RUN mkdir -p ~/.ssh
RUN ssh-keyscan github.com >> ~/.ssh/known_hosts
COPY --from=builder /go/bin/scribble /usr/local/bin/scribble
COPY --from=builder /go/bin/estimate /usr/local/bin/estimate
COPY scripts/commit.sh /usr/local/bin/commit.sh
