# 📚 DB Overflow Documentation

This documentation is created using [Vitepress](https://vitepress.vuejs.org/) and published automatically on the repositories pages.

## Prerequisites

The following dependencies are needed to generate the documentation:

* [Node.js](https://nodejs.org/en/)
* [Bun](https://bun.sh/)

## Install

First install the dependencies using `bun` inside the documentation folder:

```sh
bun install
```

## Development

Vitepress features a blazing fast dev server with hot reloading. Start it using:

```sh
bun run dev
```

The server will be available at http://localhost:5173/db-overflow/

## Docker

Only have Docker installed? No problem, just mount the folder into a Node.js container:

```sh
docker run --rm -it --entrypoint bash \
  -p 5173:5173 \
  -v $PWD:/documentation \
  -w /documentation \
  node:19-slim
```

You find yourself inside the container where you need to install Bun:

```sh
npm install --location=global bun
```

Now you can run all `bun` commands as usual.

> **Warning** \
> Start all bun commands with `--host` flag to expose the network \
> (e.g. `bun run dev --host`)
