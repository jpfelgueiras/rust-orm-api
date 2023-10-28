# Run postgres docker 

```
docker run --rm -it -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres:alpine
```

# Install libpq-dev and pgclient
```
sudo apt install libpq-dev postgresql-client
```

# Tutorial URL 
https://hackernoon.com/building-an-api-in-rust-with-rocketrs-and-dieselrs-following-clean-architecture