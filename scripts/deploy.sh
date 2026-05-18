#!/usr/bin/env bash
set -euo pipefail

[[ -f .env ]] && source .env

: "${SSH_USER:?set SSH_USER in .env or env}"
: "${SSH_HOST:?set SSH_HOST in .env or env}"
: "${SSH_PATH:?set SSH_PATH in .env or env}"
: "${SSH_KEY:=$HOME/.ssh/tseeley_deploy}"

cargo build --release --no-default-features
./target/release/tseeley build

rsync -avz --delete \
    --exclude '.DS_Store' \
    -e "ssh -i $SSH_KEY" \
    public/ \
    "$SSH_USER@$SSH_HOST:$SSH_PATH/"

echo "Deployed to $SSH_HOST"
