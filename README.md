# DB Overflow

Fill Postgres tables with large amounts of generated data.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

## Configuration

The program is designed as a command line interface. \
It's self documenting, just run `cargo run -- --help` to get an overview:

```sh
Options:
      --db-host <DB_HOST>          Database IP address of domain [env: DB_HOST=localhost]
      --db-name <DB_NAME>          Database name [env: DB_NAME=postgres]
      --db-port <DB_PORT>          Database port [env: DB_PORT=5432]
      --db-user <DB_USER>          Database username [env: DB_USER=admin]
      --db-password <DB_PASSWORD>  Database password [env: DB_PASSWORD=admin]
      --delimiter <DELIMITER>      Delimiter used to separate the data values [env: DELIMITER=|]
  -q, --quantity <QUANTITY>        How many records to insert [env: QUANTITY=100]
  -t, --table <TABLE>              Which table to fill [env: TABLE=events]
  -h, --help                       Print help
  -V, --version                    Print version
```

Options can either be passed as CLI arguments or are loaded from environment variables. \
There are some default values set in the `.env` file of the project.

## Usage

Starting the tool is as simple as running:

```
cargo run
```
