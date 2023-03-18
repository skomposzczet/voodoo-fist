# voodoo-fist

# Running app
1. Fill `.env` file based on e.g:
    ```
    MONGO_USER=mongoadmin
    MONGO_PW=secret
    MONGO_HOST=mongo
    MONGO_PORT=27017
    SECRET=some_secret
    ```
1. Build backend docker image:
    ```bash
    docker compose build backend
    ```
1. Run application:
    ```bash
    docker compose up
    ```