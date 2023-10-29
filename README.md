# Run postgres docker 

```shell
docker run --rm -it -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres:alpine
cd infrastructure
diesel setup
```

# Install libpq-dev and pgclient
```shell
sudo apt install libpq-dev postgresql-client
```

# Step to run

```shell
cargo watch -x run
```

# Tutorial URL 
https://hackernoon.com/building-an-api-in-rust-with-rocketrs-and-dieselrs-following-clean-architecture