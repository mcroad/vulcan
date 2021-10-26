FROM rust:1.55

WORKDIR /usr/src/vulcan
COPY . .

# stm32f411 target
RUN rustup target add thumbv7em-none-eabihf

# cortex-m linker
RUN cargo install flip-link

# cargo-flash dependencies
RUN apt update
RUN apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev
RUN cargo install cargo-flash

# build release version
RUN cargo build --release --bin vulcan

# flash to chip
CMD cargo flash --release --bin vulcan --chip STM32F411CEUx