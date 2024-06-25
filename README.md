![Oxidize logo](https://hamrodev.com/images/oxidize%20logo.png)

# Oxidize
Oxidize is a lightweight framework designed to create reusable modules for API rest endpoint development using Rust and Rocket.

## Beta Version
Still in development. I created this project to teach myself in Rust, creating an endpoint that will manage users, authentications and to act as a lightweight base skeleton that can be used for any sort of endpoint development. For the time being. The endpoint allows basic user crud operations with public/private key authentication (RSA) via json webtokens.

The endpoint runs db queries against a mongodb server. The instance can be ran from the docker compose file.

## Install
Clone this repo. For the time being, the code uses a nosql mongoDB database to work with users, so before development or testing you must know that the endpoint requires an active mongoDB connection. Run it from the docker compose. Then, run 'cargo build' to build the executable, and 'cargo run' to deploy the endpoint in debug mode.

## Env File
Before running, or testing, make sure you 'mv .env.dist .env' . Oxidize uses its own configuration handler that reads and parses the environment files and serves them to all modules across the application.

## Testing
simply execute 'cargo test'. Make sure a mongo db database is running and config is correct.