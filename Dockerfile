# This Dockerfile is for the application, built in the Jenkins pipeline

FROM alpine:3.11

WORKDIR /app

COPY ./target/release/calculator ./exec

CMD ./exec
