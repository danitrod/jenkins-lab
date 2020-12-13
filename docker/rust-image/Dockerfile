FROM rust:alpine

RUN apk add docker
RUN apk add openrc
RUN addgroup root docker
RUN rc-update add docker boot
