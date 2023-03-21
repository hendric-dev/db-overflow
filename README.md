# DB Overflow

Fill Postgres tables with large amounts of generated data.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

## Configuration

Configuration is done using environment variables. \
Check out the `.env` file for all supported options.

Either adapt the `.env` file or export your environment variables to override them:

```sh
export DB_USER=<my DB user>
export DB_PASSWORD=<my DB password>
```

## Usage

Starting the tool is as simple as running:

```
cargo run
```
