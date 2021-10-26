# `vulcan`

an embedded signing device.

First attempt using the STM32F411 with the ArduCAM SPI OV5642.

### Prerequisites
If you're on linux, make sure to install the necessary ST-Link udev rules from [probe.rs](https://probe.rs/docs/getting-started/probe-setup/).

## Build

Build and flash with `docker-compose`

```sh
docker-compose up
```

To use without `docker-compose`

```sh
docker build --tag vulcan_image .
docker run --privileged --volume /dev:/dev --interactive --tty --rm vulcan_image
```

## Cleanup

If using `docker-compose`

```sh
docker-compose down
docker image prune -a
```

If not using `docker-compose`

```sh
docker image prune -a
```

## VS Code

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  
If you're not using VS Code, you can safely delete the directory from the generated project.
