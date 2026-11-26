//! Core's `sendrawtransaction` answer for a refused local broadcast. RPC,
//! Electrum and Esplora all answer with it: electrs forwards Core's.

use rbitcoin_query::QueryError;

use crate::test_accept::accept_reject_reason;

/// Core `RPC_MISC_ERROR`.
const MISC_ERROR: i64 = -1;
/// Core `RPC_TRANSACTION_ERROR`.
const TRANSACTION_ERROR: i64 = -25;
/// Core `RPC_TRANSACTION_REJECTED`.
const TRANSACTION_REJECTED: i64 = -26;
/// Core `RPC_TRANSACTION_ALREADY_IN_UTXO_SET`.
const TRANSACTION_ALREADY_IN_UTXO_SET: i64 = -27;

/// Core `TransactionError::MISSING_INPUTS` text on `sendrawtransaction`.
const MISSING_OR_SPENT: &str = "bad-txns-inputs-missingorspent";

/// The code and message `sendrawtransaction` answers a refused tx with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BroadcastReject {
    pub code: i64,
    pub message: String,
}

impl BroadcastReject {
    /// Core maps `MISSING_INPUTS` to `-25 bad-txns-inputs-missingorspent`;
    /// every other mempool reject is `-26` with its reason.
    pub fn of(e: &impl std::fmt::Display) -> Self {
        let reason = accept_reject_reason(e);
        if reason == "missing-inputs" {
            return Self {
                code: TRANSACTION_ERROR,
                message: MISSING_OR_SPENT.into(),
            };
        }
        Self {
            code: TRANSACTION_REJECTED,
            message: reason,
        }
    }

    /// Core `BroadcastTransaction` when one of the tx's own outputs is a coin.
    pub fn already_in_utxo_set() -> Self {
        Self {
            code: TRANSACTION_ALREADY_IN_UTXO_SET,
            message: "Transaction outputs already in utxo set".into(),
        }
    }

    pub(crate) fn query(e: QueryError) -> Self {
        Self {
            code: MISC_ERROR,
            message: format!("query failed: {e}"),
        }
    }
}
