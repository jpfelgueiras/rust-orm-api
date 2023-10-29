#!/bin/sh
docker run --rm -it -e POSTGRES_PASSWORD=postgres -v ./postgres-data:/var/lib/postgresql/data -p 5432:5432 postgres:alpine