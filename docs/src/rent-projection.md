# Storage Rent Projection

Soroban utilizes state archival where contracts must pay ledger rent based on entry size and retention duration.

Calculate projected rent:

```bash
soroban-bench rent --bytes 2048 --tier persistent --ttl 100000
```

Output:
- 30-Day Estimated Rent
- 90-Day Estimated Rent
- 365-Day Estimated Rent

Integrate rent calculations into release gating to prevent deploying contracts with unsustainable storage footprints.
