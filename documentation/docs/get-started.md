---
title: Get Started
---

# Get Started

DB Overflow is designed as a command line interface. It's as easy as downloading the prebuilt binary and run it from a terminal.

## Download

Binaries can be downloaded from the [Releases](https://github.com/hendric-dev/db-overflow/releases) page.

An example using curl:

```sh
platform=linux-amd64
version=0.1.0
curl -L https://github.com/hendric-dev/db-overflow/releases/download/$version/db-overflow-$platform -o db-overflow
```

Make it executable and it is ready to be used:

```sh
chmod a+x db-overflow
```

## Usage

The tool is self documenting, just run it with the `--help` flag to get an overview:

```sh
$ db-overflow --help

DB Overflow - Insert large amounts of data into a Postgres DB

Usage: db-overflow <COMMAND>

Commands:
  config  Creates a config file from a DB table for more fine grained customizations
  run     Generate data and fill your DB table
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Options can either be passed as CLI arguments or are loaded from environment variables.

## Build

It is also possible to build the tool from scratch if needed.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Usage

Use `cargo run -- --help` to compile and run the program. \
Otherwise the usage is exactly the same as the binary.

:::tip
There are some default values set in the `.env` file of the project, which are loaded automatically.
:::

### Release

To compile a release binary use `cargo build --release`. \
An optimized binary is created inside the `target` folder.
