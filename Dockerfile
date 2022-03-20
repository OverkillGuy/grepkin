FROM rust:1.59-slim-buster as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock /app/
COPY src/ /app/src/
RUN cargo build --release

FROM gcr.io/distroless/cc as runtime
COPY --from=builder /app/target/release/gherkin_testcomments /
CMD ["./gherkin_testcomments"]
