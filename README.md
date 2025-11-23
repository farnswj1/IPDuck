# IP Duck

This is a variation of IP Chicken, a website that shows you your IP address.

## Setup

The project uses the following:

- Rust
- Actix Web
- Bootstrap
- Nginx
- Docker
- Docker Compose

For additional information on project specifications, see the `Cargo.toml` in `app_rs`.

## Building

The project uses Docker. Ensure Docker and Docker Compose are installed before continuing.

To build, run `docker-compose build`.

## Running

To run the web app, run `docker-compose up -d`, then go to <http://localhost> using your web browser.
