# IP Duck
This is a variation of IP Chicken, a website that shows you your IP address.

## Setup
The project uses the following:
- Python 3.9
- Flask 2.1
- Nginx 1.21
- Docker
- Docker Compose

For additional information on project specifications, see the ```Pipfile```.

### Environment
In the ```app/``` directory, create a ```.env``` file
that contains the following environment variables:
```
FLASK_ENV=production
```

## Building
The project uses Docker. Ensure Docker and Docker Compose are installed before continuing.

To build, run ```docker-compose build```

## Running
To run the web app, run ```docker-compose up -d```, then 
go to http://localhost using your web browser.
