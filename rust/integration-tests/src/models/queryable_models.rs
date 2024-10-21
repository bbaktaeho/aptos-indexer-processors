// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::extra_unused_lifetimes)]

use bigdecimal::BigDecimal;
use diesel::{Identifiable, Insertable, Queryable};
use field_count::FieldCount;
use processor::schema::{
    coin_supply, current_fungible_asset_balances_legacy, events, fungible_asset_activities,
    fungible_asset_balances, fungible_asset_metadata, token_activities_v2,
};
use serde::{Deserialize, Serialize};
/**
* Event model
* this is created // b/c there is inserated_at field which isn't defined in the Event struct, we can't just load the events directly without specifying the fields.
        // TODO: make this more generic to load all fields, then we should be able to run tests for all processor in one test case.

*/
#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(transaction_version, event_index))]
#[diesel(table_name = events)]
pub struct Event {
    pub sequence_number: i64,
    pub creation_number: i64,
    pub account_address: String,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub type_: String,
    pub data: serde_json::Value,
    pub inserted_at: chrono::NaiveDateTime,
    pub event_index: i64,
    pub indexed_type: String,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(transaction_version, event_index))]
#[diesel(table_name = fungible_asset_activities)]
pub struct FungibleAssetActivity {
    pub transaction_version: i64,
    pub event_index: i64,
    pub owner_address: Option<String>,
    pub storage_id: String,
    pub asset_type: Option<String>,
    pub is_frozen: Option<bool>,
    pub amount: Option<BigDecimal>,
    pub type_: String,
    pub is_gas_fee: bool,
    pub gas_fee_payer_address: Option<String>,
    pub is_transaction_success: bool,
    pub entry_function_id_str: Option<String>,
    pub block_height: i64,
    pub token_standard: String,
    pub transaction_timestamp: chrono::NaiveDateTime,
    pub inserted_at: chrono::NaiveDateTime,
    pub storage_refund_amount: BigDecimal,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(transaction_version, event_index))]
#[diesel(table_name = token_activities_v2)]
pub struct TokenActivityV2 {
    pub transaction_version: i64,
    pub event_index: i64,
    pub event_account_address: String,
    pub token_data_id: String,
    pub property_version_v1: BigDecimal,
    pub type_: String,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub token_amount: BigDecimal,
    pub before_value: Option<String>,
    pub after_value: Option<String>,
    pub entry_function_id_str: Option<String>,
    pub token_standard: String,
    pub is_fungible_v2: Option<bool>,
    pub transaction_timestamp: chrono::NaiveDateTime,
    pub inserted_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = fungible_asset_balances)]
pub struct FungibleAssetBalance {
    pub transaction_version: i64,
    pub write_set_change_index: i64,
    pub storage_id: String,
    pub owner_address: String,
    pub asset_type: String,
    pub is_primary: bool,
    pub is_frozen: bool,
    pub amount: BigDecimal,
    pub transaction_timestamp: chrono::NaiveDateTime,
    pub token_standard: String,
    pub inserted_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(storage_id))]
#[diesel(table_name = current_fungible_asset_balances_legacy)]
pub struct CurrentFungibleAssetBalance {
    pub storage_id: String,
    pub owner_address: String,
    pub asset_type: String,
    pub is_primary: bool,
    pub is_frozen: bool,
    pub amount: BigDecimal,
    pub last_transaction_timestamp: chrono::NaiveDateTime,
    pub last_transaction_version: i64,
    pub token_standard: String,
    pub inserted_at: chrono::NaiveDateTime,
}

// #[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable, Default)]
// #[diesel(primary_key(storage_id))]
// #[diesel(table_name = current_fungible_asset_balances)]
// pub struct CurrentUnifiedFungibleAssetBalance {
//     pub storage_id: String,
//     pub owner_address: String,
//     pub asset_type: Option<String>,
//     pub coin_type: Option<String>,  // Changed from asset_type_v1/v2 to match schema
//     pub is_primary: Option<bool>,
//     pub is_frozen: bool,
//     pub amount_v1: Option<BigDecimal>,
//     pub amount_v2: Option<BigDecimal>,
//     pub amount: Option<BigDecimal>,  // Added amount field to match schema
//     pub last_transaction_version_v1: Option<i64>,
//     pub last_transaction_version_v2: Option<i64>,
//     pub last_transaction_version: Option<i64>,  // Added to match schema
//     pub last_transaction_timestamp_v1: Option<chrono::NaiveDateTime>,
//     pub last_transaction_timestamp_v2: Option<chrono::NaiveDateTime>,
//     pub last_transaction_timestamp: Option<chrono::NaiveDateTime>,  // Added to match schema
//     pub inserted_at: chrono::NaiveDateTime,
// }

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(asset_type))]
#[diesel(table_name = fungible_asset_metadata)]
pub struct FungibleAssetMetadataModel {
    pub asset_type: String,
    pub creator_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub icon_uri: Option<String>,
    pub project_uri: Option<String>,
    pub last_transaction_version: i64,
    pub last_transaction_timestamp: chrono::NaiveDateTime,
    pub supply_aggregator_table_handle_v1: Option<String>,
    pub supply_aggregator_table_key_v1: Option<String>,
    pub token_standard: String,
    pub is_token_v2: Option<bool>,
    pub inserted_at: chrono::NaiveDateTime,
    pub supply_v2: Option<BigDecimal>,
    pub maximum_v2: Option<BigDecimal>,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(transaction_version, coin_type_hash))]
#[diesel(table_name = coin_supply)]
pub struct CoinSupply {
    pub transaction_version: i64,
    pub coin_type_hash: String,
    pub coin_type: String,
    pub supply: BigDecimal,
    pub transaction_timestamp: chrono::NaiveDateTime,
    pub transaction_epoch: i64,
    pub inserted_at: chrono::NaiveDateTime,
}