
FROM rustlang/rust:nightly

WORKDIR /usr/src/ukubulala-srv
COPY . .

RUN cargo install --path .

CMD ["show_posts"]