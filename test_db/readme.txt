To check the BD content with adminer:
- url http://localhost:8080
- system: postgreSQL
- server: db (service name)
- user, password, bdd: see docker-compose.yml

For diesel setup (ref https://diesel.rs/guides/getting-started.html) 
echo 'DATABASE_URL=postgres://twitter:changeme@localhost/twitter' > .env
