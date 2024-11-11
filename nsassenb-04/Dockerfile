FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update && apt-get install -y m4
RUN cargo build --release

CMD ["cargo", "run", "--bin", "ex07"]
