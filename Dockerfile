FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true

# RUN apt update
# RUN apt install -y pkg-config
# RUN apt install -y libssl-dev

ENV SQLX_OFFLINE=true

RUN cargo install cargo-chef
WORKDIR /job_spot_backend 

FROM chef AS planner

# Copy source code from previous stage
COPY . .

# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /job_spot_backend/recipe.json recipe.json

# # Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Copy source code from previous stage
COPY . .

# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /job_spot_backend/target/x86_64-unknown-linux-musl/release/job_spot_backend /job_spot_backend
ENTRYPOINT ["/job_spot_backend"]
EXPOSE 3000
