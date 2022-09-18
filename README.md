# Twidder

## This is a mini social network project to understand actix-web

Current repo represents the server side.

## Setup

### Create `.env` file

After starting the server it will tell what variables to add in `.env`.

### Database

Run `docker compose up` to create a DB. The mandatory variables for docker and postgresql are located in the `docker-compose.yml` file.
In this project `SeaORM` used, to allow all functionality to work properly - install `cargo install sea-orm-cli`

### TLS Certificate

`openssl` is required

Create the `/certs` directory in the root of the project

Change directory `cd ./certs`
Generate the certificates `openssl req -new -newkey rsa:4096 -x509 -sha256 -days 365 -nodes -out cert.pem -keyout key.pem`

### To run the server

Execute the following command: `cargo run`.
