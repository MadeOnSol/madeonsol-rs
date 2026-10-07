use madeonsol::types::*;
use serde_json::{json, Value};

fn de<T: serde::de::DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).expect("contract response must deserialize")
}

#[test]
fn early_observations_keep_identity_nullable_requests_and_old_responses() {
    let mut row = json!({"mint":"M", "name":null, "symbol":null, "deployer_wallet":"D",
        "signature":"S", "slot":100, "detected_at":"t", "detection_region":"AA",
        "event_id":"solana:S:1:create", "source":"shredprism", "outer_instruction_index":1,
        "observation_stage":"observed", "execution_status":"unknown", "fee_payer":null,
        "transaction_version":1, "transaction_config":{"config_mask":1,
            "priority_fee_lamports":"18446744073709551615", "compute_unit_limit":null,
            "loaded_accounts_data_size_limit":null, "heap_size":null}});
    let response: SniperRecentResponse = de(json!({"deploys":[row.clone()], "count":1, "data_age_seconds":null}));
    let event = &response.deploys[0];
    assert_eq!(event.event_id.as_deref(), Some("solana:S:1:create"));
    assert_eq!(event.execution_status.as_deref(), Some("unknown"));
    assert_eq!(event.outer_instruction_index, Some(1));
    assert!(matches!(event.transaction_version, Some(EarlyTransactionVersion::Number(1))));
    let config = event.transaction_config.as_ref().unwrap();
    assert_eq!(config.priority_fee_lamports.as_deref(), Some("18446744073709551615"));
    assert_eq!(config.compute_unit_limit, None);
    row["transaction_version"] = json!("legacy"); row["transaction_config"] = Value::Null;
    let legacy: SniperDeploy = de(row.clone());
    assert!(matches!(legacy.transaction_version, Some(EarlyTransactionVersion::Named(ref v)) if v == "legacy"));
    row.as_object_mut().unwrap().remove("transaction_version");
    assert!(de::<SniperDeploy>(row).transaction_version.is_none());
    let token: StreamToken = de(json!({"token":"T", "ws_url":"wss://old", "usage":"n",
        "early_ws_url":"wss://early", "early_stream":{"channels":["early:deploys"],
        "subscribe_example":{"type":"subscribe", "channels":["early:deploys"]},
        "execution_status":"unknown", "note":"Observed intent"}}));
    assert_eq!(token.early_ws_url.as_deref(), Some("wss://early"));
    assert_eq!(token.early_stream.unwrap().subscribe_example.frame_type, "subscribe");
    assert!(de::<StreamToken>(json!({"token":"T", "ws_url":"wss://old", "usage":"n"})).early_stream.is_none());
}

#[test]
fn concentrated_depth_preserves_unquotable_nulls_and_pool_selection() {
    let depth: TokenDepthResponse = de(json!({"mint":"M", "found":true, "sizes_sol":[1],
        "pool_selection":{"largest_known_pool":"P", "largest_known_pool_supported":true, "primary_pool":"P", "routing":"single_pool_only"},
        "unsupported_pools":[], "pools":[{"pool_address":"P", "dex":"orca", "quote_mint":"Q",
        "pool_model":"concentrated", "is_active":true, "depth_available":true,
        "model":"concentrated", "model_detail":"raydium_clmm_ticks", "pool_account":"A",
        "fee_pct":0.25, "fee_basis":"amm_config_at_quote_time", "source":"live_rpc",
        "reserves_age_ms":0, "spot_price_sol":1,
        "ticks_window":{"from_tick":-10, "to_tick":10, "slot":100},
        "bins_window":{"from_bin":-2, "to_bin":2, "slot":100},
        "quotes":[{"size_sol":1, "status":"exceeds_loaded_ticks", "tokens_out":null, "avg_price_sol":null, "price_impact_pct":null}],
        "to_move_price":{"1pct":null, "5pct":null, "10pct":null}}]}));
    let pool = &depth.pools[0];
    assert_eq!(pool.quotes[0].tokens_out, None);
    assert_eq!(pool.to_move_price.pct_1, None);
    assert_eq!(pool.ticks_window.as_ref().unwrap().from_tick, -10);
    assert_eq!(pool.bins_window.as_ref().unwrap().from_bin, -2);
    assert_eq!(depth.pool_selection.unwrap().routing, "single_pool_only");
    let quote: DepthQuote = de(json!({"size_sol":1, "tokens_out":2, "avg_price_sol":0.5, "price_impact_pct":1}));
    assert_eq!(quote.tokens_out, Some(2.0)); assert!(quote.status.is_none());
}

#[test]
fn holdings_labels_funding_and_identity_bind_their_actual_response_fields() {
    let positions: WalletPositionsResponse = de(json!({"address":"W", "positions":[{
        "token_mint":"M", "token_amount":1, "cost_basis_sol":2, "avg_entry_price_sol":2,
        "buys_in_position":1, "position_basis":"swap_derived", "holding_status":"unverified",
        "holding_unverified_reason":"not_checked", "cost_basis_status":"known", "holding":null}],
        "holding_check":{"mode":"off", "source":null, "verified_at":null, "verified":0,
        "unverified":1, "held":0, "partially_reduced":0, "transferred_or_disposed":0, "external_inflow":0, "note":"unverified"}}));
    assert_eq!(positions.positions[0].holding_status.as_deref(), Some("unverified"));
    assert_eq!(positions.holding_check.unwrap().unverified, 1);
    let labels: WalletBatchClassifyResponse = de(json!({"count":1, "as_of":"t", "rule_version":"v1",
        "evidence_horizon":{"sniper":"launch scoped"}, "wallets":[{"address":"W", "is_sniper":false,
        "is_bundler":false, "is_dumper":false, "is_kol":false,
        "label_coverage":{"sniper":{"evaluated":false, "reason":"not_checked_budget"}}}]}));
    assert!(!labels.wallets[0].label_coverage.as_ref().unwrap()["sniper"].evaluated);
    assert_eq!(labels.rule_version.as_deref(), Some("v1"));
    let funding: WalletFundingResponse = de(json!({"chain":"solana", "chain_id":"solana:mainnet",
        "native_asset":"SOL", "address":"W", "status":"not_tracked", "summary":"n", "shared_funders":[],
        "pagination":{"limit":20,"offset":0,"total":0,"has_more":false}, "coverage":{}, "disclaimer":"n",
        "wallet_coverage":{"state":"not_tracked", "currently_tracked":false, "ever_tracked":false,
        "address_kind":null, "limitations":["spl_token_transfers_not_visible"]}}));
    assert_eq!(funding.wallet_coverage.unwrap().state, "not_tracked");
    let signal: CopytradeSignal = de(json!({"id":1,"subscription_id":2,"fired_at":"t","source_wallet":"W",
        "action":"buy","token_mint":"M", "economic_action_id":"A", "identity_version":2,
        "source_actor":"W", "co_actors":["V"]}));
    assert_eq!(signal.economic_action_id.as_deref(), Some("A"));
    assert_eq!(signal.co_actors.unwrap(), vec!["V"]);
}

#[test]
fn liquidity_venue_and_rank_evidence_survive_deserialization() {
    let token: TokenSummary = de(json!({"mint":"M", "authorities_revoked":false, "is_token_2022":false,
        "lp_secured_pct":50, "lp_secured_basis":"temporary", "lp_locked_until":"2030-01-01T00:00:00Z"}));
    assert_eq!(token.lp_secured_pct, Some(50.0));
    assert_eq!(token.lp_secured_basis.as_deref(), Some("temporary"));
    let almost: AlmostBondedToken = de(json!({"mint":"M", "progress_pct":90, "stalled":false,
        "authorities_revoked":false, "venue_source":"token_pools"}));
    assert_eq!(almost.venue_source.as_deref(), Some("token_pools"));
    let cap: AlphaCapTableResponse = de(json!({"mint":"M", "buyers":[], "summary":{
        "known_alpha_wallets":0, "known_kols":0, "bundle_buyers":0, "buyer_quality_score":0,
        "confidence":"low", "signal":"unknown"}, "ranks_completeness":{
        "ranks_complete":"gap_overlap", "rank_basis":"first_persisted_buys_at_or_above_floor",
        "window":null,"gaps_overlapping":1,"open_slots":0,"gaps_in_open_slots":0}}));
    assert_eq!(cap.ranks_completeness.unwrap().ranks_complete, "gap_overlap");
}
