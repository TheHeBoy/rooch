// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::types::IndexedTransaction;
use anyhow::Result;
use coerce::actor::message::Message;
use moveos_types::h256::H256;
use moveos_types::transaction::{TransactionExecutionInfo, VerifiedMoveOSTransaction};
use rooch_types::transaction::{TransactionSequenceInfo, TransactionWithInfo, TypedTransaction};
use serde::{Deserialize, Serialize};

/// Transaction Indexer Message
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexerTransactionMessage {
    pub transaction: TypedTransaction,
    pub sequence_info: TransactionSequenceInfo,
    pub execution_info: TransactionExecutionInfo,
    pub moveos_tx: VerifiedMoveOSTransaction,
}

impl Message for IndexerTransactionMessage {
    type Result = Result<IndexedTransaction>;
}

/// Query Transactions By Hash Message
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTransactionsByHashMessage {
    pub tx_hashes: Vec<H256>,
}

impl Message for QueryTransactionsByHashMessage {
    type Result = Result<Vec<Option<TransactionWithInfo>>>;
}