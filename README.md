# Wolfgang's fancy simple todo app
This simple app allows managing tasks, inspired by Marc Andreesen's [guide on productivity](https://pmarchive.com/guide_to_personal_productivity.html).

Tasks can be tagged as *Todo Now* / *Watch* / *Later* using a simple CLI in the terminal.


# Server
## Run the server
```
nix develop && cd backend
cargo run -- --config ./example_config.json
```

# Terminal client
## Basic usage
```
nix run .#client -- --config ./client/example_config.json help
```

## Generate completions
```
nix run .#client -- completions --shell=bash > todoclient
source todoclient
# or install:
sudo mkdir -p /usr/local/share/bash-completion/completions/
sudo mv todoclient /usr/local/share/bash-completion/completions/
```

## Convenient aliases
You may want to add some aliases to your `.bashrc`:
```
alias glist='todoclient list --label Now'
alias glistwatch='todoclient list --label Watch'
alias glistlater='todoclient list --label Later'

alias gclose='todoclient close --tid $1'
alias gcreate='todoclient create --title $1'
alias gopen='todoclient re-open --tid $1'

alias gsetnow='todoclient set-label --label Now --tid $1'
alias gsetwatch='todoclient set-label --label Watch --tid $1'
alias gsetlater='todoclient set-label --label Later --tid $1'
```
