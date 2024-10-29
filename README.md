# IP Duck
This is a variation of IP Chicken, a website that shows you your IP address.

## Setup
The project uses the following:
- Rust
- Axum
- Bootstrap
- Nginx
- Docker
- Docker Compose

For additional information on project specifications, see the ```Pipfile```.

### Environment
In the ```app_rs/``` directory, create a ```.env``` file that contains the following environment variables:
```
CORS_ALLOWED_ORIGINS=http://localhost http://127.0.0.1
```

## Building
The project uses Docker. Ensure Docker and Docker Compose are installed before continuing.

To build, run ```docker-compose build```

## Running
To run the web app, run ```docker-compose up -d```, then go to http://localhost using your web browser.
