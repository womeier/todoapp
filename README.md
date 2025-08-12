# Wolfgang's fancy todo app

## Run the backend

```
nix develop && cd backend
cargo run -- --config ./example_config.json
```

## In a new tab, call the client

```
nix run .#client -- --config ./client/example_config.json help
```

## To generate the OpenAPI schema, and client
```
nix develop
just openapi-generate
```

## Generate client completions
```
nix run .#client -- completions --shell=bash > todoclient
source todoclient
# or install:
sudo mkdir -p /usr/local/share/bash-completion/completions/
sudo mv todoclient /usr/local/share/bash-completion/completions/
```
