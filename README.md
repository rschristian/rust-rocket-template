# Rust Rocket Template Server

This is a template API server built with Rocket, Diesel, and Postgres, with a prebuilt authentication system. This should help you get up and running rapidly.

This server is built with [Rocket](https://rocket.rs), a simple, fast, and type-safe web framework for Rust. The database used is [PostgreSQL](https://www.postgresql.org/), the world's most advanced open source relational database, with [Diesel](http://diesel.rs), a safe, extensible ORM and query builder for Rust, acting as the bridge between Rocket and the database. Diesel also provides all migration management for this project.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

```
Rust nightly
Cargo
Docker/Compose - optional
Diesel-cli and PostgreSQL - necessary only if Docker is not used
PostgreSQL-libs (libpq-dev, postgresql-libs, etc. Provided by your distribution)
```

### Running

To aid in the process of starting the server, I have provided a Docker-Compose file for the database and created a container for the CLI tool used to manage migrations. From the project root, run:

```
docker-compose up -d
```

This will start the Postgres DB in the background. To populate the database, run:

```
docker run --rm \
    -v "$PWD:/volume" \
    -w /volume \
    --network="$(basename $PWD)_default" \
    ryanchristian4427/diesel-cli migration run
```

The Docker image built for diesel-cli will run "Diesel" without any arguments, making the container act like a normal CLI. However, that very large command is necessary upon every use. I therefore recommend creating an alias in a `.bashrc` or `.zshrc` so the tool can be just called with "diesel-cli [command]".

```
alias diesel-cli='docker run --rm \
    -v "$PWD:/volume" \
    -w /volume \
    --network="$(basename $PWD)_default" \
    ryanchristian4427/diesel-cli';
```

If you'd like to avoid Docker, a local Postgres database is necessary. Make sure to edit the [.env](.env) file to match your connection URL. You will also need to install the diesel-cli and run it with:

```
diesel migration run
```

The server can then be ran using a debug build with:

```
cargo run
```

Create a production build with:

```
cargo build --release
```

## Running the tests

The unit tests and integration tests can all be ran using:

```
cargo test
```

The unit tests are found in the same file as the code they test, while the integration tests are found in ~/tests.

### Code Style

All formatting is done with the lovely Rustfmt, which can be ran with:

```
cargo fmt
```

The linter Clippy is also used and often its suggestions are often used, but there are some exceptions. Clippy can be ran with

```
cargo clippy
```

## Built With

* [Rocket](https://rocket.rs) - A simple, fast, and type-safe web framework for Rust
* [Diesel](http://diesel.rs) - A safe, extensible ORM and query builder for Rust
* [PostgreSQL](https://www.postgresql.org/) - The world's most advanced open source relational database

## Authors

* **Ryan Christian** - *Entire Project* - [RyanChristian4427](https://github.com/RyanChristian4427)
