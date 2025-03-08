# lipong

[![crates.io](https://img.shields.io/crates/v/lisnake)](https://crates.io/crates/lisnake)
[![Build](https://github.com/fwcd/lisnake/actions/workflows/build.yml/badge.svg)](https://github.com/fwcd/lisnake/actions/workflows/build.yml)

Pong for Project Lighthouse.

## Building

To build the project, run

```sh
cargo build
```

## Running

To run the project, make sure to have the following environment variables set or in a `.env` file in the working directory:

```
LIGHTHOUSE_USER=<your user>
LIGHTHOUSE_TOKEN=<your token>
```

Then run

```sh
cargo run
```
