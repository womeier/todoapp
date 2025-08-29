build:
    cd backend && cargo build

install-client:
    cd client && cargo build --release
    sudo cp ./client/target/release/client /usr/local/bin/todoclient

run:
    cd backend && cargo run -- --config {{justfile_directory()}}/backend/example_config.json

watch:
    cd backend && watchexec -r -e rs "sleep 1 && cargo run -- --config {{justfile_directory()}}/backend/example_config.json"

diesel-make-migration name:
    cd backend && diesel migration generate --diff-schema {{name}} \
        --database-url {{justfile_directory()}}/backend/db/database.db

diesel-migrate:
    cd backend && diesel migration run \
        --database-url {{justfile_directory()}}/backend/db/database.db

openapi-update:
    cd backend && cargo run -- \
        --config {{justfile_directory()}}/backend/example_config.json \
        --openapi-output-path {{justfile_directory()}}/openapi.json

    openapi-generator-cli generate -i openapi.json -g rust -c config.json --library hyper -o ./client/api

    # patch client to support TLS
    cd ./client/api && cargo fmt
    cd ./client/api/ && cargo add hyper-rustls rustls
    git apply ./client/hyper-rustls.patch
    cd ./client/api && cargo fmt
