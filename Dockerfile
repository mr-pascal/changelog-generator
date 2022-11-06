ARG APP="changelog-generator"
ARG ARCH="x86_64-unknown-linux-musl"

################
##### Builder
FROM rust:1.61.0-slim as builder
ARG APP
ARG ARCH

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new $APP

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/$APP/

# Set the working directory
WORKDIR /usr/src/$APP

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add $ARCH

# This is a dummy build to get the dependencies cached.
RUN cargo build --target $ARCH --release

# Now copy in the rest of the sources
COPY src /usr/src/$APP/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/$APP/src/main.rs

# This is the actual application build.
RUN cargo build --target $ARCH --release

################
##### Runtime
FROM alpine:3.16.0 AS runtime 
ARG APP
ARG ARCH

# Copy application binary from builder image
COPY --from=builder /usr/src/$APP/target/$ARCH/release/$APP /usr/local/bin/app


# Run the application
ENTRYPOINT ["/usr/local/bin/app"]


