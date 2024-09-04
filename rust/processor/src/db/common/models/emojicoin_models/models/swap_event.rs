use crate::{
    db::common::models::emojicoin_models::{
        enums,
        json_types::{StateEvent, SwapEvent, TxnInfo},
        utils::micros_to_naive_datetime,
    },
    schema::swap_events,
};
use bigdecimal::BigDecimal;
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(market_id, market_nonce))]
#[diesel(table_name = swap_events)]
pub struct SwapEventModel {
    // Transaction metadata.
    pub transaction_version: i64,
    pub sender: String,
    pub entry_function: Option<String>,
    pub transaction_timestamp: chrono::NaiveDateTime,

    // Market and state metadata.
    pub market_id: i64,
    pub symbol_bytes: Vec<u8>,
    pub bump_time: chrono::NaiveDateTime,
    pub market_nonce: i64,
    pub trigger: enums::Trigger,

    // Swap event data.
    pub swapper: String,
    pub integrator: String,
    pub integrator_fee: i64,
    pub input_amount: i64,
    pub is_sell: bool,
    pub integrator_fee_rate_bps: i16,
    pub net_proceeds: i64,
    pub base_volume: i64,
    pub quote_volume: i64,
    pub avg_execution_price_q64: BigDecimal,
    pub pool_fee: i64,
    pub starts_in_bonding_curve: bool,
    pub results_in_state_transition: bool,
    pub balance_as_fraction_of_circulating_supply_before_q64: BigDecimal,
    pub balance_as_fraction_of_circulating_supply_after_q64: BigDecimal,

    // State event data.
    pub clamm_virtual_reserves_base: i64,
    pub clamm_virtual_reserves_quote: i64,
    pub cpamm_real_reserves_base: i64,
    pub cpamm_real_reserves_quote: i64,
    pub lp_coin_supply: BigDecimal,
    pub cumulative_stats_base_volume: BigDecimal,
    pub cumulative_stats_quote_volume: BigDecimal,
    pub cumulative_stats_integrator_fees: BigDecimal,
    pub cumulative_stats_pool_fees_base: BigDecimal,
    pub cumulative_stats_pool_fees_quote: BigDecimal,
    pub cumulative_stats_n_swaps: i64,
    pub cumulative_stats_n_chat_messages: i64,
    pub instantaneous_stats_total_quote_locked: i64,
    pub instantaneous_stats_total_value_locked: BigDecimal,
    pub instantaneous_stats_market_cap: BigDecimal,
    pub instantaneous_stats_fully_diluted_value: BigDecimal,
}

impl SwapEventModel {
    pub fn new(
        txn_info: TxnInfo,
        swap_event: SwapEvent,
        state_event: StateEvent,
    ) -> SwapEventModel {
        let StateEvent {
            market_metadata,
            state_metadata,
            clamm_virtual_reserves: clamm,
            cpamm_real_reserves: cpamm,
            lp_coin_supply,
            cumulative_stats: c_stats,
            instantaneous_stats: i_stats,
            ..
        } = state_event;

        let SwapEvent {
            market_id,
            market_nonce,
            swapper,
            integrator,
            integrator_fee,
            input_amount,
            is_sell,
            integrator_fee_rate_bps,
            net_proceeds,
            base_volume,
            quote_volume,
            avg_execution_price_q64,
            pool_fee,
            starts_in_bonding_curve,
            results_in_state_transition,
            time,
            balance_as_fraction_of_circulating_supply_before_q64,
            balance_as_fraction_of_circulating_supply_after_q64,
            ..
        } = swap_event;

        SwapEventModel {
            // Transaction metadata.
            transaction_version: txn_info.version,
            sender: txn_info.sender.clone(),
            entry_function: txn_info.entry_function.clone(),
            transaction_timestamp: txn_info.timestamp,

            // Market and state metadata.
            market_id,
            symbol_bytes: market_metadata.emoji_bytes,
            bump_time: micros_to_naive_datetime(time),
            market_nonce,
            trigger: state_metadata.trigger,

            // Swap event data.
            swapper,
            integrator,
            integrator_fee,
            input_amount,
            is_sell,
            integrator_fee_rate_bps,
            net_proceeds,
            base_volume,
            quote_volume,
            avg_execution_price_q64,
            pool_fee,
            starts_in_bonding_curve,
            results_in_state_transition,
            balance_as_fraction_of_circulating_supply_before_q64,
            balance_as_fraction_of_circulating_supply_after_q64,

            // State event data.
            clamm_virtual_reserves_base: clamm.base,
            clamm_virtual_reserves_quote: clamm.quote,
            cpamm_real_reserves_base: cpamm.base,
            cpamm_real_reserves_quote: cpamm.quote,
            lp_coin_supply: lp_coin_supply.clone(),
            cumulative_stats_base_volume: c_stats.base_volume,
            cumulative_stats_quote_volume: c_stats.quote_volume,
            cumulative_stats_integrator_fees: c_stats.integrator_fees,
            cumulative_stats_pool_fees_base: c_stats.pool_fees_base,
            cumulative_stats_pool_fees_quote: c_stats.pool_fees_quote,
            cumulative_stats_n_swaps: c_stats.n_swaps,
            cumulative_stats_n_chat_messages: c_stats.n_chat_messages,
            instantaneous_stats_total_quote_locked: i_stats.total_quote_locked,
            instantaneous_stats_total_value_locked: i_stats.total_value_locked,
            instantaneous_stats_market_cap: i_stats.market_cap,
            instantaneous_stats_fully_diluted_value: i_stats.fully_diluted_value,
        }
    }
}
