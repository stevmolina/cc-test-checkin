# Event check-in on Stellar 🎟️

An event check-in list on **Stellar Testnet**: each attendee checks in **once** with their name, signed with their wallet, and the page shows who came. No real money.

- Page: https://stevmolina.github.io/cc-test-checkin/
- Contract: [`CBRDHN56…WZSR`](https://stellar.expert/explorer/testnet/contract/CBRDHN56EEA3ACWM4MV2QVVY5ERNIXWYOJY6DZLTBI67NYMHT4SVWZSR)

Built for **The Commit Club** at BAF × Stellar, from the [Proof of Thanks starter](https://github.com/stevmolina/proof-of-thanks-starter).

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/stevmolina/cc-test-checkin)

## What's inside

- `contracts/checkin`: a Soroban smart contract with `check_in` (once per wallet), `count`, `get` and `has_checked_in`.
- `docs/`: a static page that connects [Freighter](https://www.freighter.app) and calls the contract (`docs/checkin-client.js`). Published with GitHub Pages from `/docs`.
- `scripts/`: `setup.sh` (tools + funded Testnet account), `deploy.sh` (test, build, deploy), `smoke-test.cjs` (end-to-end check with a fresh Testnet wallet, including a rejected duplicate).

## Quick start

```bash
bash scripts/setup.sh      # once
bash scripts/deploy.sh     # deploy your contract to Testnet
npx --yes serve docs        # open the page locally
```

Then push to `main`. To publish the page: **Settings → Pages → Deploy from a branch → `main` / `/docs`** (or run `gh api -X POST repos/OWNER/REPO/pages -f "source[branch]=main" -f "source[path]=/docs"`).

## With an AI agent

Install the Commit Club kit and run `/stellarize`. See [stellar-commit-club](https://github.com/stevmolina/stellar-commit-club).

## Wallet setup

1. Install [Freighter](https://www.freighter.app).
2. Settings → Network → **Testnet**.
3. Fund your account for free with [Friendbot](https://lab.stellar.org/account/fund).
