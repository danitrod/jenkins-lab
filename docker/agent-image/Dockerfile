# This Dockerfile is for the Jenkins pipeline agent, it has Docker and Rust installed.

FROM rust:alpine3.11

RUN apk add docker
RUN apk add openrc
RUN addgroup root docker
RUN rc-update add docker boot
