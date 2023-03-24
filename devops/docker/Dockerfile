# ---------BUILD-STAGE--------- #
FROM rust:1.67 as BUILDER

# For debian use x86_64-unknown-linux-gnu
ARG LINUX_DISTRO=x86_64-unknown-linux-musl \
    WORKDIR=/usr/src

WORKDIR $WORKDIR

COPY . $WORKDIR

# Compile for alpine
RUN rustup target add $LINUX_DISTRO
RUN cargo build --release --target $LINUX_DISTRO

# ---------PACKAGE-STAGE--------- #
FROM alpine:latest

ARG BIN_PATH=/usr/src/target/x86_64-unknown-linux-musl/release/instagrey \
    INPUT_DIR=/mnt/data \
    OUTPUT_DIR=/tmp/instagrey

RUN mkdir -p $INPUT_DIR $OUTPUT_DIR

COPY --from=BUILDER $BIN_PATH /usr/local/bin

VOLUME [$INPUT_DIR, $OUTPUT_DIR]

CMD ["instagrey"]