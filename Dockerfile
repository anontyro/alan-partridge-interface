FROM rustlang/rust:nightly

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8000

ADD . /app

WORKDIR /app

RUN rustup default nightly && rustup update

RUN cargo build --all-features --release

EXPOSE 8000

CMD ["cargo", "run"]