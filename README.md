# voodoo-fist
To do list app created in order to learn basics of Rust (and accidently vue).
# Running app
1. Fill `.env` file e.g:
    ```
    MONGO_USER=mongoadmin
    MONGO_PW=secret
    MONGO_HOST=mongo
    MONGO_PORT=27017
    SECRET=some_secret
    ```
1. Build and run backend:
    ```bash
    docker compose build backend && docker compose up
    ```
1. Run front:
    ```bash
    cd application/front && npm run serve
    ```
