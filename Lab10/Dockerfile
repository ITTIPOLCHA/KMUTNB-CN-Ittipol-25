FROM rust

WORKDIR /app
COPY . .
RUN cargo build --release

EXPOSE 8080

ENTRYPOINT [ "./target/release/web-server" ]