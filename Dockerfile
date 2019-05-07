FROM rust:slim
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && cargo install --path .
EXPOSE 9001
CMD ["/usr/local/cargo/bin/rust-webservice-helloworld"]