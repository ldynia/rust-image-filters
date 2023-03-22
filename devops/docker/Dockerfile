# ---------COMPILE-STAGE--------- #
FROM rust:1.67 as BUILDER

ARG WORKDIR=/usr/src

WORKDIR $WORKDIR

COPY . $WORKDIR

RUN cargo build --release

# ---------BUILD-STAGE--------- #
FROM debian:bullseye-slim

ARG BIN_PATH=/usr/src/target/release/run \
  INPUT_DIR=/mnt/data \
  OUTPUT_DIR=/tmp/app \

RUN mkdir -p $INPUT_DIR $OUTPUT_DIR

COPY --from=BUILDER $BIN_PATH /usr/local/bin

VOLUME [$INPUT_DIR, $OUTPUT_DIR]

CMD ["run"]