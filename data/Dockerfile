FROM alpine AS build

WORKDIR /app

RUN apk add -U --no-cache \
    curl \
    make \
    sqlite

COPY Makefile import.sql .

RUN make
