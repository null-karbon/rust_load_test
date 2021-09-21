FROM rust:1.13-alpine

WORKDIR /test

RUN apk add curl

COPY ./load-test /test/load-test
RUN chmod +x /test/load-test

ENTRYPOINT [ "/test/load-test" ]