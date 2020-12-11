FROM rust:alpine

WORKDIR /app

COPY src/ src/
COPY Cargo* ./

RUN cargo build --release

CMD ./target/release/calculator
