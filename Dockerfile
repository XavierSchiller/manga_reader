FROM rust:alpine

WORKDIR /project

COPY . .

RUN cargo build

CMD ["cargo" "run"]