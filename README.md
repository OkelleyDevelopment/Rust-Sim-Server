# Rust API Server

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
- An API tester / frontend to consume the data

```
cargo run --release
```

Then you will be able to access any of the service endpoints defined.

## Known Bugs

- The web pages are not fully functional, the focus was the API in Rust. At this time there are no plans to continue this frontend, however, a new one could be built to consume the data from this project.

## Resources

- [Actix Docs](https://actix.rs/docs/whatis/)
- [Lotka Volterra Wiki](https://en.wikipedia.org/wiki/Lotka%E2%80%93Volterra_equations)
