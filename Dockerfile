FROM rust:1.47

WORKDIR /usr/src/ukubulala-srv
COPY . .

RUN cargo install --path .

CMD ["ukubulala-srv"]