# `vulcan`

An attempt at an embedded signing device using the [STM32H7](https://stm32-base.org/boards/STM32H743VIT6-STM32H7XX-M) with the OV2640 camera module.

## Get the parts

You can buy the development kit from [Adafruit](https://www.adafruit.com/product/5032)
or [AliExpress](https://www.aliexpress.com/wholesale?SearchText=stm32h750).

Buy the screen from [Waveshare](https://www.waveshare.com/product/displays/lcd-oled/lcd-oled-3/2inch-lcd-module.htm).

Buy the keypad from [Adafruit](https://www.adafruit.com/product/1824) or
[AliExpress](https://www.aliexpress.com/wholesale?SearchText=3x4+keypad).

## Prerequisites
If you're on linux, make sure to install the necessary ST-Link udev rules from [probe.rs](https://probe.rs/docs/getting-started/probe-setup/).

Install [Docker](https://docs.docker.com/engine/install/) and [Docker Compose](https://docs.docker.com/compose/install/).

## Build

The easiest way to build from source is to use [Docker Compose](https://docs.docker.com/compose/install/). Just plug in the
STM32H7 and run the command.

```sh
docker-compose up
```

If you do not wish to use Docker, you can run the commands detailed in the [Dockerfile](./Dockerfile).

## Cleanup

If using `docker-compose`

```sh
docker-compose down
docker image prune -a
```
