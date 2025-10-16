FROM alpine:latest

RUN apk add cargo
RUN apk add rust
RUN apk add rustfmt
RUN apk add rust-src
RUN apk add yaml-dev
RUN apk add openssl-dev

RUN mkdir /root/project
WORKDIR /root/project

CMD /bin/sh -c "cargo run > build.txt"
