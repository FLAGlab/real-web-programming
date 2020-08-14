# Real Web Programming
Web application created with Rust and WebAssembly, specifically using the rocket.rs and yew.rs frameworks for back-end and front-end respectively. The objective of this project is to test the current state of these technologies.

# Prerequisites
To run the application it is necessary to have installed the Rust programming language and the node package manager, npm. It is also necessary to run the nightly version of Rust by following the instructions in the following url: <a href="https://rocket.rs/v0.4/guide/getting-started/" target="_blank">Rocket Getting Started</a>.

# Back-end
## How to build

```sh
cargo build
```

## How to run

```sh
cargo run
```

# Front-end
## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests
npm test
```
