//! Wire-shape regression tests for the 2026-10-03 API contract parity pass.
//! Every sample mirrors what the route in `src/app/api/v1/**` actually emits
//! (including the null / absent cases), not the docs.

use madeonsol::types::*;
use serde_json::json;

fn de<T: serde::de::DeserializeOwned>(v: serde_json::Value) -> T {
    serde_json::from_value(v).expect("route-shaped sample must deserialize")
}

#[test]
fn alpha_linked_uses_linked_key() {
    // The route sends `linked` and no `total`; the old struct required
    // `linked_wallets` + `total` and never deserialized.
    let r: AlphaLinkedResponse = de(json!({
        "wallet": "W1",
        "linked": [{ "wallet_address": "W2", "shared_tokens": 4, "avg_time_diff_secs": 12.5,
                     "avg_sol_diff": 0.003, "similarity_score": 0.8123 }]
    }));
    assert_eq!(r.linked_wallets.len(), 1);
    assert_eq!(r.linked_wallets[0].avg_time_diff_secs, Some(12.5));
    assert!(r.total.is_none());
}

#[test]
fn deployer_stats_mc_maps_and_failed_aggregate() {
    let ok: DeployerStats = de(json!({
        "tracked_count": 10, "signals_today": 2, "bonds_detected": 3, "bond_rate": 12.5,
        "tiers": { "elite": 1, "good": 2, "rising": 3 },
        "avg_mc_at_alert_usd_30d": { "elite": 15000, "good": null, "rising": 9000 },
        "mc_at_alert_samples_30d": { "elite": 4, "good": 0, "rising": 7 },
        "mc_at_alert_window_start": "2026-09-03T00:00:00.000Z",
        "mc_at_alert_complete": true
    }));
    assert_eq!(ok.avg_mc_at_alert_usd_30d.unwrap()["good"], None);
    let failed: DeployerStats = de(json!({
        "tracked_count": 10, "signals_today": 2, "bonds_detected": 3, "bond_rate": 12.5,
        "tiers": { "elite": 1, "good": 2, "rising": 3 },
        "avg_mc_at_alert_usd_30d": { "elite": null, "good": null, "rising": null },
        "mc_at_alert_samples_30d": { "elite": null, "good": null, "rising": null },
        "mc_at_alert_window_start": "2026-09-03T00:00:00.000Z",
        "mc_at_alert_complete": false
    }));
    assert_eq!(failed.mc_at_alert_complete, Some(false));
    assert_eq!(failed.mc_at_alert_samples_30d.unwrap()["elite"], None);
}

#[test]
fn deployer_tokens_route_shape() {
    let r: DeployerTokensResponse = de(json!({
        "tokens": [{ "id": "6f1c0c1e-0000-4000-8000-000000000000", "token_mint": "M1", "token_name": "A",
                     "token_symbol": "A", "deployed_at": "2026-10-01T00:00:00Z", "bonded_at": null,
                     "time_to_bond_minutes": null, "peak_market_cap": 1234.5, "mc_at_bond": null,
                     "market_cap_at_alert": null, "alerted_at": null, "instant_bond": false }],
        "total": 1, "limit": 50, "offset": 0, "has_more": false
    }));
    assert_eq!(r.total, 1);
    assert_eq!(r.tokens[0].instant_bond, Some(false));
}

#[test]
fn deployer_profile_pump_tokens_typed() {
    let r: DeployerProfileResponse = de(json!({
        "is_deployer": true, "wallet": "W",
        "pump_tokens": [{ "mint": "M", "name": "N", "symbol": "S", "image_uri": "", "creator": "W",
                          "created_timestamp": 1759449600000u64, "complete": true, "ath_market_cap": 90000,
                          "usd_market_cap": 30000, "market_cap": 180, "reply_count": 3,
                          "pump_swap_pool": "P" }]
    }));
    assert_eq!(r.pump_tokens[0].pump_swap_pool.as_deref(), Some("P"));
    assert!(r.pump_tokens[0].pool_address.is_none());
}

#[test]
fn kol_token_and_profile_route_shapes() {
    let t: KolTokenActivity = de(json!({
        "token_mint": "M",
        "summary": { "kol_count": 1, "total_bought_sol": 2.5, "total_sold_sol": 1.0,
                     "net_flow_sol": 1.5, "signal": "accumulating" },
        "kols": [{ "name": "k", "wallet": "W", "buy_count": 2, "sell_count": 1, "total_bought": 2.5,
                   "total_sold": 1.0, "net_sol": -1.5, "position": "net_buyer",
                   "first_trade": "2026-10-01T00:00:00Z", "last_trade": "2026-10-02T00:00:00Z" }]
    }));
    assert_eq!(t.kols[0].total_bought, 2.5);

    let p: KolWalletProfile = de(json!({
        "kol": { "name": "k", "wallet": "W", "twitter_url": null, "strategy_tag": null, "auto_strategy_tag": null },
        "stats": { "pnl": 1.0, "buy_count": 3, "sell_count": 1, "volume": 5.0 },
        "scores": { "winrate_7d": null, "closed_positions_7d": 0, "is_cold": false },
        "peer_ranks": { "percentile_pnl_7d": 91.2, "percentile_winrate_7d": null,
                        "percentile_pnl_30d": null, "percentile_winrate_30d": null,
                        "percentile_early_entry_30d": null },
        "recent_trades": [],
        "pnl_by_token": [{ "token_mint": "M", "token_symbol": null, "token_name": null, "buy_count": 1,
                           "sell_count": 0, "total_bought": 1.0, "total_sold": 0, "pnl": -1.0,
                           "result": "open", "first_trade": "x", "last_trade": "y" }]
    }));
    assert!(p.stats.win_rate.is_none());
    assert_eq!(p.peer_ranks.unwrap().percentile_pnl_7d, Some(91.2));
    assert_eq!(p.pnl_by_token.unwrap()[0].total_sold, Some(0.0));
}

#[test]
fn kol_pnl_positions_and_compare_meta() {
    let c: KolClosedPosition = de(json!({
        "token_mint": "M", "token_symbol": "S", "token_name": "N", "buy_count": 1, "sell_count": 1,
        "bought_sol": 1.0, "sold_sol": 0.5, "pnl_sol": -0.5, "roi_pct": -50.0, "still_holding": true,
        "held_value_sol": 0.2, "unrealized_pnl_sol": 0.1, "hold_minutes": 3.0, "result": "loss",
        "first_trade": "a", "last_trade": "b"
    }));
    assert_eq!(c.still_holding, Some(true));
    let o: KolOpenPosition = de(json!({
        "token_mint": "M", "token_symbol": "S", "token_name": "N", "buy_count": 1, "bought_sol": 1.0,
        "held_value_sol": 0, "unrealized_pnl_sol": 0, "is_priced": false, "first_buy_at": "a"
    }));
    assert_eq!(o.is_priced, Some(false));
    let m: KolCompareResponse = de(json!({
        "profiles": [], "overlap": [],
        "overlap_meta": { "window_start": "2026-09-03T00:00:00Z", "min_wallets": 2, "total": null,
                          "returned": 0, "has_more": null, "complete": false }
    }));
    assert!(!m.overlap_meta.unwrap().complete);
}

#[test]
fn scouts_history_webhook_test_typed() {
    let s: KolScoutLeaderboardResponse = de(json!({
        "scouts": [{ "wallet": "W", "name": "k", "avatar_url": null, "twitter_url": null, "scout_tier": "A",
                     "first_touches_30d": 12, "avg_followers_4h": 3.5, "swarm_3plus_pct": 40.0,
                     "swarm_5plus_pct": 10.0, "computed_at": "2026-10-03T00:00:00Z" }],
        "count": 1
    }));
    assert_eq!(s.scouts[0].first_touches_30d, Some(12));
    let h: KolCoordinationHistoryResponse = de(json!({
        "events": [{ "token_mint": "M", "fired_at": "t", "coordination_score": 77, "total_fires": 3,
                     "kol_buyers": 4, "current_mc_usd": null, "current_price_usd": null }],
        "count": 1
    }));
    assert_eq!(h.events[0].total_fires, 3);
    let w: WebhookTestResponse = de(json!({ "success": false, "error": "timeout", "response_time_ms": 0 }));
    assert!(!w.success);
}

#[test]
fn token_batch_item_degraded_and_full() {
    let r: TokenBatchResponse = de(json!({
        "tokens": [{
            "mint": "M", "price_usd": null, "price_sol": null, "price_source": null, "price_observed_at": null,
            "price_age_seconds": null, "price_is_stale": null, "market_cap": null,
            "vwap_price_usd": null, "vwap_price_sol": null, "primary_pool_address": null,
            "token_supply_burn_detected": null, "burn_detected": null, "lp_burn_status": "unknown",
            "deployer": null,
            "deployer_identity": { "identity_status": "lookup_failed", "history_status": null, "address": null, "source": null },
            "kol_activity": { "buying_kols": null, "selling_kols": null, "net_flow_sol": null, "signal": null,
                              "top_buyers": [], "window_hours": 168, "complete": false },
            "mc_change_pct": { "5m": 1.5, "15m": null }, "volume_usd": { "5m": 100.0 },
            "mev_volume_pct": { "5m": null }, "history_age_seconds": 600
        }],
        "count": 1, "as_of": "2026-10-03T00:00:00Z"
    }));
    let t = &r.tokens[0];
    assert_eq!(t.kol_activity.complete, Some(false));
    assert_eq!(t.deployer_identity.as_ref().unwrap().identity_status, "lookup_failed");
    assert_eq!(t.mc_change_pct.as_ref().unwrap()["15m"], None);
}

#[test]
fn risk_assessment_resolved_from_and_batch_errors() {
    let r: TokenRisk = de(json!({
        "mint": "M", "risk_score": 40, "band": "caution", "factors": [], "inputs": {},
        "score_version": "v3", "as_of": "t", "dev_status": "unavailable",
        "assessment": { "status": "incomplete", "unknown_inputs": ["holders"], "not_assessed": [],
                        "explanations": { "holders": "lookup failed" } },
        "resolved_from": { "address": "P", "kind": "pool", "dex": "pumpswap", "source": "token_pools" }
    }));
    assert_eq!(r.assessment.unwrap().status, "incomplete");
    assert_eq!(r.resolved_from.unwrap().kind, "pool");
    let b: BatchRiskResponse = de(json!({
        "tokens": [
            { "mint": "A", "error": "unavailable", "code": "risk_inputs_unavailable",
              "unavailable_inputs": ["supply"], "retryable": true },
            { "mint": "B", "error": "error", "retryable": false }
        ],
        "count": 2
    }));
    assert_eq!(b.tokens[0].unavailable_inputs.as_deref(), Some(&["supply".to_string()][..]));
    assert_eq!(b.tokens[1].retryable, Some(false));
}

#[test]
fn candles_outside_plan_and_tracker_events() {
    let c: CandlesResponse = de(json!({
        "mint": "M", "timeframe": "1m", "from": "a", "to": "b", "count": 0, "net_flow_included": false,
        "truncated": false, "covered_from": "a", "history_floor": "f", "history_clamped": false,
        "history_outside_plan": true, "candles": []
    }));
    assert_eq!(c.history_outside_plan, Some(true));
    let t: WalletTrackerTradesResponse = de(json!({
        "events": [{ "wallet_address": "W", "label": null, "event_type": "swap", "action": null,
                     "token_mint": null, "token_symbol": null, "token_name": null, "sol_amount": null,
                     "token_amount": null, "tx_signature": "sig", "block_time": 1759449600, "slot": null,
                     "replayed": false, "ingested_at": "2026-10-03T00:00:00.000Z",
                     "timestamp": "2026-10-03T00:00:00.000Z" }],
        "count": 1, "ordered_by": "slot", "next_cursor": null, "next_cursor_slot": null
    }));
    assert_eq!(t.events[0].replayed, Some(false));
    assert!(t.events[0].id.is_none() && t.events[0].action.is_none());
}

#[test]
fn copytrade_rule_signal_and_delete() {
    let r: CopytradeSubscriptionResponse = de(json!({
        "subscription": { "id": 152, "name": null, "source_wallets": ["A", "B"], "min_trade_sol": 0,
                          "only_action": "buy", "sizing_mode": "fixed", "sizing_amount": 0.1,
                          "delivery_mode": "webhook", "webhook_url": "https://x", "min_mc_usd": null,
                          "max_mc_usd": null, "is_active": true, "created_at": "t",
                          "source_wallets_tracked": ["A"], "source_wallets_untracked": ["B"],
                          "warnings": [{ "code": "untracked_source_wallets", "message": "m" }] },
        "warnings": [{ "code": "untracked_source_wallets", "message": "m" }],
        "webhook_secret": "abc", "note": "Save the webhook_secret"
    }));
    assert_eq!(r.subscription.source_wallets_untracked.as_deref(), Some(&["B".to_string()][..]));
    let l: CopytradeSubscriptionListResponse = de(json!({ "subscriptions": [{ "id": 1, "source_wallets": [],
        "source_wallets_tracked": null, "source_wallets_untracked": null,
        "warnings": [{ "code": "tracking_status_unavailable", "message": "m" }] }] }));
    assert!(l.subscriptions[0].source_wallets_tracked.is_none());
    let s: CopytradeSignalsResponse = de(json!({ "signals": [{
        "id": 9, "subscription_id": 152, "fired_at": "t", "source_wallet": "A", "action": "buy",
        "token_mint": "M", "token_symbol": null, "token_name": null, "source_sol_amount": 1.2,
        "suggested_sol_amount": 0.1, "tx_signature": "sig", "delivered": true, "delivered_at": "t",
        "market_cap_usd_at_trade": null, "price_usd_at_trade": null, "market_cap_usd": 5000.0,
        "last_price_usd": 0.000005, "mc_change_pct": { "5m": 2.0 }, "volume_usd": { "5m": 10.0 },
        "mev_volume_pct": { "5m": null }, "history_age_seconds": 300 }] }));
    assert_eq!(s.signals[0].subscription_id, 152);
    let d: CopytradeSubscriptionDeleteResponse = de(json!({ "deleted": true }));
    assert!(d.deleted);
    let body = serde_json::to_value(CopytradeSubscriptionUpdateParams { is_active: Some(false), ..Default::default() }).unwrap();
    assert_eq!(body, json!({ "is_active": false }));
}

#[test]
fn roster_manifests_search_top_traders() {
    let k: KolWalletsResponse = de(json!({
        "wallets": [{ "wallet_address": "W", "name": "k", "twitter_url": null, "avatar_url": null,
                      "strategy_tag": null, "twitter_followers": null, "follow_count": 0, "is_active": true,
                      "tracked_since": "2025-01-01T00:00:00Z" }],
        "count": 1, "total": 900, "limit": 200, "offset": 0, "has_more": true
    }));
    assert!(k.has_more);
    let m: ManifestsResponse = de(json!({
        "manifests": [{ "dataset": "token_trades", "data_as_of": "t", "produced_at": "t", "producer": "p",
                        "ts_column": "block_time", "rows_24h": 123456, "min_ts": "a", "max_ts": "b",
                        "row_count_estimate": 987654321, "schema_hash": "h", "fingerprint": "f" }],
        "count": 1, "dataset": null, "methodology_version": "2026-09", "note": "n"
    }));
    assert_eq!(m.manifests[0].rows_24h, Some(123456));
    let s: TokenSearchResponse = de(json!({
        "q": "bonk", "count": 1,
        "results": [{ "mint": "M", "symbol": "BONK", "name": "Bonk", "match": "symbol_exact",
                      "market_cap_usd": null, "liquidity_usd": null, "primary_dex": null, "last_trade_at": null,
                      "image_url": null, "twitter": null, "website": null }]
    }));
    assert_eq!(s.results[0].match_kind.as_deref(), Some("symbol_exact"));
    let t: TopTradersResponse = de(json!({
        "mint": "M", "sort": "pnl", "window_days": 30,
        "traders": [{ "rank": 1, "wallet": "W", "trades": 3, "buys": 2, "sells": 1, "bought_sol": 1.0,
                      "sold_sol": 2.0, "realized_pnl_sol": 1.0, "unrealized_pnl_sol": 0.0, "total_pnl_sol": 1.0,
                      "held_value_sol": 0.0, "roi": null, "still_holding": false, "first_trade_at": "a",
                      "last_trade_at": "b", "is_kol": false, "kol_name": null, "is_alpha_tracked": false,
                      "bot_confidence": null, "historical_win_rate": null, "historical_pnl_sol": null,
                      "historical_tokens": null }],
        "summary": { "returned": 1, "known_kols": 0, "known_alpha_wallets": 0, "net_realized_pnl_sol": 1.0 },
        "coverage": null
    }));
    assert_eq!(t.traders[0].rank, 1);
}

#[test]
fn wallet_batch_flags_funding_list_score() {
    let b: WalletBatchTradesResponse = de(json!({
        "wallets": [{ "wallet": "W", "count": 1, "trades": [{ "tx_signature": "s", "token_mint": "M",
            "action": "buy", "sol_amount": 1.0, "token_amount": 1000.0, "price_sol": 0.001, "price_usd": null,
            "market_price_sol": null, "market_price_usd": null, "block_time": 1759449600,
            "traded_at": "2026-10-03T00:00:00.000Z" }] }],
        "since": 1751673600, "next_since": 1759449600, "limit_per_wallet": 20,
        "coverage": { "history_start_days": 90, "scope": "pump.fun pipeline", "note": "n" }
    }));
    assert_eq!(b.next_since, 1759449600);
    let f: WalletFlagsResponse = de(json!({
        "wallet": "W", "as_of": "t", "flagged": ["sniper"],
        "sources": { "sniper": { "member": true, "score": 3, "snapshot_at": "t", "active": true, "carried": true },
                     "kol": null },
        "history": [{ "source": "sniper", "flags": { "member": true }, "snapshot_at": "t" }],
        "note": "n"
    }));
    let sniper = f.sources["sniper"].as_ref().unwrap();
    assert!(sniper.carried && sniper.flags.contains_key("score"));
    assert!(f.sources["kol"].is_none());
    let fu: WalletFundingResponse = de(json!({
        "chain": "solana", "chain_id": "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp", "native_asset": "SOL",
        "address": "W", "status": "ok", "summary": "s",
        "shared_funders": [{ "funder": "F", "funder_explorer_url": "u", "funder_label": null, "service_funder": false,
            "to_this_wallet": [{ "asset": "native", "symbol": "SOL", "decimals": 9, "amount_raw": "1500000000",
                "amount": "1.5", "transfer_count": 1, "first_seen": "a", "last_seen": "a",
                "transactions": [{ "tx": "sig", "explorer_url": "u" }] }],
            "connected_wallets": [{ "address": "W2", "explorer_url": "u", "tracked_as": ["kol"], "transfers": [] }] }],
        "pagination": { "limit": 10, "offset": 0, "total": 1, "has_more": false },
        "coverage": { "collection_enabled": true, "mode": "on" },
        "disclaimer": "d"
    }));
    assert_eq!(fu.shared_funders[0].to_this_wallet[0].amount_raw, "1500000000");
    assert!(fu.direct_funding.is_none());
    let s: WalletListScoreResponse = de(json!({
        "wallets": [
            { "address": "A", "status": "scored", "score": 61.5, "pnl": { "total_pnl_sol": 1.0 },
              "reputation": { "is_sniper": false, "is_bundler": false, "is_dumper": false, "is_kol": false,
                              "kol_name": null, "bot_confidence": null },
              "cache_hit": true, "computed_at": "t", "cache_age_seconds": 30 },
            { "address": "B", "status": "not_computed", "message": "cap", "score": null, "pnl": null,
              "reputation": { "is_sniper": true, "is_bundler": false, "is_dumper": false, "is_kol": false,
                              "kol_name": null, "bot_confidence": null },
              "cache_hit": null, "computed_at": null }
        ],
        "count": 2, "scored": 1, "cached": 1, "computed_now": 0, "no_trades": 0, "not_computed": 1,
        "max_wallets": 200, "max_live_compute": 25, "score_methodology": "m", "as_of": "t"
    }));
    assert_eq!(s.wallets[1].status, "not_computed");
}

#[test]
fn deployer_activity_pro_and_ultra_shapes() {
    let fam = json!({ "source": "token_trades", "retention": "r", "scope": "s", "truncated": false, "loaded": true,
        "complete": true, "complete_from": "2026-09-04T12:00:00.000Z", "archive_required_before": null, "boundary_known": true });
    let pro: DeployerActivityResponse = de(json!({
        "wallet": "W", "is_deployer": true,
        "deployer": { "tier": "cold", "first_deploy_at": "2026-01-01T00:00:00Z", "last_deploy_at": null },
        "plan": { "entitlement": "pro", "window_days": 30, "max_limit": 100, "history": {
            "requested": { "from": "2026-09-04T12:00:00.000Z", "to": "2026-10-04T12:00:00.000Z", "source": "plan_default" },
            "effective": { "from": "2026-09-04T12:00:00.000Z", "to": "2026-10-04T12:00:00.000Z", "clamped": false, "max_days": 30 },
            "online": { "from": "2026-09-04T12:00:00.000Z", "to": "2026-10-04T12:00:00.000Z", "served": true },
            "archive_only": null } },
        "window": { "since": "2026-09-04T12:00:00.000Z", "until": "2026-10-04T12:00:00.000Z", "max_days": 30, "applies_to": "event_time" },
        "events": [
            { "id": "launch:M", "type": "launch", "at": "2026-10-03T10:00:00Z", "time_basis": "ingest", "mint": "M", "name": null, "symbol": "ONE",
              "launchpad": "pumpfun", "bonded_at": null, "fee_payer_is_creator": false, "external_fee_payer": true,
              "dev_buy_sol": 1.5, "dev_buy_tokens": 1000, "dev_buy_supply_pct": null },
            { "id": "dev_trade:s:M:sell", "type": "dev_sell", "at": "2026-10-03T11:00:00.000Z", "time_basis": "ingest", "mint": "M",
              "own_token": true, "sol": 2.5, "tokens": 5000, "price_usd": null, "tx": "s" },
            { "id": "creator_transferred:t:0", "type": "creator_transferred", "at": "2026-10-02T12:00:00Z", "time_basis": "chain", "mint": "M",
              "from": "W", "to": "C", "direction": "out", "initiated_by": "creator", "tx": "t" },
            { "id": "capital_out:E:native", "type": "capital_out", "at": "2026-10-01T13:00:00Z", "time_basis": "chain", "recipient": "E",
              "asset": "native", "amount_raw": "1", "decimals": 9, "transfer_count": 1, "first_at": "2026-10-01T13:00:00Z",
              "last_at": "2026-10-01T13:00:00Z", "first_tx": null, "aggregate": true },
            { "id": "dev_token_transfer:x:1:M:out", "type": "dev_token_transfer_out", "at": "2026-10-04T10:00:00Z", "time_basis": "chain",
              "mint": "M", "counterparty": "E", "token_amount_raw": "777" }
        ],
        "pagination": { "limit": 100, "requested_limit": 5000, "limit_capped": true, "next_cursor": null, "has_more": false },
        "coverage": { "status": "observed", "families": { "dev_trades": fam.clone() }, "future_events_dropped": 0, "note": "n" }
    }));
    assert!(pro.identity.is_none());
    assert_eq!(pro.events[0].external_fee_payer, Some(true));
    assert_eq!(pro.events[2].from.as_deref(), Some("W"));
    assert_eq!(pro.events[3].recipient.as_deref(), Some("E"));
    assert_eq!(pro.events[4].extra["counterparty"], "E"); // additive event types never break
    assert!(pro.plan.history.archive_only.is_none());

    let mut skipped = fam.clone();
    skipped["skipped_reason"] = json!("no_attributed_launch");
    skipped["complete"] = json!(false);
    let ultra: DeployerActivityResponse = de(json!({
        "wallet": "N", "is_deployer": false, "deployer": null,
        "plan": { "entitlement": "ultra", "window_days": 365, "max_limit": 100, "history": {
            "requested": { "from": "2025-10-04T12:00:00.000Z", "to": "2026-10-04T12:00:00.000Z", "source": "plan_default" },
            "effective": { "from": "2025-10-04T12:00:00.000Z", "to": "2026-10-04T12:00:00.000Z", "clamped": false, "max_days": 365 },
            "online": { "from": "2026-07-01T00:00:00.000Z", "to": "2026-10-04T12:00:00.000Z", "served": true },
            "archive_only": { "from": "2026-04-01T00:00:00.000Z", "to": "2026-07-01T00:00:00.000Z", "served": false, "reason": "archive_reads_not_enabled" } } },
        "window": { "since": "2025-10-04T12:00:00.000Z", "until": "2026-10-04T12:00:00.000Z", "max_days": 365, "applies_to": "event_time" },
        "events": [],
        "pagination": { "limit": 50, "requested_limit": 50, "limit_capped": false, "next_cursor": null, "has_more": false },
        "coverage": { "status": "partial", "families": { "dev_trades": skipped }, "future_events_dropped": 0, "note": "n" },
        "identity": { "status": "not_available", "reason": "identity_stitching_not_released", "note": "n" }
    }));
    assert_eq!(ultra.identity.unwrap().status, "not_available");
    assert_eq!(ultra.plan.history.archive_only.unwrap().served, Some(false));
    assert_eq!(ultra.coverage.families["dev_trades"].skipped_reason.as_deref(), Some("no_attributed_launch"));
}

#[test]
fn copytrade_any_wallet_operational_state() {
    // Server 2026-10-04: any-wallet admission + operational state on every rule.
    let ok: CopytradeSubscription = de(json!({
        "id": 7, "name": null, "source_wallets": ["W1", "W2"], "min_trade_sol": 0, "only_action": "buy",
        "sizing_mode": "fixed", "sizing_amount": 0.1, "delivery_mode": "websocket", "webhook_url": null,
        "min_mc_usd": null, "max_mc_usd": null, "is_active": true, "created_at": "t", "updated_at": "t",
        "source_wallets_tracked": ["W1"], "source_wallets_untracked": ["W2"],
        "operational_state": "eligible", "source_admission": "any_wallet"
    }));
    assert_eq!(ok.source_admission.as_deref(), Some("any_wallet"));
    assert_eq!(ok.operational_state.as_deref(), Some("eligible"));
    assert!(ok.monitoring_reasons.is_none());
    let down: CopytradeSubscription = de(json!({
        "id": 8, "source_wallets": ["W3"], "operational_state": "monitoring_unavailable",
        "source_admission": "any_wallet", "monitoring_reasons": ["trade_stream_stale"]
    }));
    assert_eq!(down.monitoring_reasons.unwrap(), vec!["trade_stream_stale".to_string()]);
}

#[test]
fn token_lock_sablier_holder_and_smithii_untracked_withdrawals() {
    let base = |program: &str| json!({
        "lock_account": "L", "program": program, "kind": "vesting", "status": "active", "mint": "M",
        "sender": "S", "recipient": null, "name": null, "amount_raw": "100", "amount": null, "amount_usd": null,
        "amount_pct_of_supply": null, "locked_raw": "60", "locked": null, "locked_usd": null,
        "locked_pct_of_supply": null, "unlocked_raw": "40", "unlocked": null,
        "withdrawn": null, "claimable": null, "start_at": null, "cliff_at": null, "end_at": null,
        "period_seconds": null, "continuous": false, "amount_per_period_raw": null, "amount_per_period": null,
        "cliff_amount_raw": null, "cliff_amount": null, "perpetual": false, "next_unlock": null,
        "cancelable_by_sender": null, "cancelable_by_recipient": null, "transferable": null, "can_topup": null,
        "cancelled_at": null, "created_at": null, "created_at_estimated": false, "tx_signature": null
    });
    let mut sablier = base("sablier_lockup");
    let o = sablier.as_object_mut().unwrap();
    o.insert("holder_status".into(), json!("lost_proof"));
    o.insert("last_proven_holder".into(), json!("H"));
    o.insert("holder_proven_at_slot".into(), json!(312000000));
    o.insert("withdrawn_raw".into(), json!("10"));
    o.insert("withdrawn_tracked".into(), json!(true));
    o.insert("claimable_raw".into(), json!("30"));
    let s: TokenLock = de(sablier);
    assert!(matches!(s.program, LockProgram::SablierLockup));
    assert!(s.recipient.is_none());
    assert_eq!(s.holder_status.as_deref(), Some("lost_proof"));
    assert_eq!(s.last_proven_holder.as_deref(), Some("H"));
    assert_eq!(s.holder_proven_at_slot, Some(312000000));

    // Smithii: withdrawals are not exposed — null raw strings, never zero.
    let mut smithii = base("smithii_vesting");
    let o = smithii.as_object_mut().unwrap();
    o.insert("withdrawn_raw".into(), json!(null));
    o.insert("withdrawn_tracked".into(), json!(false));
    o.insert("claimable_raw".into(), json!(null));
    o.insert("holder_status".into(), json!(null));
    let m: TokenLock = de(smithii);
    assert!(matches!(m.program, LockProgram::SmithiiVesting));
    assert_eq!(m.withdrawn_tracked, Some(false));
    assert!(m.withdrawn_raw.is_none() && m.claimable_raw.is_none());
}
