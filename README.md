# Oxidize
Oxidize is a lightweight framework designed to create reusable modules for API rest endpoint development using Rust and Rocket.

## Beta Version
Still in development. I created this project to teach myself in Rust, creating an endpoint that will manage users, authentications and to act as a lightweight base skeleton that can be used for any sort of endpoint development.

## Install
Clone this repo. For the time being, the code uses a nosql mongoDB database to work with users, so before development or testing you must know that the endpoint requires an active mongoDB connection. Run it from the docker compose.

## Testing
simply execute 'cargo test'