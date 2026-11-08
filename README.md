# Five Aside — Soroban Contracts

Soroban workspace for **[Five Aside](https://github.com/Five-Aside)** — a multi-tenant SaaS that runs weekly five-a-side leagues (fixtures, tables, payments, discipline).

## Contracts

1. **`subscription/`** — `FiveAsideSubscription`  
   League organisers pay SaaS plans (`STARTER` / `CLUB` / `PRO`) in **USDC on Stellar**. Backend unlocks the tenant entitlement after verifying the on-chain payment.

2. **`receipt/`** — `FiveAsideReceipt`  
   Anchors matchday / standings hashes on-chain (`publish_receipt` / `settle_receipt`) so published results are tamper-evident.

## Companion repos

- Web (SaaS portal): https://github.com/Five-Aside/web  
- Backend (API + Prisma + league engine): https://github.com/Five-Aside/backend  

## Why Stellar

- Global USDC SaaS billing without forcing every amateur club onto a single local card rail  
- Public, cheap anchors for official week results  

## Build

```bash
rustup target add wasm32v1-none
cargo install --locked stellar-cli
stellar contract build
```

## Not in scope

Five Aside is **league ops SaaS**, not a custodial consumer wallet and not a betting exchange.
