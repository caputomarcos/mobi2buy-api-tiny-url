FROM mobi2buy/base

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

ADD . /app
WORKDIR /app

RUN cargo build
CMD  ["cargo", "run"]
