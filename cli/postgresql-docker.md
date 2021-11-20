# Docker page
https://hub.docker.com/_/postgres

# Pull container : 
```bash
docker pull postgres
```

# Run container: 
```bash
    docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
```

The default `postgres` user and database are created in the entrypoint with `initdb`


# To get the Postgres server's IP address

https://stackoverflow.com/questions/26343178/dockerizing-postgresql-psql-connection-refused/26344405



