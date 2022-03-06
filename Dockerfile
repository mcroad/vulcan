# install rust
# https://rustup.rs/
FROM rustlang/rust:nightly

WORKDIR /usr/src/vulcan
COPY . .

# add the compilation target
RUN rustup target add thumbv7em-none-eabihf

# install the cortex-m linker
RUN cargo install flip-link

# install cargo-flash dependencies
RUN apt update
RUN apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev

# install cargo-flash
RUN cargo install cargo-flash

# build release version
RUN cargo build -p vulcan --release --target thumbv7em-none-eabihf

# flash release to chip
CMD cargo flash -p vulcan --release --target thumbv7em-none-eabihf --chip STM32H743VITx