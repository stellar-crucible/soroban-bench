# Storage Rent Projection & State Archival

Soroban incorporates state archival: ledger entries expire if their Time-To-Live (TTL) is not actively renewed. Entries that become archived incur restoration fees, while live entries require retention rent.

## Calculating Retention Rent

Estimate storage costs over 30, 90, and 365-day horizons:

```bash
soroban-bench rent --bytes 2048 --tier persistent --ttl 100000 --protocol 21
```

### Storage Tiers

- **Persistent**: Long-lived data (balances, ownership records, vaults). Must be renewed or restored.
- **Instance**: Bound to the contract instance lifecycle.
- **Temporary**: Ephemeral data automatically reclaimed after TTL without archival fees.

## Protocol 20 vs 21 vs 22 State Archival

Stellar Protocol upgrades iteratively reduce restoration and footprint overhead:

```bash
soroban-bench rent --bytes 4096 --protocol 22
```

The calculator compares base restoration fees and per-byte dynamic charges to guide optimal data packing strategies.
