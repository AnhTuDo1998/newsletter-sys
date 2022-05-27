FROM rust:1.59.0

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

# Ensure no error when building
ENV SQLX_OFFLINE true

RUN cargo build --release

# Specify the host address to use in yaml configs
ENV APP_ENVIRONMENT local

ENTRYPOINT ["./target/release/newsletter_sys"]