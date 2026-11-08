#!/usr/bin/env bash
set -euo pipefail
# Requires: stellar CLI + funded testnet identity
stellar contract build
echo "Upload wasm + deploy FiveAsideSubscription / FiveAsideReceipt on testnet."
echo "See README for initialize / set_plan sequence."
