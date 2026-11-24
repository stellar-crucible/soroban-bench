use serde::{Deserialize, Serialize};

/// Storage tier under Stellar Soroban state archival.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageTier {
    Instance,
    Persistent,
    Temporary,
}

/// Stellar Protocol version configuration for rent and state archival calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolVersion {
    Protocol20,
    Protocol21,
    Protocol22,
}

/// Ledger rent calculation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentCalculation {
    /// Number of bytes stored in the ledger entry.
    pub entry_bytes: usize,
    /// Storage classification tier.
    pub tier: StorageTier,
    /// Minimum ledger sequence retention target.
    pub target_ttl_ledgers: u32,
    /// Approximate rent fee in stroops for 30 days (~518,400 ledgers).
    pub rent_30_days_stroops: i64,
    /// Approximate rent fee in stroops for 90 days.
    pub rent_90_days_stroops: i64,
    /// Approximate rent fee in stroops for 365 days.
    pub rent_365_days_stroops: i64,
}

/// Estimated cost to restore an archived storage entry back to active state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorationEstimate {
    pub protocol: ProtocolVersion,
    pub entry_bytes: usize,
    /// Base restoration flat fee in stroops.
    pub base_restore_fee_stroops: i64,
    /// Byte-dependent restoration fee in stroops.
    pub byte_fee_stroops: i64,
    /// Total restoration fee in stroops.
    pub total_fee_stroops: i64,
}

/// Utility for calculating Soroban state archiving and TTL extension rent fees.
pub struct StorageRentCalculator {
    /// Fee per byte per ledger (in fractions of stroop, typically parameterized by protocol constants).
    fee_per_byte_per_ledger: f64,
}

impl Default for StorageRentCalculator {
    fn default() -> Self {
        Self {
            fee_per_byte_per_ledger: 0.0001,
        }
    }
}

impl StorageRentCalculator {
    pub fn new(fee_per_byte_per_ledger: f64) -> Self {
        Self {
            fee_per_byte_per_ledger,
        }
    }

    /// Calculate rent projection for a ledger entry.
    pub fn project_rent(
        &self,
        entry_bytes: usize,
        tier: StorageTier,
        target_ttl_ledgers: u32,
    ) -> RentCalculation {
        let ledgers_per_day = 17_280.0;
        let ledgers_30d = 30.0 * ledgers_per_day;
        let ledgers_90d = 90.0 * ledgers_per_day;
        let ledgers_365d = 365.0 * ledgers_per_day;

        let multiplier = match tier {
            StorageTier::Temporary => 0.5,
            StorageTier::Instance => 1.0,
            StorageTier::Persistent => 1.0,
        };

        let rent_30 = (entry_bytes as f64 * ledgers_30d * self.fee_per_byte_per_ledger * multiplier)
            .ceil() as i64;
        let rent_90 = (entry_bytes as f64 * ledgers_90d * self.fee_per_byte_per_ledger * multiplier)
            .ceil() as i64;
        let rent_365 =
            (entry_bytes as f64 * ledgers_365d * self.fee_per_byte_per_ledger * multiplier).ceil()
                as i64;

        RentCalculation {
            entry_bytes,
            tier,
            target_ttl_ledgers,
            rent_30_days_stroops: rent_30,
            rent_90_days_stroops: rent_90,
            rent_365_days_stroops: rent_365,
        }
    }

    /// Calculate restoration fee projection for restoring archived state back into active ledger.
    pub fn project_restoration(
        &self,
        entry_bytes: usize,
        protocol: ProtocolVersion,
    ) -> RestorationEstimate {
        let (base_fee, per_byte_multiplier) = match protocol {
            ProtocolVersion::Protocol20 => (10_000, 2.0),
            ProtocolVersion::Protocol21 => (8_000, 1.8),
            ProtocolVersion::Protocol22 => (6_500, 1.5),
        };

        let byte_fee = (entry_bytes as f64 * per_byte_multiplier).ceil() as i64;
        let total = base_fee + byte_fee;

        RestorationEstimate {
            protocol,
            entry_bytes,
            base_restore_fee_stroops: base_fee,
            byte_fee_stroops: byte_fee,
            total_fee_stroops: total,
        }
    }
}
