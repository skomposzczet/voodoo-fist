# voodoo-fist
To do list app created in order to learn basics of Rust (and accidently vue).

https://github.com/skomposzczet/voodoo-fist/assets/72391558/0c09bd11-34e6-4056-96ae-9ec883a724d7

# About
It is a simple todo list created with Rust and TypeScript with MongoDB as database.
## Backend
Backend is written in Rust. It is powered by Warp framework and Tokio as async runtime.
API contains endpoints for logging, acessing and modyfing users data.
Ensures safety with password hashing and uses JWT for authenticating.
## Frontend
Front app is written in Vue with TypeScript (for whatever reason). It is quite pretty.
## Deploying
App can be deployed with supplied `docker-compose.yaml`. For both backend and frontend there is Dockerfile.
Backend docker image is built pretty straighforward. Compiled with cargo, binary is run and port is exposed to docker network.
Frontend is run in docker with NGINX which also acts as reverse proxy for API.
# Running app
1. Fill `.env.mongo` file e.g:
    ```
    MONGO_USER=<mongo username consistent with one in docker-compose.yaml>
    MONGO_PW=<mongo password consistent with one in docker-compose.yaml>
    MONGO_HOST=mongo
    MONGO_PORT=27017
    SECRET=some_secret
    ```
1. Build and run cluster:
    ```bash
    docker compose up
    ```
