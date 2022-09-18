# Twidder Server

A mini social network project.\
This is the server-side.

Client-side is available [here](https://github.com/CommanderXA/twidder_mobile)

**Used technologies in this project:**
* Rust
* Actix-web
* Sea-ORM
* PostgreSQL
* Docker
* TSL Certificates

## Setup

### Create `.env` file

After starting the server it will tell what variables to add in `.env`.

### Database

Run `docker compose up` to deploy docker containers.\
The mandatory variables for docker and postgresql are located in the `docker-compose.yml` file.\
`SeaORM` is used in this project, to use all functionality of the ORM - install `cargo install sea-orm-cli`

### TLS Certificate

`openssl` is required\
Create the `/certs` directory in the root of the project

Change directory `cd ./certs`\
Generate the certificates `openssl req -new -newkey rsa:4096 -x509 -sha256 -days 365 -nodes -out cert.pem -keyout key.pem`

### To run the server

Execute the following command: `cargo run`.

## License

The project is distributed under the Mozilla Public License 2.0 (MPL-2.0).
