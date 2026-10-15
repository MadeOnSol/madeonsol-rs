//! Wire-shape tests for the 0.34 terminal-gap pass (contract 1bba8efe27bc,
//! 2026-10-15): the three new operations and the opt-in fields of existing
//! responses, including their absent / null cases.

use madeonsol::types::*;
use serde_json::{json, Value};

fn de<T: serde::de::DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).expect("route-shaped sample must deserialize")
}

fn held_now() -> Value {
    json!({"source":"solana_rpc_getTokenAccountsByOwner", "checked_at":"t", "wallets":3,
        "verified_wallets":3, "holding_wallets":2, "complete":true, "incomplete_reason":null,
        "slot_min":100, "slot_max":101, "tokens_held_raw":"18446744073709551615",
        "tokens_held":"18446744073709.551615", "held_pct_of_supply":0.12,
        "supply_basis":"circulating_supply_raw"})
}

fn balance_now() -> Value {
    json!({"amount_raw":"1000000", "decimals":6, "amount":"1", "holding":true, "token_accounts":1, "slot":100})
}

#[test]
fn params_serialize_only_set_opt_ins() {
    let q = serde_json::to_value(TokenBundlesParams {
        include_balances: Some(true),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(q, json!({"include_balances": true}));
    assert_eq!(
        serde_json::to_value(TokenBundleShareParams::default()).unwrap(),
        json!({})
    );
    let q = serde_json::to_value(CapTableParams {
        verify_current_balances: Some(true),
        include_scout_scores: Some(true),
        limit: None,
    })
    .unwrap();
    assert_eq!(
        q,
        json!({"verify_current_balances": true, "include_scout_scores": true})
    );
    let q = serde_json::to_value(TokenHoldersParams {
        include_wallet_flags: Some(true),
    })
    .unwrap();
    assert_eq!(q, json!({"include_wallet_flags": true}));
    let q = serde_json::to_value(BuyerQualityParams {
        include_balances: Some(true),
    })
    .unwrap();
    assert_eq!(q, json!({"include_balances": true}));
    let q = serde_json::to_value(TokenBundleParams {
        include_balances: Some(false),
    })
    .unwrap();
    assert_eq!(q, json!({"include_balances": false}));
    let q = serde_json::to_value(DeployerTokensParams {
        include_launches_24h: Some(true),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(q, json!({"include_launches_24h": true}));
    let q = serde_json::to_value(KolTokenTradesParams {
        action: Some("buy".into()),
        cursor: Some("c".into()),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(q, json!({"action": "buy", "cursor": "c"}));
}

#[test]
fn kol_token_trades_shape() {
    let r: KolTokenTradesResponse = de(json!({"token_mint":"M", "trades":[{
        "id":"6f1c0c1e-0000-4000-8000-000000000000", "kol_name":null, "kol_twitter":null, "wallet":"W",
        "action":"sell", "sol_amount":1.5, "token_amount":null, "price_usd_at_trade":null,
        "market_cap_usd_at_trade":null, "tx_signature":"S", "traded_at":"2026-10-15T00:00:00Z"}],
        "count":1, "kols_on_page":1, "next_cursor":null, "has_more":false, "note":"n"}));
    assert_eq!(r.trades[0].action, "sell");
    assert_eq!(r.trades[0].token_amount, None);
    assert!(!r.has_more && r.next_cursor.is_none());
    let empty: KolTokenTradesResponse = de(json!({"token_mint":"M", "trades":[], "count":0,
        "kols_on_page":0, "next_cursor":null, "has_more":false}));
    assert!(empty.trades.is_empty());
}

#[test]
fn bundle_share_and_clusters_shapes() {
    let s: TokenBundleShareResponse = de(json!({"mint":"M", "window":"24h", "min_wallets":3,
        "bundled_pct_of_supply_now":null, "scope":"launch_cohort_only", "clusters_status":"unavailable",
        "wallets":{"total":5, "launch_cohort_wallets":5, "cluster_wallets":0, "overlap":0,
            "clusters_counted":0, "verified":4, "holding":2, "max":60},
        "held_now":{"complete":false, "incomplete_reason":"rpc_error", "tokens_held_raw":null,
            "tokens_held":null, "held_pct_of_supply":null, "supply_basis":null},
        "as_of":"t", "note":"n"}));
    assert_eq!(s.bundled_pct_of_supply_now, None);
    assert_eq!(
        s.held_now.unwrap().incomplete_reason.as_deref(),
        Some("rpc_error")
    );
    assert_eq!(s.wallets.unwrap().verified, Some(4));

    let b: TokenBundlesResponse = de(json!({"mint":"M", "window":"6h", "min_wallets":3,
        "launch_at":"2026-10-15T00:00:00Z",
        "summary":{"cluster_count":2, "launch_clusters":1, "later_clusters":1,
            "latest_cluster_at":"t", "largest_cluster_wallets":7,
            "baseline":{"active_units":10, "buys":40, "buys_per_active_unit":4.0}},
        "clusters":[
            {"slot":101, "atomic_txs":1, "sol_volume":3.2, "token_volume":null, "at":"t",
             "minutes_after_launch":42.5, "phase":"later", "wallet_count":7, "new_wallets":5,
             "wallets":["A","B"], "held_now":held_now(),
             "exit":{"cluster_id":9, "followed_until":"t", "wallets_tracked":7, "wallets_sold":3,
                 "wallets_exited":1, "bought_tokens":1000.0, "sold_tokens":400.0, "sold_pct":0.4,
                 "realized_sol":1.1, "last_sell_at":null}},
            {"slot":100, "atomic_txs":0, "sol_volume":null, "token_volume":null, "at":"t",
             "minutes_after_launch":null, "phase":"unknown", "wallet_count":3, "new_wallets":3,
             "wallets":[], "exit":null}],
        "held_now_clusters":1, "exit_tracking":"ok", "exit_note":"n", "note":"n"}));
    assert_eq!(b.clusters.len(), 2);
    assert_eq!(b.clusters[0].exit.as_ref().unwrap().sold_pct, Some(0.4));
    assert_eq!(
        b.clusters[0]
            .held_now
            .as_ref()
            .unwrap()
            .tokens_held_raw
            .as_deref(),
        Some("18446744073709551615")
    );
    assert!(b.clusters[1].held_now.is_none() && b.clusters[1].exit.is_none());
    assert!(b.summary.unwrap().baseline.is_some());
}

#[test]
fn bundle_held_now_and_wallet_balance() {
    let base = json!({"mint":"M", "bundle":{"wallet_count":2, "bundle_kind":"same_slot",
        "held_ratio":null, "held_pct_of_supply":null, "fully_exited":false, "buy_volume":1.0, "tokens_held":1.0},
        "wallets":[{"rank":1, "wallet":"W", "held_ratio":null, "has_sold":false, "atomic":false,
            "is_kol":false, "tokens_held":1.0}]});
    let plain: TokenBundle = de(base.clone());
    assert!(plain.held_now.is_none() && plain.wallets[0].balance_now.is_none());
    let mut opt = base;
    opt["held_now"] = held_now();
    opt["wallets"][0]["balance_now"] = balance_now();
    let b: TokenBundle = de(opt);
    assert_eq!(b.held_now.unwrap().held_pct_of_supply, Some(0.12));
    assert_eq!(
        b.wallets[0].balance_now.as_ref().unwrap().amount_raw,
        "1000000"
    );
}

#[test]
fn buyer_quality_dump_cluster_held_now() {
    let r: AlphaBuyerQualityResponse = de(json!({"mint":"M", "score":40, "confidence":"low",
        "signal":"neutral", "cached_at":"t",
        "dump_cluster_held_now":{"cohort_size":20, "members":2, "held_now":held_now(),
            "wallets":[{"wallet":"A", "balance_now":balance_now()}, {"wallet":"B", "balance_now":null}],
            "note":"n"}}));
    let d = r.dump_cluster_held_now.unwrap();
    assert_eq!(d.members, Some(2));
    assert!(d.wallets[1].balance_now.is_none());
}

#[test]
fn cap_table_balance_verification_and_scout_scores() {
    let r: AlphaCapTableResponse = de(json!({"mint":"M", "buyers":[{"rank":2, "wallet":"W",
        "first_buy_sol":1.0, "first_buy_at":null, "is_bundle":false, "is_kol":true, "kol_name":"K",
        "bot_confidence":null, "historical_win_rate":null, "historical_pnl_sol":null,
        "historical_tokens":null, "still_holding":true, "balance_now":balance_now(),
        "holding_status":"verified_zero", "exited_without_sell":true,
        "scout_score":{"as_of":"t", "first_touches_30d":4, "avg_followers_4h":null,
            "swarm_3plus_pct":0.5, "swarm_5plus_pct":null, "scout_tier":"A"}}],
        "summary":{"known_alpha_wallets":0, "known_kols":1, "bundle_buyers":0,
            "buyer_quality_score":10, "confidence":"low", "signal":"unknown"},
        "scout_scores_status":"ok", "scout_scores_note":"n",
        "onchain_balance_verification":{"source":"s", "checked_at":"t", "slot_min":1, "slot_max":2,
            "verified_wallets":1, "complete":true, "verified_still_holding":0, "verified_exited":1,
            "exited_without_sell":1, "note":"n"}}));
    let b = &r.buyers[0];
    assert_eq!(b.exited_without_sell, Some(true));
    assert_eq!(
        b.scout_score.as_ref().unwrap().scout_tier.as_deref(),
        Some("A")
    );
    assert_eq!(r.scout_scores_status.as_deref(), Some("ok"));
    assert_eq!(
        r.onchain_balance_verification.unwrap().exited_without_sell,
        Some(1)
    );
}

#[test]
fn holders_wallet_flags() {
    let h: TokenHolder = de(
        json!({"rank":1, "owner":"O", "token_accounts":["A"], "amount_raw":"5",
        "amount":5.0, "pct_of_supply":0.1, "pct_of_circulating":0.1,
        "labels":["sniper", "bundler_wallet", "dumper", "alpha_tracked", "future_label"],
        "wallet_flags":{"is_sniper":true, "is_bundler":false, "is_dumper":false, "is_kol":false,
            "is_alpha_tracked":true, "is_bot_high_confidence":false,
            "label_coverage":{"bundler":{"evaluated":false, "reason":"not_checked_budget"}}}}),
    );
    assert_eq!(
        h.labels,
        vec![
            HolderLabel::Sniper,
            HolderLabel::BundlerWallet,
            HolderLabel::Dumper,
            HolderLabel::AlphaTracked,
            HolderLabel::Other
        ]
    );
    let f = h.wallet_flags.unwrap();
    assert_eq!(f.is_sniper, Some(true));
    assert!(!f.label_coverage.unwrap()["bundler"].evaluated);
    let meta: WalletFlagsMeta =
        de(json!({"status":"partial", "rule_version":"v1", "note":"n", "sources":{"x":1}}));
    assert_eq!(meta.status, "partial");
    assert!(meta.extra.contains_key("sources"));
}

#[test]
fn top_traders_fifo_fields_and_null_valuations() {
    let t: TopTradersResponse = de(json!({"mint":"M", "sort":"pnl", "window_days":90,
        "cost_method":"fifo", "cost_method_note":"n",
        "traders":[{"rank":1, "wallet":"W", "trades":2, "buys":1, "sells":1, "bought_sol":1.0,
            "sold_sol":2.0, "realized_pnl_sol":0.5, "unrealized_pnl_sol":null, "total_pnl_sol":null,
            "held_value_sol":null, "roi":null, "unmatched_sell_tokens":250.0, "cost_basis_complete":false,
            "still_holding":true, "first_trade_at":"a", "last_trade_at":"b", "is_kol":false,
            "kol_name":null, "is_alpha_tracked":false}],
        "summary":{"returned":1, "known_kols":0, "known_alpha_wallets":0, "net_realized_pnl_sol":0.5}}));
    assert_eq!(t.cost_method.as_deref(), Some("fifo"));
    let tr = &t.traders[0];
    assert_eq!(tr.unrealized_pnl_sol, None);
    assert_eq!(tr.unmatched_sell_tokens, Some(250.0));
    assert_eq!(tr.cost_basis_complete, Some(false));
}

#[test]
fn depth_unsupported_pool_fillable() {
    let p: DepthUnsupportedPool = de(
        json!({"pool_address":"P", "dex":"meteora_dlmm", "quote_mint":"Q",
        "pool_model":"dlmm", "liquidity_usd":10.0, "is_active":true, "fillable_up_to_sol":0.0,
        "reason":"dlmm_insufficient_liquidity_in_loaded_bins"}),
    );
    assert_eq!(p.fillable_up_to_sol, Some(0.0));
}

#[test]
fn deployer_activity_dev_event_fields_and_launches_24h() {
    let e: DeployerActivityEvent = de(json!({"id":"e1", "type":"dev_token_transfer_out", "at":"t",
        "time_basis":"chain", "actor_wallet":"L", "actor_role":"linked_wallet",
        "token_amount_raw":"18446744073709551615", "supply_pct":1.5, "venue":null,
        "counterparty":"C", "counterparty_class":"cex_deposit", "feed":"realtime"}));
    assert_eq!(e.actor_role.as_deref(), Some("linked_wallet"));
    assert_eq!(e.token_amount_raw.as_deref(), Some("18446744073709551615"));
    assert_eq!(e.counterparty_class.as_deref(), Some("cex_deposit"));
    assert!(!e.extra.contains_key("actor_wallet"));
    assert_eq!(e.extra["counterparty"], "C"); // unbound fields stay in `extra`
    let r: DeployerTokensResponse = de(json!({"tokens":[], "total":0, "limit":20, "offset":0,
        "has_more":false, "launches_24h":{"window_start":"a", "window_end":"b", "count":null,
            "status":"over_budget", "completeness":"indexed_observations_only", "provenance":"p",
            "row_cap_per_source":5000, "source_rows":{"bonded":5000, "pending":0}}}));
    let l = r.launches_24h.unwrap();
    assert_eq!(l.count, None);
    assert_eq!(l.status, "over_budget");
}

#[test]
fn sniper_finalized_at() {
    let mut row = json!({"mint":"M", "name":null, "symbol":null, "deployer_wallet":"D",
        "signature":"S", "slot":100, "detected_at":"t", "detection_region":"AA",
        "finalized_at":"2026-10-15T00:00:01Z"});
    let d: SniperDeploy = de(row.clone());
    assert_eq!(d.finalized_at.as_deref(), Some("2026-10-15T00:00:01Z"));
    row.as_object_mut().unwrap().remove("finalized_at");
    assert!(de::<SniperDeploy>(row).finalized_at.is_none());
}
