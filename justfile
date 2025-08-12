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
    cd ./client/api/ && cargo add hyper-tls@0.6.0
    sed "s/HttpConnector>/HttpsConnector<HttpConnector>>/g" -i ./client/api/src/apis/configuration.rs
    sed "s/build_http()/build(HttpsConnector::new())/g" -i ./client/api/src/apis/configuration.rs
    sed "s/use hyper;/use hyper; use hyper_tls::HttpsConnector;/g" -i ./client/api/src/apis/configuration.rs

    cd ./client/api && cargo fmt
