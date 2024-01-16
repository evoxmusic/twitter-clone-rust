# Tutorial

[Read the tutorial article](https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/)

# Test database

See `test_db` folder to set up a dockerized postgres database for testing

To check the database content with adminer:
- url http://localhost:8080
- system: postgreSQL
- server: db (service name)
- user, password, bdd: see docker-compose.yml



Note that thanks to dotenvy package, the application finds the database with the .env file. Same as diesel initialization process

For diesel setup see https://diesel.rs/guides/getting-started.html
