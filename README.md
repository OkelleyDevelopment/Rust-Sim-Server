# Rust Simulation Server

Author(s): Nicholas O'Kelley

Date: April 14, 2022

## Motivation

Having experience with NodeJS and TypeScript, I wanted to build out
a web server that allowed for similar tasks to be completed but in Rust. This project allows
me to reinforce the concepts and continue to get practice in a language that will be
mission critical for a few other ideas in the pipeline.

## Project Execution

You will need installed:

- Rust
- API tester (Insomnia, Postman, curl, etc)

Optional:

- frontend to consume the data
- Docker

To run the project:

```
cargo run --release
```

Then you will be able to access any of the service endpoints defined.

## Docker

Build the project:

```
docker build . -t sim-serv
```

The resulting image will be about 79.5 MB as we transfer the final executable
to a clean minimal image.

```
docker container run sim-serv
```

From here you would be able to reach any of the endpoints like above, but this
time from a minified environment _without needing to install the Rust compiler locally_!

## Resources

- [Actix Docs](https://actix.rs/docs/whatis/)
- [Lotka Volterra Wiki](https://en.wikipedia.org/wiki/Lotka%E2%80%93Volterra_equations)
