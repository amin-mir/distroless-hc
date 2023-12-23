FROM rust:1.74 as builder

COPY . .
RUN cargo build --release

# Copy binary into a distroless container.
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /target/release/hc . 

CMD ["./hc"] 
