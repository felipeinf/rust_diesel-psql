FROM rust:1.67

RUN apt-get update && apt-get install -y \
    postgresql \
    libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/app
COPY . .

ENV DATABASE_URL=postgres://postgres:PGPASS@postgres:5432/postgres

RUN cargo install --path . && \
    cargo install cargo-watch
    
CMD cargo watch -x 'run'