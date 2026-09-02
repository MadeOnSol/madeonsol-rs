//! Request and response types for every endpoint in the MadeOnSol API.
//!
//! Field names mirror the JSON wire format exactly so you can paste API examples
//! straight from the docs at <https://madeonsol.com/api-docs>.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Shared enums ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum KolAction {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LeaderboardPeriod {
    #[serde(rename = "today")]
    Today,
    #[serde(rename = "7d")]
    D7,
    #[serde(rename = "30d")]
    D30,
    #[serde(rename = "90d")]
    D90,
    #[serde(rename = "180d")]
    D180,
}

impl LeaderboardPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Today => "today",
            Self::D7 => "7d",
            Self::D30 => "30d",
            Self::D90 => "90d",
            Self::D180 => "180d",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoordinationPeriod {
    #[serde(rename = "1h")]
    H1,
    #[serde(rename = "6h")]
    H6,
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "7d")]
    D7,
}

impl CoordinationPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H1 => "1h",
            Self::H6 => "6h",
            Self::H24 => "24h",
            Self::D7 => "7d",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KolStrategy {
    Scalper,
    DayTrader,
    SwingTrader,
    Hodler,
    Mixed,
}

impl KolStrategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Scalper => "scalper",
            Self::DayTrader => "day_trader",
            Self::SwingTrader => "swing_trader",
            Self::Hodler => "hodler",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KolLeaderboardSort {
    Pnl,
    Winrate,
    Volume,
    AvgRoi,
    ProfitFactor,
    EarlyEntryPct,
    Consistency,
}

impl KolLeaderboardSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pnl => "pnl",
            Self::Winrate => "winrate",
            Self::Volume => "volume",
            Self::AvgRoi => "avg_roi",
            Self::ProfitFactor => "profit_factor",
            Self::EarlyEntryPct => "early_entry_pct",
            Self::Consistency => "consistency",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeployerTier {
    Elite,
    Good,
    Moderate,
    Rising,
    Cold,
    #[serde(rename = "unranked")]
    Unranked,
}

impl DeployerTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Elite => "elite",
            Self::Good => "good",
            Self::Moderate => "moderate",
            Self::Rising => "rising",
            Self::Cold => "cold",
            Self::Unranked => "unranked",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeployerSortField {
    BondingRate,
    RecentBondRate,
    TotalBonded,
    LastDeployAt,
}

impl DeployerSortField {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BondingRate => "bonding_rate",
            Self::RecentBondRate => "recent_bond_rate",
            Self::TotalBonded => "total_bonded",
            Self::LastDeployAt => "last_deploy_at",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertPeriod {
    #[serde(rename = "7d")]
    D7,
    #[serde(rename = "30d")]
    D30,
    #[serde(rename = "all")]
    All,
}

impl AlertPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::D7 => "7d",
            Self::D30 => "30d",
            Self::All => "all",
        }
    }
}

pub type BestTokensPeriod = AlertPeriod;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WalletTrackerSummaryPeriod {
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "7d")]
    D7,
    #[serde(rename = "30d")]
    D30,
}

impl WalletTrackerSummaryPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H24 => "24h",
            Self::D7 => "7d",
            Self::D30 => "30d",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WalletTrackerEventType {
    Swap,
    Transfer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WalletTrackerAction {
    Buy,
    Sell,
    TransferIn,
    TransferOut,
}

impl WalletTrackerAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
            Self::TransferIn => "transfer_in",
            Self::TransferOut => "transfer_out",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CoordinationDeliveryMode {
    Websocket,
    Webhook,
    Both,
}

impl CoordinationDeliveryMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Websocket => "websocket",
            Self::Webhook => "webhook",
            Self::Both => "both",
        }
    }
}

// ─── KOL ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolFeedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor — return trades strictly older than this ISO 8601 timestamp.
    /// Pass `next_before` from the previous response for polling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<KolAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_sol: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_age_max_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_sells: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kol_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<KolStrategy>,
    /// v0.7 — Lower bound on `market_cap_usd_at_trade`. Trades with unknown MC drop out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    /// v0.7 — Upper bound on `market_cap_usd_at_trade`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolLeaderboardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<LeaderboardPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<KolLeaderboardSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<KolStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolWalletParams {
    /// Comma-separated extras: "pnl_by_token", "recent_winners", "recent_losers".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolCoordinationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<CoordinationPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_avg_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_strategies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_majors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<u32>,
    /// v0.7 — Lower bound on entry MC (MC at first KOL buy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    /// v0.7 — Upper bound on entry MC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTradeDeployer {
    pub wallet: String,
    pub tier: String,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTrade {
    pub tx_signature: String,
    pub wallet_address: String,
    #[serde(default)]
    pub kol_name: Option<String>,
    #[serde(default)]
    pub kol_twitter: Option<String>,
    pub action: KolAction,
    pub token_mint: String,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    pub sol_amount: f64,
    pub token_amount: f64,
    /// Token market cap in USD at the moment of trade (real-time, sourced
    /// from our in-memory price tracker — not Dexscreener spot, which lags).
    #[serde(default)]
    pub market_cap_usd_at_trade: Option<f64>,
    /// Token price in USD at the moment of trade.
    #[serde(default)]
    pub price_usd_at_trade: Option<f64>,
    pub traded_at: String,
    #[serde(default)]
    pub kol_strategy_tag: Option<String>,
    #[serde(default)]
    pub kol_auto_strategy_tag: Option<String>,
    #[serde(default)]
    pub kol_winrate_7d: Option<f64>,
    #[serde(default)]
    pub kol_winrate_30d: Option<f64>,
    #[serde(default)]
    pub kol_early_entry_pct_30d: Option<f64>,
    #[serde(default)]
    pub kol_is_heating_up: Option<bool>,
    #[serde(default)]
    pub kol_percentile_pnl_7d: Option<f64>,
    #[serde(default)]
    pub kol_percentile_winrate_7d: Option<f64>,
    #[serde(default)]
    pub token_age_minutes: Option<f64>,
    #[serde(default)]
    pub deployer: Option<KolTradeDeployer>,
    #[serde(default)]
    pub deployer_tier: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolFeedResponse {
    pub trades: Vec<KolTrade>,
    pub count: u32,
    #[serde(default)]
    pub data_age_seconds: Option<u64>,
    /// Cursor for the next page — pass as `before` to fetch older trades.
    #[serde(default)]
    pub next_before: Option<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolLeaderboardEntry {
    #[serde(default)]
    pub name: Option<String>,
    pub wallet: String,
    #[serde(default)]
    pub strategy_tag: Option<String>,
    #[serde(default)]
    pub auto_strategy_tag: Option<String>,
    pub pnl: f64,
    pub buy_count: u64,
    pub sell_count: u64,
    pub volume: f64,
    #[serde(default)]
    pub win_rate: Option<f64>,
    #[serde(default)]
    pub avg_roi: Option<f64>,
    #[serde(default)]
    pub profit_factor: Option<f64>,
    #[serde(default)]
    pub early_entry_pct_30d: Option<f64>,
    #[serde(default)]
    pub consistency_7d: Option<f64>,
    #[serde(default)]
    pub is_heating_up: Option<bool>,
    #[serde(default)]
    pub is_cold: Option<bool>,
    #[serde(default)]
    pub percentile_pnl_7d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_7d: Option<f64>,
    #[serde(default)]
    pub percentile_pnl_30d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_30d: Option<f64>,
    /// Median position hold time in minutes over the trailing 30 days.
    #[serde(default)]
    pub median_hold_minutes_30d: Option<f64>,
    /// Percentile rank for early-entry rate over the trailing 30 days (0–100).
    #[serde(default)]
    pub percentile_early_entry_30d: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolLeaderboardResponse {
    pub leaderboard: Vec<KolLeaderboardEntry>,
    pub period: String,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlByToken {
    pub mint: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub realized_pnl_usd: f64,
    pub buy_count: u32,
    pub sell_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolWalletProfile {
    pub wallet: String,
    pub kol_name: Option<String>,
    pub kol_twitter: Option<String>,
    pub total_pnl_usd: f64,
    pub win_rate: f64,
    pub trade_count: u32,
    #[serde(default)]
    pub pnl_by_token: Option<Vec<KolPnlByToken>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationKol {
    pub name: String,
    pub wallet: String,
    #[serde(default)]
    pub buy_sol: Option<f64>,
    #[serde(default)]
    pub sell_sol: Option<f64>,
    #[serde(default)]
    pub exited: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinatedToken {
    pub token_mint: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub kol_count: u32,
    pub total_buys: u64,
    pub total_sells: u64,
    pub net_sol_flow: f64,
    pub signal: String,
    pub avg_winrate_7d: Option<f64>,
    pub entry_rank_avg: Option<f64>,
    pub unique_strategies: u32,
    pub strategies: Vec<String>,
    pub first_buy_at: String,
    pub last_buy_at: String,
    pub time_to_consensus_sec: i64,
    #[serde(default)]
    pub peak_window_start: Option<String>,
    #[serde(default)]
    pub peak_window_end: Option<String>,
    #[serde(default)]
    pub peak_kols: Option<u32>,
    #[serde(default)]
    pub peak_buys: Option<u64>,
    #[serde(default)]
    pub exited_count: Option<u32>,
    #[serde(default)]
    pub holders_count: Option<u32>,
    #[serde(default)]
    pub coordination_score: Option<u32>,
    /// v0.6 (2026-05-06) — market cap (USD) stamped on the cluster's chronologically-first KOL buy.
    #[serde(default)]
    pub market_cap_usd_at_first_buy: Option<f64>,
    /// v0.6 — current market cap (USD), from `token_prices`.
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    /// v0.6 — current last-trade price (USD).
    #[serde(default)]
    pub last_price_usd: Option<f64>,
    #[serde(default)]
    pub kols: Option<Vec<CoordinationKol>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolCoordinationResponse {
    pub coordination: Vec<CoordinatedToken>,
    #[serde(default)]
    pub score_version: Option<String>,
    #[serde(default)]
    pub window_minutes: Option<u32>,
    #[serde(default)]
    pub period: Option<String>,
    #[serde(default)]
    pub min_kols: Option<u32>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTokenActivity {
    pub mint: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub kol_buyers: Vec<String>,
    pub kol_sellers: Vec<String>,
    pub buy_count: u32,
    pub sell_count: u32,
    pub total_sol_volume: f64,
    pub recent_trades: Vec<KolTrade>,
}

// ─── KOL pairs / timing / hot-tokens / pnl / trending ───────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolPairsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_shared: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPairMember {
    pub name: String,
    #[serde(default)]
    pub wallet: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPair {
    pub kol_a: KolPairMember,
    pub kol_b: KolPairMember,
    pub shared_token_count: u32,
    #[serde(default)]
    pub agreement_rate: Option<f64>,
    #[serde(default)]
    pub shared_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPairsResponse {
    pub pairs: Vec<KolPair>,
    pub period: String,
    pub min_shared: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolTimingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTimingProfile {
    pub tokens_traded: u32,
    pub positions_closed: u32,
    pub avg_hold_minutes: Option<f64>,
    #[serde(default)]
    pub median_hold_minutes: Option<f64>,
    #[serde(default)]
    pub pct_closed_1h: Option<f64>,
    #[serde(default)]
    pub pct_closed_6h: Option<f64>,
    #[serde(default)]
    pub pct_closed_24h: Option<f64>,
    #[serde(default)]
    pub avg_buy_size_sol: Option<f64>,
    #[serde(default)]
    pub avg_sell_size_sol: Option<f64>,
    #[serde(default)]
    pub most_active_hours: Option<Vec<u32>>,
    #[serde(default)]
    pub hour_distribution: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTimingResponse {
    pub kol: KolPairMember,
    pub timing: KolTimingProfile,
    pub period: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolHotTokensParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_avg_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_strategies: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HotToken {
    pub token_mint: String,
    pub token_symbol: String,
    pub token_name: String,
    pub kols_total: u32,
    pub kols_recent: u32,
    pub acceleration: f64,
    pub total_buy_sol: f64,
    pub total_sell_sol: f64,
    pub net_flow: f64,
    pub first_kol_buy_age_minutes: Option<f64>,
    #[serde(default)]
    pub kols: Option<Vec<KolPairMember>>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    #[serde(default)]
    pub first_kol_buy_at: Option<String>,
    #[serde(default)]
    pub last_kol_buy_at: Option<String>,
    #[serde(default)]
    pub time_to_consensus_sec: Option<i64>,
    #[serde(default)]
    pub avg_winrate_7d: Option<f64>,
    #[serde(default)]
    pub entry_rank_avg: Option<f64>,
    #[serde(default)]
    pub unique_strategies: Option<u32>,
    #[serde(default)]
    pub strategies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolHotTokensResponse {
    pub hot_tokens: Vec<HotToken>,
    pub period: String,
    pub min_kols: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolPnlParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlSummary {
    pub realized_pnl_sol: f64,
    pub total_volume_sol: f64,
    pub tokens_traded: u32,
    pub closed_positions: u32,
    pub open_positions: u32,
    pub win_count: u32,
    pub loss_count: u32,
    pub win_rate: Option<f64>,
    pub profit_factor: Option<f64>,
    pub best_trade_pnl_sol: f64,
    pub worst_trade_pnl_sol: f64,
    pub avg_roi_pct: Option<f64>,
    pub avg_hold_minutes: f64,
    pub median_hold_minutes: f64,
    pub max_drawdown_sol: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlCurvePoint {
    pub date: String,
    pub day_pnl: f64,
    pub cumulative_pnl: f64,
    pub trades: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolClosedPosition {
    pub token_mint: String,
    pub token_symbol: String,
    pub token_name: String,
    pub buy_count: u32,
    pub sell_count: u32,
    pub bought_sol: f64,
    pub sold_sol: f64,
    pub pnl_sol: f64,
    pub roi_pct: f64,
    pub hold_minutes: f64,
    pub result: String,
    pub first_trade: String,
    pub last_trade: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolOpenPosition {
    pub token_mint: String,
    pub token_symbol: String,
    pub token_name: String,
    pub buy_count: u32,
    pub bought_sol: f64,
    pub first_buy_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlIdentity {
    pub name: String,
    #[serde(default)]
    pub wallet: Option<String>,
    #[serde(default)]
    pub twitter_url: Option<String>,
    #[serde(default)]
    pub strategy_tag: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlResponse {
    pub kol: KolPnlIdentity,
    pub summary: KolPnlSummary,
    #[serde(default)]
    pub pnl_curve: Option<Vec<KolPnlCurvePoint>>,
    #[serde(default)]
    pub closed_positions: Option<Vec<KolClosedPosition>>,
    #[serde(default)]
    pub open_positions: Option<Vec<KolOpenPosition>>,
    pub period: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolTrendingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrendingToken {
    pub token_mint: String,
    pub token_symbol: String,
    pub token_name: String,
    pub buy_volume_sol: f64,
    pub sell_volume_sol: f64,
    pub net_flow_sol: f64,
    pub buy_count: u32,
    pub sell_count: u32,
    pub kol_count: u32,
    pub latest_buy_age_minutes: Option<f64>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    #[serde(default)]
    pub first_buy_at: Option<String>,
    #[serde(default)]
    pub latest_buy_at: Option<String>,
    #[serde(default)]
    pub kols: Option<Vec<KolPairMember>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTrendingResponse {
    pub trending: Vec<TrendingToken>,
    pub period: String,
    pub min_kols: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── KOL entry-order / compare / alerts ─────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolEntryOrderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolEntryOrderEntry {
    pub rank: u32,
    pub kol_name: Option<String>,
    pub kol_twitter: Option<String>,
    pub wallet: String,
    pub strategy_tag: Option<String>,
    pub auto_strategy_tag: Option<String>,
    pub winrate_7d: Option<f64>,
    pub winrate_30d: Option<f64>,
    pub early_entry_pct_30d: Option<f64>,
    #[serde(default)]
    pub percentile_pnl_7d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_7d: Option<f64>,
    pub first_buy_at: String,
    pub seconds_after_first: f64,
    pub sol_amount: f64,
    pub token_amount: f64,
    #[serde(default)]
    pub tx_signature: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolEntryOrderResponse {
    pub token_mint: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub total_kol_buyers: u32,
    pub first_buy_at: String,
    pub last_buy_at: String,
    pub span_sec: i64,
    pub entries: Vec<KolEntryOrderEntry>,
}

#[derive(Debug, Clone)]
pub struct KolCompareParams {
    /// 2–5 Solana wallet addresses. Tier limit: BASIC=2, PRO=4, ULTRA=5.
    pub wallets: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolCompareProfile {
    pub wallet: String,
    pub found: bool,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub twitter_url: Option<String>,
    #[serde(default)]
    pub strategy_tag: Option<String>,
    #[serde(default)]
    pub auto_strategy_tag: Option<String>,
    #[serde(default)]
    pub winrate_7d: Option<f64>,
    #[serde(default)]
    pub winrate_30d: Option<f64>,
    #[serde(default)]
    pub avg_roi_7d: Option<f64>,
    #[serde(default)]
    pub avg_roi_30d: Option<f64>,
    #[serde(default)]
    pub profit_factor_7d: Option<f64>,
    #[serde(default)]
    pub profit_factor_30d: Option<f64>,
    #[serde(default)]
    pub pnl_7d: Option<f64>,
    #[serde(default)]
    pub pnl_30d: Option<f64>,
    #[serde(default)]
    pub early_entry_pct_30d: Option<f64>,
    #[serde(default)]
    pub consistency_7d: Option<f64>,
    #[serde(default)]
    pub median_hold_minutes_30d: Option<f64>,
    #[serde(default)]
    pub closed_positions_7d: Option<u32>,
    #[serde(default)]
    pub closed_positions_30d: Option<u32>,
    #[serde(default)]
    pub is_heating_up: Option<bool>,
    #[serde(default)]
    pub is_cold: Option<bool>,
    #[serde(default)]
    pub percentile_pnl_7d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_7d: Option<f64>,
    #[serde(default)]
    pub percentile_pnl_30d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_30d: Option<f64>,
    #[serde(default)]
    pub percentile_early_entry_30d: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolCompareOverlapToken {
    pub token_mint: String,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
    pub wallets: Vec<String>,
    pub buy_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolCompareResponse {
    pub profiles: Vec<KolCompareProfile>,
    #[serde(default)]
    pub overlap: Option<Vec<KolCompareOverlapToken>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolAlertsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolAlert {
    #[serde(rename = "type")]
    pub alert_type: String,
    pub severity: String,
    pub detected_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolAlertsResponse {
    pub alerts: Vec<KolAlert>,
    pub count: u32,
    pub window: String,
    pub types: Vec<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Scout leaderboard (v1.9) ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScoutLeaderboardSort {
    Swarm3PlusPct,
    NFirstTouches30d,
    Swarm5PlusPct,
    ScoutScore,
}

impl ScoutLeaderboardSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Swarm3PlusPct => "swarm_3plus_pct",
            Self::NFirstTouches30d => "n_first_touches_30d",
            Self::Swarm5PlusPct => "swarm_5plus_pct",
            Self::ScoutScore => "scout_score",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ScoutLeaderboardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scout_tier: Option<ScoutTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ScoutLeaderboardSort>,
}

// ─── Coordination history (v1.9) ────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct CoordinationHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<u32>,
}

// ─── KOL consensus (v1.9) ──────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct KolConsensusResponse {
    pub total_kol_buyers: u32,
    pub total_kol_sellers: u32,
    #[serde(default)]
    pub kol_exit_rate: Option<f64>,
    pub net_flow_sol: f64,
    pub total_buy_sol: f64,
    pub total_sell_sol: f64,
    #[serde(default)]
    pub first_kol_buy_at: Option<String>,
    #[serde(default)]
    pub last_kol_buy_at: Option<String>,
    #[serde(default)]
    pub first_touch_wallet: Option<String>,
    #[serde(default)]
    pub first_touch_at: Option<String>,
    #[serde(default)]
    pub median_entry_mc_usd: Option<f64>,
    /// ULTRA only — individual buyer wallet addresses.
    #[serde(default)]
    pub buyers: Option<Vec<String>>,
    /// ULTRA only — wallets that have fully exited.
    #[serde(default)]
    pub exited: Option<Vec<String>>,
}

// ─── Peak history (v1.9) ───────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct PeakHistoryResponse {
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_updated_at: Option<String>,
    #[serde(default)]
    pub current_mc_usd: Option<f64>,
    #[serde(default)]
    pub current_price_usd: Option<f64>,
    #[serde(default)]
    pub decline_from_peak_pct: Option<f64>,
    #[serde(default)]
    pub mc_at_bond: Option<f64>,
    #[serde(default)]
    pub mc_1h_after_bond: Option<f64>,
    #[serde(default)]
    pub mc_6h_after_bond: Option<f64>,
    #[serde(default)]
    pub mc_24h_after_bond: Option<f64>,
    #[serde(default)]
    pub mc_7d_after_bond: Option<f64>,
    #[serde(default)]
    pub still_alive_1h: Option<bool>,
    #[serde(default)]
    pub time_to_bond_minutes: Option<f64>,
    #[serde(default)]
    pub deployed_at: Option<String>,
    #[serde(default)]
    pub bonded_at: Option<String>,
}

// ─── Coordination alerts (v1.1) ─────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertRule {
    pub id: String,
    pub name: Option<String>,
    pub min_kols: u32,
    pub window_minutes: u32,
    pub min_score: u32,
    pub include_majors: bool,
    pub cooldown_min: u32,
    pub score_jump_break: u32,
    pub delivery_mode: CoordinationDeliveryMode,
    pub webhook_url: Option<String>,
    /// v0.7 — entry-MC band on the rule (None = open-ended).
    #[serde(default)]
    pub min_mc_usd: Option<f64>,
    #[serde(default)]
    pub max_mc_usd: Option<f64>,
    pub is_active: bool,
    pub created_at: String,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CoordinationAlertCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_majors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_jump_break: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<CoordinationDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// v0.7 — entry-MC band the rule will require for triggers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CoordinationAlertUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_majors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_jump_break: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<CoordinationDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// v0.7 — pass `Some(0.0)` etc. to update the band; `None` leaves unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertListResponse {
    pub rules: Vec<CoordinationAlertRule>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertCreateResponse {
    pub rule: CoordinationAlertRule,
    /// One-time HMAC secret. Save it — will not be shown again.
    pub webhook_secret: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertGetResponse {
    pub rule: CoordinationAlertRule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertUpdateResponse {
    pub rule: CoordinationAlertRule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertDeleteResponse {
    pub deleted: bool,
}

// ─── First-touch signal ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum ScoutTier {
    #[serde(rename = "S")] S,
    #[serde(rename = "A")] A,
    #[serde(rename = "B")] B,
    #[serde(rename = "C")] C,
}

impl ScoutTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScoutTier::S => "S",
            ScoutTier::A => "A",
            ScoutTier::B => "B",
            ScoutTier::C => "C",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FirstTouchPreset {
    Scout,
    FreshLaunch,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FirstTouchesParams {
    /// ISO datetime — events strictly newer than this. Polling cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// ISO datetime — events strictly older than this. Pagination cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 1–100. Default: 50 (BASIC capped at 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Single KOL wallet (32–44 base58 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kol: Option<String>,
    /// 0–100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kol_winrate_7d: Option<f64>,
    /// Restrict to scouts of this tier or better. Requires `n_first_touches_30d >= 30`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_scout_tier: Option<ScoutTier>,
    /// Lower the minimum sample size for scout scoring (default 30).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_n_touches: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<KolStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_age_max_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_first_buy_sol: Option<f64>,
    /// Suffix-filter the token mint (e.g. "pump", "bonk").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint_suffix: Option<String>,
    /// Shortcut filter sets — `scout` = min_scout_tier=B + min_n_touches=30 + token_age_max_min=60.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<FirstTouchPreset>,
    /// Comma-separated includes — currently `followers_4h`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    /// v0.7 — Lower bound on `market_cap_usd_at_first_buy`. Touches with unknown MC drop out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    /// v0.7 — Upper bound on `market_cap_usd_at_first_buy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchKol {
    /// Wallet address — only present on Ultra tier.
    #[serde(default)]
    pub wallet: Option<String>,
    pub name: Option<String>,
    pub twitter_url: Option<String>,
    pub winrate_7d: Option<f64>,
    pub strategy: Option<String>,
    pub scout_tier: Option<ScoutTier>,
    /// Same as swarm_3plus_pct on the scout leaderboard.
    pub scout_score: Option<f64>,
    pub n_first_touches_30d: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchEvent {
    pub token_mint: String,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
    pub token_image_url: Option<String>,
    pub first_buy_at: String,
    pub sol_amount: Option<f64>,
    pub token_amount: Option<f64>,
    pub tx_signature: Option<String>,
    pub token_age_minutes: Option<u32>,
    pub first_kol: FirstTouchKol,
    #[serde(default)]
    pub followers_4h: Option<u32>,
    /// v0.6 (2026-05-06) — market cap (USD) stamped on the exact tx that fired
    /// the first KOL buy, joined via `tx_signature`.
    #[serde(default)]
    pub market_cap_usd_at_first_buy: Option<f64>,
    /// v0.6 — token price (USD) at the same moment.
    #[serde(default)]
    pub price_usd_at_first_buy: Option<f64>,
    /// v0.6 — current market cap (USD), from `token_prices`.
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    /// v0.6 — current last-trade price (USD).
    #[serde(default)]
    pub last_price_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchesResponse {
    pub events: Vec<FirstTouchEvent>,
    pub count: u32,
    pub next_before: Option<String>,
    pub data_age_seconds: Option<u32>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FirstTouchSubscriptionFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_first_buy_sol: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_scout_tier: Option<ScoutTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_n_touches: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscription {
    pub id: String,
    pub name: Option<String>,
    pub filters: FirstTouchSubscriptionFilters,
    pub delivery_mode: CoordinationDeliveryMode,
    pub webhook_url: Option<String>,
    /// v0.7 — first-touch MC band on the subscription (None = open-ended).
    #[serde(default)]
    pub min_mc_usd: Option<f64>,
    #[serde(default)]
    pub max_mc_usd: Option<f64>,
    pub is_active: bool,
    pub created_at: String,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FirstTouchSubscriptionCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FirstTouchSubscriptionFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<CoordinationDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// v0.7 — first-touch MC band on the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FirstTouchSubscriptionUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FirstTouchSubscriptionFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<CoordinationDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// v0.7 — pass `Some(0.0)` etc. to update the band; `None` leaves unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionListResponse {
    pub subscriptions: Vec<FirstTouchSubscription>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionCreateResponse {
    pub subscription: FirstTouchSubscription,
    /// One-time HMAC secret. Save it — will not be shown again.
    pub webhook_secret: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionGetResponse {
    pub subscription: FirstTouchSubscription,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionUpdateResponse {
    pub subscription: FirstTouchSubscription,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionDeleteResponse {
    pub ok: bool,
}

// ─── Deployer Hunter ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerLeaderboardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<DeployerTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<DeployerSortField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerTokensParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerAlertsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Cursor — return alerts strictly older than this ISO 8601 timestamp.
    /// Pass `next_before` from previous response. Preferred over `offset` at scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// PRO/ULTRA only. BASIC subscribers receive HTTP 403.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<DeployerTier>,
    /// Filter by alert_type (e.g. "new_deploy", "bonded").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    /// Filter by alert priority ("high" | "medium" | "low").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// Only alerts where at least N KOLs bought the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kol_buys: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerAlertStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<AlertPeriod>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BestTokensParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<BestTokensPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecentBondsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// ISO 8601 datetime — only bonds strictly newer than this timestamp.
    /// Pass `next_since` from the previous response for incremental polling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Filter by deployer reputation tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<DeployerTier>,
    /// Only bonds that reached at least this peak market cap (USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_mc_min: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTierCounts {
    pub elite: u32,
    pub good: u32,
    pub rising: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerStats {
    pub tracked_count: u32,
    pub signals_today: u32,
    pub bonds_detected: u32,
    pub bond_rate: f64,
    pub tiers: DeployerTierCounts,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Shared: DeployerSummary (used by RecentBond, DeployerAlert) ─────────────

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerSummary {
    pub wallet_address: String,
    pub tier: DeployerTier,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
    #[serde(default)]
    pub total_bonded: Option<u32>,
    #[serde(default)]
    pub recent_outcomes: Option<String>,
    #[serde(default)]
    pub recent_bond_rate: Option<f64>,
    #[serde(default)]
    pub total_tokens_deployed: Option<u32>,
    /// Peak market cap (USD) of this deployer's best token to date. Populated on alert rows.
    #[serde(default)]
    pub best_token_peak_mc: Option<f64>,
    /// Fraction of the deployer's labeled tokens that ran (peak >=60min after deploy) vs dumped.
    #[serde(default)]
    pub runner_rate: Option<f64>,
    /// Confidence denominator; gate on >=3.
    #[serde(default)]
    pub labeled_tokens: Option<i64>,
    #[serde(default)]
    pub avg_time_to_bond_minutes: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerLeaderboardEntry {
    pub id: String,
    pub wallet_address: String,
    pub tier: DeployerTier,
    pub bonding_rate: f64,
    pub recent_bond_rate: f64,
    pub total_tokens_deployed: u32,
    pub total_bonded: u32,
    #[serde(default)]
    pub last_deploy_at: Option<String>,
    #[serde(default)]
    pub recent_outcomes: Option<String>,
    #[serde(default)]
    pub avg_time_to_bond_minutes: Option<f64>,
    /// Fraction of the deployer's labeled tokens that ran (peak >=60min after deploy) vs dumped.
    #[serde(default)]
    pub runner_rate: Option<f64>,
    /// Confidence denominator; gate on >=3.
    #[serde(default)]
    pub labeled_tokens: Option<i64>,
    #[serde(default)]
    pub best_token_peak_mc: Option<f64>,
    #[serde(default)]
    pub avg_peak_mc: Option<f64>,
    #[serde(default)]
    pub last_bond_at: Option<String>,
    #[serde(default)]
    pub is_tracked: Option<bool>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub first_seen_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerLeaderboardResponse {
    pub deployers: Vec<DeployerLeaderboardEntry>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    pub has_more: bool,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerToken {
    pub mint: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub bonded: bool,
    pub deployed_at: String,
    pub bonded_at: Option<String>,
    pub peak_market_cap_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerProfile {
    pub wallet: String,
    pub tier: DeployerTier,
    pub bonding_rate: f64,
    pub recent_bond_rate: f64,
    pub total_deployed: u32,
    pub total_bonded: u32,
    pub last_deploy_at: Option<String>,
    pub first_seen: Option<String>,
    /// Fraction of the deployer's labeled tokens that ran (peak >=60min after deploy) vs dumped.
    #[serde(default)]
    pub runner_rate: Option<f64>,
    /// Confidence denominator; gate on >=3.
    #[serde(default)]
    pub labeled_tokens: Option<i64>,
    #[serde(default)]
    pub avg_time_to_bond_minutes: Option<i64>,
    #[serde(default)]
    pub tokens: Option<Vec<DeployerToken>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTokensResponse {
    pub tokens: Vec<DeployerToken>,
    pub count: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolBuysSummary {
    pub count: u32,
    pub total_sol: f64,
    pub kols: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerAlert {
    pub id: String,
    pub token_mint: String,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    pub alert_type: String,
    pub title: String,
    pub message: String,
    pub priority: String,
    pub created_at: String,
    #[serde(default)]
    pub market_cap_at_alert: Option<f64>,
    /// Deployer wallet's SOL balance at alert time, in SOL. `None` when unknown.
    #[serde(default)]
    pub deployer_sol_balance: Option<f64>,
    pub deployers: DeployerSummary,
    #[serde(default)]
    pub kol_buys: Option<KolBuysSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerAlertsResponse {
    pub alerts: Vec<DeployerAlert>,
    pub limit: u32,
    pub offset: u32,
    /// Cursor for the next page — pass as `before` to fetch older alerts.
    #[serde(default)]
    pub next_before: Option<String>,
    #[serde(default)]
    pub data_age_seconds: Option<u64>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BondRateStats {
    pub total_deploys: u32,
    pub total_bonded: u32,
    pub rate: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MultiplierStats {
    pub total_with_mc: u32,
    pub pct_2x: f64,
    pub pct_5x: f64,
    pub pct_10x: f64,
    pub pct_50x: f64,
    pub avg_multiplier: f64,
    pub best_multiplier: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TierStats {
    pub deploys: u32,
    pub bonded: u32,
    pub bond_rate: f64,
    #[serde(default)]
    pub avg_multiplier: Option<f64>,
    pub total_with_mc: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerAlertStats {
    pub bond_rate: BondRateStats,
    pub multiplier: MultiplierStats,
    pub tiers: HashMap<String, TierStats>,
    pub period: String,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BestToken {
    pub id: String,
    pub token_mint: String,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    pub bonded_at: String,
    #[serde(default)]
    pub peak_market_cap: Option<f64>,
    #[serde(default)]
    pub mc_at_bond: Option<f64>,
    #[serde(default)]
    pub market_cap_at_alert: Option<f64>,
    #[serde(default)]
    pub mc_multiplier: Option<f64>,
    pub deployer_wallet: String,
    pub deployer_tier: DeployerTier,
    #[serde(default)]
    pub alerted_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BestTokensResponse {
    pub tokens: Vec<BestToken>,
    pub period: String,
    pub limit: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecentBond {
    pub id: String,
    pub token_mint: String,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    pub deployed_at: String,
    pub bonded_at: String,
    #[serde(default)]
    pub time_to_bond_minutes: Option<f64>,
    #[serde(default)]
    pub peak_market_cap: Option<f64>,
    #[serde(default)]
    pub mc_at_bond: Option<f64>,
    pub deployers: DeployerSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecentBondsResponse {
    pub tokens: Vec<RecentBond>,
    pub limit: u32,
    /// Cursor for incremental polling — pass as `since` on the next call to
    /// fetch only newer bonds.
    #[serde(default)]
    pub next_since: Option<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerStreak {
    #[serde(rename = "type")]
    pub streak_type: String,
    pub count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRollingRate {
    pub window_end: u32,
    pub bond_rate: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerStretch {
    pub start_index: u32,
    pub end_index: u32,
    pub bond_rate: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTrajectoryData {
    pub current_streak: DeployerStreak,
    pub longest_bond_streak: u32,
    pub longest_fail_streak: u32,
    pub rolling_bond_rates: Vec<DeployerRollingRate>,
    pub trend: String,
    pub avg_days_between_deploys: Option<f64>,
    pub avg_recovery_tokens: Option<f64>,
    pub best_stretch: Option<DeployerStretch>,
    pub worst_stretch: Option<DeployerStretch>,
    pub total_tokens_analyzed: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTrajectoryDeployer {
    pub wallet_address: String,
    pub total_tokens_deployed: u32,
    pub total_bonded: u32,
    pub bonding_rate: f64,
    pub recent_bond_rate: f64,
    pub tier: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerTrajectoryParams {
    /// Pass `"daily_snapshots"` for up to 90 daily tier/bonding snapshots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerDailySnapshot {
    pub snapshot_date: String,
    pub tier: String,
    pub total_tokens_deployed: u32,
    pub total_bonded: u32,
    pub bonding_rate: f64,
    pub recent_bond_rate: f64,
    #[serde(default)]
    pub avg_peak_mc: Option<f64>,
    #[serde(default)]
    pub best_token_peak_mc: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTrajectoryResponse {
    pub deployer: DeployerTrajectoryDeployer,
    pub trajectory: DeployerTrajectoryData,
    /// Present when `include=daily_snapshots` is requested.
    #[serde(default)]
    pub daily_snapshots: Option<Vec<DeployerDailySnapshot>>,
}

// ─── Deployer history (/deployer-hunter/{wallet}/history) ────────────────────

/// Query params for [`Deployer::history`](crate::api::deployer::Deployer::history).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerHistoryParams {
    /// Number of daily snapshots to return (1..=365). Defaults server-side when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// A single daily performance snapshot for a deployer wallet.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerSnapshot {
    /// Snapshot date (ISO 8601).
    pub date: String,
    pub tier: String,
    pub is_tracked: bool,
    pub total_deployed: i64,
    pub total_bonded: i64,
    /// Lifetime bonding rate. `None` when unavailable.
    #[serde(default)]
    pub bonding_rate: Option<f64>,
    /// Recent (rolling) bond rate. `None` when unavailable.
    #[serde(default)]
    pub recent_bond_rate: Option<f64>,
    /// Average peak market cap across deploys. `None` when unavailable.
    #[serde(default)]
    pub avg_peak_mc: Option<f64>,
    /// Peak market cap of the best token. `None` when unavailable.
    #[serde(default)]
    pub best_token_peak_mc: Option<f64>,
}

/// Daily performance snapshots for a deployer wallet.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerHistoryResponse {
    pub is_deployer: bool,
    pub wallet: String,
    pub snapshots: Vec<DeployerSnapshot>,
}

// ─── Deployer as-of (/deployer-hunter/{wallet}/as-of) ─────────────────────────

/// Query params for [`Deployer::as_of`](crate::api::deployer::Deployer::as_of).
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerAsOfParams {
    /// YYYY-MM-DD (UTC). Unset = today. Must be >= 2026-04-07 and not in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

/// The reputation snapshot current on the requested date. `snapshot_date` can be
/// earlier than `requested_date` (snapshots are write-on-change); `carried: true`
/// means the state was recorded earlier and had not changed by then.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerAsOfSnapshot {
    pub snapshot_date: String,
    pub carried: bool,
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub is_tracked: Option<bool>,
    #[serde(default)]
    pub total_deployed: Option<i64>,
    #[serde(default)]
    pub total_bonded: Option<i64>,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
    #[serde(default)]
    pub recent_bond_rate: Option<f64>,
    #[serde(default)]
    pub avg_peak_mc: Option<f64>,
    #[serde(default)]
    pub best_token_peak_mc: Option<f64>,
    #[serde(default)]
    pub captured_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerAsOfResponse {
    pub is_deployer: bool,
    pub wallet: String,
    pub requested_date: String,
    /// `true` when a snapshot at or before `requested_date` exists.
    pub as_of: bool,
    /// `None` when no snapshot exists at or before `requested_date` — nothing is
    /// ever synthesized.
    #[serde(default)]
    pub snapshot: Option<DeployerAsOfSnapshot>,
    #[serde(default)]
    pub first_snapshot_date: Option<String>,
    pub note: String,
}

// ─── Deployer creator-fee rewards (/deployer-hunter/{wallet}/rewards) ─────────

/// `sol` / `usdc` are summed separately (never mixed); `usd` is `None` (not 0)
/// when a SOL amount exists and no SOL price was available.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsMoney {
    pub sol: f64,
    pub usdc: f64,
    #[serde(default)]
    pub usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsRail {
    pub sol: f64,
    pub usdc: f64,
    #[serde(default)]
    pub usd: Option<f64>,
    pub count: i64,
    #[serde(default)]
    pub first_at: Option<String>,
    #[serde(default)]
    pub last_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsDirectClaims {
    #[serde(flatten)]
    pub rail: DeployerRewardsRail,
    pub window_days: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsSharePayouts {
    #[serde(flatten)]
    pub rail: DeployerRewardsRail,
    pub tokens: i64,
    pub on_own_tokens: DeployerRewardsMoney,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsCollected {
    pub sol: f64,
    pub usdc: f64,
    #[serde(default)]
    pub usd: Option<f64>,
    pub direct_claims: DeployerRewardsDirectClaims,
    pub social_claims: DeployerRewardsRail,
    pub share_payouts: DeployerRewardsSharePayouts,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsAttributed {
    pub sol: f64,
    pub usdc: f64,
    #[serde(default)]
    pub usd: Option<f64>,
    pub count: i64,
    pub to_self: DeployerRewardsMoney,
    pub to_others: DeployerRewardsMoney,
    /// Share of attributed fees redirected away from the deployer, 0–100. `None` with no attributed fees.
    #[serde(default)]
    pub redirected_pct: Option<f64>,
    pub tokens_with_payouts: i64,
    pub distributions: i64,
    pub recipients: i64,
    #[serde(default)]
    pub first_at: Option<String>,
    #[serde(default)]
    pub last_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsSocial {
    pub platform: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsTopToken {
    pub mint: String,
    pub quote: String,
    pub total: f64,
    #[serde(default)]
    pub total_usd: Option<f64>,
    pub to_self: f64,
    #[serde(default)]
    pub to_self_usd: Option<f64>,
    pub payouts: i64,
    pub recipients: i64,
    pub last_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsTopRecipient {
    pub address: String,
    pub quote: String,
    pub total: f64,
    #[serde(default)]
    pub total_usd: Option<f64>,
    pub tokens: i64,
    pub payouts: i64,
    pub last_at: String,
    pub is_self: bool,
    pub is_social_pda: bool,
    #[serde(default)]
    pub social: Option<DeployerRewardsSocial>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsQuote {
    #[serde(default)]
    pub sol_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsCoverage {
    pub payouts_since: String,
    pub direct_claims_window_days: i64,
    pub note: String,
}

/// pump.fun creator-fee rewards for a wallet. `collected` (what actually reached
/// it) and `attributed` (what was paid out on the tokens it deployed) are kept
/// deliberately separate — never merge them.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRewardsResponse {
    pub wallet: String,
    pub is_deployer: bool,
    /// Tokens attributed to this wallet in our token table — the universe
    /// `attributed` is computed over. NOT the deployer profile's total deploy count.
    pub tokens_in_scope: i64,
    pub collected: DeployerRewardsCollected,
    pub attributed: DeployerRewardsAttributed,
    pub top_tokens: Vec<DeployerRewardsTopToken>,
    pub top_recipients: Vec<DeployerRewardsTopRecipient>,
    pub quote: DeployerRewardsQuote,
    pub coverage: DeployerRewardsCoverage,
}

// ─── Alpha Wallet Intelligence ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlphaSort {
    WinRate,
    Pnl,
    Roi,
}

impl AlphaSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WinRate => "win_rate",
            Self::Pnl => "pnl",
            Self::Roi => "roi",
        }
    }
}

pub type AlphaPeriod = AlertPeriod;

#[derive(Debug, Clone, Default, Serialize)]
pub struct AlphaLeaderboardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<AlphaPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<AlphaSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_bots: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletEntry {
    pub rank: u32,
    pub wallet: String,
    pub tokens_traded: u32,
    pub wins: u32,
    pub losses: u32,
    pub win_rate: Option<f64>,
    pub net_pnl_sol: f64,
    #[serde(default)]
    pub total_sol_bought: Option<f64>,
    #[serde(default)]
    pub total_sol_sold: Option<f64>,
    #[serde(default)]
    pub roi: Option<f64>,
    #[serde(default)]
    pub avg_rank: Option<f64>,
    #[serde(default)]
    pub best_rank: Option<f64>,
    #[serde(default)]
    pub total_buys: Option<u32>,
    #[serde(default)]
    pub total_sells: Option<u32>,
    #[serde(default)]
    pub last_seen: Option<String>,
    #[serde(default)]
    pub bundle_rate: Option<f64>,
    #[serde(default)]
    pub buy_size_stddev: Option<f64>,
    #[serde(default)]
    pub active_hours: Option<f64>,
    #[serde(default)]
    pub bot_confidence: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaLeaderboardResponse {
    pub leaderboard: Vec<AlphaWalletEntry>,
    pub total: u32,
    pub period: String,
    pub sort: String,
    pub min_tokens: u32,
    pub exclude_bots: bool,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletPosition {
    pub token_mint: String,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
    pub first_buy_at: Option<String>,
    pub last_trade_at: Option<String>,
    pub buy_count: u32,
    pub sell_count: u32,
    pub total_bought_sol: f64,
    pub total_sold_sol: f64,
    pub realized_pnl_sol: f64,
    pub roi_pct: Option<f64>,
    pub result: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletBotSignal {
    pub signal: String,
    pub detail: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletSummary {
    pub tokens_traded: u32,
    pub wins: u32,
    pub losses: u32,
    pub win_rate: Option<f64>,
    pub net_pnl_sol: f64,
    pub total_vol_sol: f64,
    pub roi: Option<f64>,
    pub avg_rank: Option<f64>,
    pub best_rank: Option<f64>,
    pub bundle_rate: f64,
    pub buy_size_stddev: f64,
    pub active_hours: Option<f64>,
    pub bot_confidence: String,
    pub night_only_activity: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletResponse {
    pub wallet: String,
    pub summary: AlphaWalletSummary,
    pub positions: Vec<AlphaWalletPosition>,
    pub bot_signals: Vec<AlphaWalletBotSignal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaLinkedWallet {
    pub wallet_address: String,
    pub shared_tokens: u32,
    pub similarity_score: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaLinkedResponse {
    pub wallet: String,
    pub linked_wallets: Vec<AlphaLinkedWallet>,
    pub total: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaCapTableBuyer {
    pub rank: u32,
    pub wallet: String,
    pub first_buy_sol: f64,
    pub first_buy_at: Option<String>,
    pub is_bundle: bool,
    pub is_kol: bool,
    pub kol_name: Option<String>,
    pub bot_confidence: Option<String>,
    pub historical_win_rate: Option<f64>,
    pub historical_pnl_sol: Option<f64>,
    pub historical_tokens: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaCapTableSummary {
    pub known_alpha_wallets: u32,
    pub known_kols: u32,
    pub bundle_buyers: u32,
    pub buyer_quality_score: u32,
    pub confidence: String,
    pub signal: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaCapTableResponse {
    pub mint: String,
    pub buyers: Vec<AlphaCapTableBuyer>,
    pub summary: AlphaCapTableSummary,
    /// v0.23.4 — trade-coverage disclosure (`None` on older cached responses).
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaBuyerQualityBreakdown {
    pub alpha_wallet_count: u32,
    pub kol_count: u32,
    pub bundle_buyer_count: u32,
    pub avg_historical_win_rate: Option<f64>,
    pub bot_dominated: bool,
    /// First-20 buyers on the rolling dump-cluster list (wallets whose 5+
    /// recent first-20 appearances are exclusively on tokens that peaked
    /// <15 min after deploy; trailing 42d, refreshed daily). Out-of-sample:
    /// 3+ such wallets predicted a sub-15-min peak 94% of the time vs 61%
    /// base. Informational — does not move the score.
    #[serde(default)]
    pub dump_cluster_count: u32,
    /// First-20 buyers with 5+ recent first-20 appearances of any kind.
    /// Alone it predicts nothing; a heavily recycled cohort with
    /// `dump_cluster_count` 0 historically leans runner.
    #[serde(default)]
    pub recycled_early_buyer_count: u32,
}

/// Payload of a `token:graduation` stream event — every pump.fun graduation
/// (bonding curve complete → PumpSwap migration), tracked deployer or not.
/// Delivered on the `token:graduations` WebSocket channel (PRO+).
#[derive(Debug, Clone, Deserialize)]
pub struct GraduationEvent {
    pub token_mint: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub time_to_bond_minutes: Option<f64>,
    pub deployer_wallet: Option<String>,
    /// `"unranked"` when the deployer is unknown to deployer-hunter.
    pub deployer_tier: String,
    pub market_cap_usd: Option<f64>,
    pub bonded_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaBuyerQualityResponse {
    pub mint: String,
    pub score: u32,
    pub confidence: String,
    pub signal: String,
    pub cached_at: String,
    #[serde(default)]
    pub breakdown: Option<AlphaBuyerQualityBreakdown>,
    #[serde(default)]
    pub note: Option<String>,
    /// v0.23.4 — trade-coverage disclosure (`None` on older cached responses).
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaBuyerQualityBatchResponse {
    pub tokens: Vec<AlphaBuyerQualityResponse>,
    pub count: u32,
    /// Number of mints served from the shared 5-min LRU cache without a DB query.
    pub cache_hits: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Token intelligence (/token/{mint}) ─────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct TokenKolTopBuyer {
    pub name: String,
    pub sol_amount: f64,
    /// ULTRA only — individual KOL wallet address.
    #[serde(default)]
    pub wallet: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenKolActivity {
    pub buying_kols: u32,
    pub selling_kols: u32,
    pub net_flow_sol: f64,
    /// "accumulating" | "distributing" | "neutral".
    pub signal: String,
    pub top_buyers: Vec<TokenKolTopBuyer>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenDeployerInfo {
    pub wallet: String,
    pub tier: DeployerTier,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
    #[serde(default)]
    pub total_deployed: Option<u32>,
    #[serde(default)]
    pub total_bonded: Option<u32>,
    #[serde(default)]
    pub recent_bond_rate: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponseBody {
    pub mint: String,
    #[serde(default)]
    pub price_usd: Option<f64>,
    #[serde(default)]
    pub price_sol: Option<f64>,
    #[serde(default)]
    pub market_cap: Option<f64>,
    #[serde(default)]
    pub volume_24h_usd: Option<f64>,
    #[serde(default)]
    pub volume_24h_sol: Option<f64>,
    #[serde(default)]
    pub trades_24h: Option<u32>,
    #[serde(default)]
    pub last_trade_at: Option<String>,
    /// When the mint first appeared in our indexer.
    #[serde(default)]
    pub first_seen_at: Option<String>,
    #[serde(default)]
    pub age_seconds: Option<u64>,
    #[serde(default)]
    pub is_blacklisted: Option<bool>,
    /// "stablecoin" | "wrapped_sol" | "lst" | "rug" | custom category when blacklisted.
    #[serde(default)]
    pub blacklist_category: Option<String>,
    #[serde(default)]
    pub deployer: Option<TokenDeployerInfo>,
    pub kol_activity: TokenKolActivity,
    /// Liquidity-to-MC ratio (`liquidity_usd / market_cap`). Useful for spotting
    /// thin-liquidity tokens where price impact is high.
    #[serde(default)]
    pub liquidity_to_mc_ratio: Option<f64>,
    /// Total SOL spent by the first-20 buyers of this token.
    #[serde(default)]
    pub launch_cohort_sol: Option<f64>,
    /// Count of first-20 buyers of this token (0–20).
    #[serde(default)]
    pub launch_cohort_size: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub token: TokenResponseBody,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenBatchResponse {
    pub tokens: Vec<TokenResponseBody>,
    pub count: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Token risk score (/tokens/{mint}/risk) ─────────────────────────────────

/// Overall risk band for a token's rug-risk/safety score.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskBand {
    Safe,
    Caution,
    Danger,
}

impl RiskBand {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Caution => "caution",
            Self::Danger => "danger",
        }
    }
}

/// Per-factor status within a token risk breakdown.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskFactorStatus {
    Ok,
    Warn,
    Danger,
}

impl RiskFactorStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Warn => "warn",
            Self::Danger => "danger",
        }
    }
}

/// A single contributing factor to a token's risk score.
#[derive(Debug, Clone, Deserialize)]
pub struct RiskFactor {
    /// Stable machine-readable key (e.g. "mint_authority", "liquidity").
    pub key: String,
    /// Human-readable label.
    pub label: String,
    pub status: RiskFactorStatus,
    /// Points this factor contributed to the total risk score.
    pub points: i64,
    /// Explanatory detail for the factor.
    pub detail: String,
}

/// Raw signals the risk score was computed from. Each nullable field is `None`
/// when the underlying data was unavailable at scoring time.
#[derive(Debug, Clone, Deserialize)]
pub struct RiskInputs {
    #[serde(default)]
    pub mint_authority_revoked: Option<bool>,
    #[serde(default)]
    pub freeze_authority_revoked: Option<bool>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    #[serde(default)]
    pub liquidity_to_mc_ratio: Option<f64>,
    pub transfer_fee_bps: i64,
    pub is_token_2022: bool,
    pub burn_detected: bool,
    #[serde(default)]
    pub launch_cohort_sol: Option<f64>,
    pub launch_cohort_size: i64,
    #[serde(default)]
    pub deployer_bonding_rate: Option<f64>,
    #[serde(default)]
    pub deployer_total_deployed: Option<i64>,
    #[serde(default)]
    pub kol_signal: Option<String>,
    pub is_blacklisted: bool,
    /// v0.22 — slot-window launch-snipe rollup ([`SniperFootprint`]). `None`
    /// when no rollup exists for the mint; inside it, `data_available = false`
    /// means "not observable", NOT "0 snipes".
    #[serde(default)]
    pub sniper_footprint: Option<SniperFootprint>,
}

/// v0.23 — Deployer (dev) wallet activity block on a token's risk response.
/// Combines the create-tx dev-buy snapshot, the trade-derived dev-sell rollup
/// (~2 min lag), and a live on-chain holdings check answering "is the dev
/// wallet empty NOW". Every field is `None` when the underlying data is
/// unavailable — never a guess.
#[derive(Debug, Clone, Deserialize)]
pub struct RiskDev {
    /// Deployer wallet address (base58).
    #[serde(default)]
    pub wallet: Option<String>,
    /// Launchpad the token deployed on (e.g. `"pumpfun"`).
    #[serde(default)]
    pub launchpad: Option<String>,
    /// Deploy timestamp (ISO 8601).
    #[serde(default)]
    pub deployed_at: Option<String>,
    /// SOL the dev spent in the create-tx buy. `None` on rows pre-dating the
    /// snapshot or on launchpads without it.
    #[serde(default)]
    pub buy_sol: Option<f64>,
    /// Tokens the dev bought in the create tx (UI units).
    #[serde(default)]
    pub buy_tokens: Option<f64>,
    /// Create-tx dev buy as a percent of total supply.
    #[serde(default)]
    pub buy_supply_pct: Option<f64>,
    /// Tokens the dev bought AFTER the create tx (catches the
    /// same-second-separate-tx dev buy the snapshot reads as 0).
    #[serde(default)]
    pub bought_tokens_after: Option<f64>,
    /// Tokens the dev has sold (trade rollup, ~2 min lag).
    #[serde(default)]
    pub sold_tokens: Option<f64>,
    /// SOL the dev received selling (trade rollup, ~2 min lag).
    #[serde(default)]
    pub sold_sol: Option<f64>,
    /// First dev sell timestamp (ISO 8601).
    #[serde(default)]
    pub first_sell_at: Option<String>,
    /// Most recent dev sell timestamp (ISO 8601).
    #[serde(default)]
    pub last_sell_at: Option<String>,
    /// Live on-chain dev holdings of this mint (UI units). `None` = RPC
    /// unavailable right now.
    #[serde(default)]
    pub holdings_tokens: Option<f64>,
    /// `holdings_tokens` as percent of supply — pump.fun only (fixed 1B
    /// denominator); `None` for other launchpads rather than a wrong number.
    #[serde(default)]
    pub holdings_supply_pct: Option<f64>,
    /// `Some(true)` when the dev wallet holds <1 token on-chain right now.
    /// `None` = couldn't check.
    #[serde(default)]
    pub wallet_empty: Option<bool>,
    /// `Some(true)` when on-chain holdings are well below the trade-derived
    /// expectation — tokens likely moved out without a sell. Only computed
    /// when trade coverage exists AND the sell-rollup cursor is fresh;
    /// otherwise `None` (unknown), never a guess.
    #[serde(default)]
    pub transferred_out: Option<bool>,
}

/// Transparent 0–100 token rug-risk/safety score (PRO/ULTRA). Higher = riskier.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenRisk {
    pub mint: String,
    /// 0–100, higher = riskier.
    pub risk_score: u32,
    pub band: RiskBand,
    /// Per-factor breakdown that sums into `risk_score`.
    pub factors: Vec<RiskFactor>,
    /// Raw inputs the score was derived from.
    pub inputs: RiskInputs,
    pub score_version: String,
    /// v0.23 — deployer wallet activity ([`RiskDev`]): dev buy/sell rollup +
    /// live on-chain holdings. `None` when the mint has no tracked deploy row.
    #[serde(default)]
    pub dev: Option<RiskDev>,
    /// v0.23.4 — trade-coverage disclosure (single-mint `/risk` only). Its
    /// `note` names the split: trade-derived sub-fields are launchpad-pipeline
    /// scoped, on-chain sub-fields are unaffected by trade coverage.
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
    pub as_of: String,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MintBatchRequest {
    /// 1–50 base58 Solana token mint addresses.
    pub mints: Vec<String>,
}

// ─── Batch token risk (/tokens/batch/risk) ──────────────────────────────────

/// One entry in a [`Token::batch_risk`](crate::api::token::Token::batch_risk)
/// response — either a scored result or an error entry for an untracked mint.
///
/// A scored result populates the risk fields (`risk_score`, `band`, `factors`,
/// `inputs`, `score_version`, `as_of`) exactly like [`TokenRisk`], with `error`
/// set to `None`. An untracked mint comes back as `{ mint, error:
/// Some("not_tracked") }` with the risk fields left `None`/empty — it does
/// **not** fail the batch. Use [`is_error`](Self::is_error), or match on
/// `error`, to tell the two apart.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchRiskResult {
    pub mint: String,
    /// 0–100, higher = riskier. `None` on an error entry.
    #[serde(default)]
    pub risk_score: Option<u32>,
    #[serde(default)]
    pub band: Option<RiskBand>,
    /// Per-factor breakdown that sums into `risk_score`. Empty on an error entry.
    #[serde(default)]
    pub factors: Vec<RiskFactor>,
    /// Raw inputs the score was derived from. `None` on an error entry.
    #[serde(default)]
    pub inputs: Option<RiskInputs>,
    #[serde(default)]
    pub score_version: Option<String>,
    /// ISO-8601 scoring timestamp. `None` on an error entry.
    #[serde(default)]
    pub as_of: Option<String>,
    /// `Some("not_tracked")` when the mint isn't tracked; `None` for a scored result.
    #[serde(default)]
    pub error: Option<String>,
}

impl BatchRiskResult {
    /// `true` when this entry is an error (untracked mint) rather than a score.
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
}

/// Response of [`Token::batch_risk`](crate::api::token::Token::batch_risk).
#[derive(Debug, Clone, Deserialize)]
pub struct BatchRiskResponse {
    /// One entry per unique input mint, in de-duplicated input order.
    pub tokens: Vec<BatchRiskResult>,
    /// Number of unique mints returned.
    pub count: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Token bundle intelligence (/tokens/{mint}/bundle) ──────────────────────

/// How a token's early cohort of wallets bought in — the strongest coordination
/// signal detected. `AtomicTx` means multiple wallets bought within a single
/// transaction (a classic bundle); `SameSlot` means they landed in the same
/// slot without sharing a tx; `None` means no coordinated cohort was detected.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BundleKind {
    AtomicTx,
    SameSlot,
    None,
}

impl BundleKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AtomicTx => "atomic_tx",
            Self::SameSlot => "same_slot",
            Self::None => "none",
        }
    }
}

/// Aggregate summary of the bundled cohort for a token.
#[derive(Debug, Clone, Deserialize)]
pub struct BundleSummary {
    /// Number of wallets in the detected bundle cohort.
    pub wallet_count: u32,
    /// The kind of coordination detected across the cohort.
    pub bundle_kind: BundleKind,
    /// Fraction of supply (0–1) the cohort still holds. `None` when unknown.
    #[serde(default)]
    pub held_ratio: Option<f64>,
    /// Percent of total supply the cohort still holds. `None` when unknown.
    #[serde(default)]
    pub held_pct_of_supply: Option<f64>,
    /// `true` when the entire cohort has fully exited.
    pub fully_exited: bool,
    /// Total buy volume attributed to the cohort.
    pub buy_volume: f64,
    /// Tokens still held by the cohort.
    pub tokens_held: f64,
}

/// A single wallet within a token's bundled cohort. Identity fields
/// (`is_kol`, `kol_name`, `win_rate`, `bot_confidence`) are populated on ULTRA;
/// on lower tiers they may be `None`.
#[derive(Debug, Clone, Deserialize)]
pub struct BundleWallet {
    /// 1-based rank within the cohort (by holdings / buy volume).
    pub rank: u32,
    pub wallet: String,
    /// Fraction of supply (0–1) this wallet still holds. `None` when unknown.
    #[serde(default)]
    pub held_ratio: Option<f64>,
    /// `true` when this wallet has sold any of its position.
    pub has_sold: bool,
    /// `true` when this wallet bought in the same atomic transaction as the cohort.
    pub atomic: bool,
    /// `true` when this wallet is a tracked KOL.
    pub is_kol: bool,
    /// KOL display name. `None` on lower tiers or when not a KOL.
    #[serde(default)]
    pub kol_name: Option<String>,
    /// KOL win rate. `None` on lower tiers or when unavailable.
    #[serde(default)]
    pub win_rate: Option<f64>,
    /// Bot-confidence classification. `None` on lower tiers or when unavailable.
    #[serde(default)]
    pub bot_confidence: Option<String>,
    /// Tokens still held by this wallet.
    pub tokens_held: f64,
}

/// Bundle intelligence for a token (PRO/ULTRA): detects wallets that bought in
/// the same atomic transaction or same slot, how much of supply they still
/// hold, and whether they've fully exited. **ULTRA** populates the per-wallet
/// identity fields; lower tiers may return an empty `wallets` array.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenBundle {
    pub mint: String,
    /// Aggregate summary of the bundled cohort.
    pub bundle: BundleSummary,
    /// Per-wallet breakdown of the cohort. Empty on lower tiers.
    #[serde(default)]
    pub wallets: Vec<BundleWallet>,
    /// v0.23.4 — trade-coverage disclosure (`None` on older cached responses).
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Token pools (/tokens/{mint}/pools) ─────────────────────────────────────

/// A single liquidity pool for a token across a DEX.
#[derive(Debug, Clone, Deserialize)]
pub struct Pool {
    pub pool_address: String,
    pub dex: String,
    pub quote_mint: String,
    /// Pool liquidity in USD. `None` when unknown.
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    /// Last observed price in SOL. `None` when unknown.
    #[serde(default)]
    pub last_price_sol: Option<f64>,
    /// Timestamp of the last swap (ISO 8601). `None` when unknown.
    #[serde(default)]
    pub last_swap_at: Option<String>,
    /// AMM identifier for the pool. `None` when unavailable.
    #[serde(default)]
    pub amm_id: Option<String>,
    /// `true` when the pool is currently active.
    pub is_active: bool,
}

/// Aggregate summary across all of a token's pools.
#[derive(Debug, Clone, Deserialize)]
pub struct PoolsSummary {
    pub pool_count: i64,
    pub active_pool_count: i64,
    pub dex_count: i64,
    pub dexes: Vec<String>,
    /// Total liquidity across all pools in USD. `None` when unknown.
    #[serde(default)]
    pub total_liquidity_usd: Option<f64>,
    /// Address of the primary (deepest) pool. `None` when unknown.
    #[serde(default)]
    pub primary_pool: Option<String>,
    /// DEX of the primary pool. `None` when unknown.
    #[serde(default)]
    pub primary_dex: Option<String>,
    /// Share of total liquidity held by the top pool (percent). `None` when unknown.
    #[serde(default)]
    pub top_pool_share_pct: Option<f64>,
}

/// All liquidity pools for a token plus an aggregate summary.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenPoolsResponse {
    pub mint: String,
    pub pools: Vec<Pool>,
    pub summary: PoolsSummary,
}

// ─── Token holders (/tokens/{mint}/holders) ─────────────────────────────────

/// Wallet-intelligence label on a live holder. Labels come from MadeOnSol's
/// swap-ledger data — an empty `labels` vec means "unknown to us", NOT
/// "verified clean". `Other` catches labels added after this crate shipped.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HolderLabel {
    Deployer,
    Kol,
    EarlyBuyer,
    Buyer,
    Bundle,
    Bot,
    DumpCluster,
    #[serde(other)]
    Other,
}

/// Why an owner was excluded from the circulating denominator and listed in
/// `excluded`: `Pool` = vault authority of a known pool (`dex` + `pool_address`
/// set); `BondingCurve` = pump.fun / LaunchLab curve; `Burn` = incinerator /
/// system program; `ProgramAccount` = an off-curve owner we could not attribute
/// to a known pool (vault, escrow, staking, unknown pool).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HolderExcludedReason {
    Pool,
    BondingCurve,
    Burn,
    ProgramAccount,
    #[serde(other)]
    Other,
}

/// How the holder set was read. `GetProgramAccountsCensus` = full mint-scoped
/// census (exact `holder_count`, ranks 1–100 retained).
/// `GetTokenLargestAccounts` = the top-20 fallback used only when the provider
/// refuses the census for a mega-cap mint (`holder_count: None`,
/// `source.census_fallback_reason` set).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HoldersMethod {
    #[serde(rename = "getProgramAccounts_census")]
    GetProgramAccountsCensus,
    #[serde(rename = "getTokenLargestAccounts")]
    GetTokenLargestAccounts,
    #[serde(other)]
    Other,
}

/// One disclosed holder — token accounts merged per owner wallet.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenHolder {
    /// 1-based rank by balance among non-excluded owners.
    pub rank: i64,
    /// Owner wallet (owner of the token account(s)).
    pub owner: String,
    /// Every non-zero token account of this owner for the mint.
    #[serde(default)]
    pub token_accounts: Vec<String>,
    /// Raw u64 balance as a decimal STRING — never a float. Parse with
    /// `amount_raw.parse::<u64>()` / `u128`.
    pub amount_raw: String,
    /// UI amount (`amount_raw / 10^decimals`); `None` when decimals are unknown.
    #[serde(default)]
    pub amount: Option<f64>,
    /// Share of TOTAL supply, 0–100.
    #[serde(default)]
    pub pct_of_supply: Option<f64>,
    /// Share of circulating supply (total minus pools/curves/burns), 0–100.
    #[serde(default)]
    pub pct_of_circulating: Option<f64>,
    /// MadeOnSol labels. Empty = unknown to us, not verified clean.
    #[serde(default)]
    pub labels: Vec<HolderLabel>,
    #[serde(default)]
    pub kol_name: Option<String>,
    /// This owner's rank in the token's early-buyer cohort, if any.
    #[serde(default)]
    pub early_buyer_rank: Option<i64>,
    /// Alpha-wallet classifier enum (`"none"`/`"low"`/`"medium"`/`"high"`), not a number.
    #[serde(default)]
    pub bot_confidence: Option<String>,
    #[serde(default)]
    pub historical_win_rate: Option<f64>,
}

/// An owner excluded from the circulating denominator (pool / bonding curve /
/// burn / unattributed program account), NAMED where possible.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenHoldersExcluded {
    pub owner: String,
    #[serde(default)]
    pub token_accounts: Vec<String>,
    /// Raw u64 balance as a decimal STRING — never a float.
    pub amount_raw: String,
    /// Share of TOTAL supply, 0–100.
    #[serde(default)]
    pub pct_of_supply: Option<f64>,
    pub reason: HolderExcludedReason,
    /// pumpfun | pumpswap | raydium | launchlab | meteora | orca | jupiter — when named.
    #[serde(default)]
    pub dex: Option<String>,
    #[serde(default)]
    pub pool_address: Option<String>,
}

/// Concentration over the FULL owner set — tier only governs how many ranks
/// are disclosed in `holders`; these numbers are identical on
/// PRO/ULTRA/BUSINESS. All `*_share` / `*_pct` values are 0–100 (or `None`
/// when the denominator is unknown).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenHoldersConcentration {
    /// Exact distinct non-zero owners minus excluded pools/curves/burns, at
    /// `slot` (census). `None` ONLY when the provider refused the census (see
    /// `source.census_fallback_reason`) — never estimated from trades.
    #[serde(default)]
    pub holder_count: Option<i64>,
    /// `"census"` when `holder_count` is exact; `None` on the top-20 fallback.
    #[serde(default)]
    pub holder_count_source: Option<String>,
    #[serde(default)]
    pub token_accounts_nonzero: Option<i64>,
    /// Total supply, raw u64 as a string.
    #[serde(default)]
    pub supply_raw: Option<String>,
    /// Supply minus excluded owners, raw u64 as a string.
    #[serde(default)]
    pub circulating_raw: Option<String>,
    #[serde(default)]
    pub decimals: Option<i64>,
    /// Shares of CIRCULATING supply held by ranks 1 / 1–10 / 1–20.
    #[serde(default)]
    pub top1_share: Option<f64>,
    #[serde(default)]
    pub top10_share: Option<f64>,
    #[serde(default)]
    pub top20_share: Option<f64>,
    /// Census only — `None` on the top-20 fallback.
    #[serde(default)]
    pub top50_share: Option<f64>,
    /// Census only — `None` on the top-20 fallback.
    #[serde(default)]
    pub top100_share: Option<f64>,
    /// Share of TOTAL supply in excluded owners (= pool_pct + burned_pct + program_pct).
    #[serde(default)]
    pub pool_and_program_pct: Option<f64>,
    /// Share of total supply in NAMED pools + bonding curves.
    #[serde(default)]
    pub pool_pct: Option<f64>,
    /// Share of total supply at burn addresses.
    #[serde(default)]
    pub burned_pct: Option<f64>,
    /// Share of total supply held by off-curve owners we could not attribute.
    #[serde(default)]
    pub program_pct: Option<f64>,
    /// Shares of circulating supply held by labelled cohorts.
    #[serde(default)]
    pub deployer_pct: Option<f64>,
    #[serde(default)]
    pub kol_pct: Option<f64>,
    #[serde(default)]
    pub early_buyer_pct: Option<f64>,
    #[serde(default)]
    pub bundle_pct: Option<f64>,
    #[serde(default)]
    pub bot_pct: Option<f64>,
    #[serde(default)]
    pub dump_cluster_pct: Option<f64>,
    #[serde(default)]
    pub distinct_owners_in_top20: i64,
    /// How many ranked owners the scan retained (≤100 census, ≤20 fallback).
    #[serde(default)]
    pub ranked_owners_available: i64,
}

/// Tracked deployer of the mint, when known.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenHoldersDeployer {
    pub wallet: String,
    pub tier: String,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
}

/// How the holder set was produced.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenHoldersSource {
    pub method: HoldersMethod,
    /// SPL Token or Token-2022 program id the mint lives under (pump.fun = Token-2022).
    #[serde(default)]
    pub token_program: Option<String>,
    /// Ranks retained by the scan: 100 (census) or 20 (fallback).
    #[serde(default)]
    pub rpc_cap: i64,
    #[serde(default)]
    pub commitment: String,
    #[serde(default)]
    pub scan_ms: i64,
    /// Set when the census was refused and the top-20 fallback was served.
    #[serde(default)]
    pub census_fallback_reason: Option<String>,
    #[serde(default)]
    pub note: String,
}

/// `GET /tokens/{mint}/holders` — live holder census + concentration for a
/// Solana mint, read from the ledger at `confirmed` (who holds NOW, as
/// opposed to `alpha.cap_table`, which is who bought first).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenHoldersResponse {
    pub mint: String,
    /// Ledger slot the holder set was read at.
    pub slot: i64,
    pub as_of: String,
    /// Disclosed ranks: PRO 1–10, ULTRA 1–50, BUSINESS 1–100 (≤20 on the fallback).
    #[serde(default)]
    pub holders: Vec<TokenHolder>,
    pub count: i64,
    /// Rank cap for your tier: 10 PRO, 50 ULTRA, 100 BUSINESS.
    pub disclosed: i64,
    /// Pools / bonding curves / burns / unattributed program accounts —
    /// excluded from the circulating denominator, named where possible.
    #[serde(default)]
    pub excluded: Vec<TokenHoldersExcluded>,
    pub concentration: TokenHoldersConcentration,
    #[serde(default)]
    pub deployer: Option<TokenHoldersDeployer>,
    pub source: TokenHoldersSource,
}

// ─── Token locks & vesting (/tokens/{mint}/locks, /tokens/locks, /tokens/unlocks) — v0.26 ───

/// Locker program a lock / vesting contract lives in. `Other` catches
/// programs added after this crate shipped.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LockProgram {
    Streamflow,
    JupiterLock,
    BonfidaVesting,
    #[serde(other)]
    Other,
}

impl LockProgram {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Streamflow => "streamflow",
            Self::JupiterLock => "jupiter_lock",
            Self::BonfidaVesting => "bonfida_vesting",
            Self::Other => "other",
        }
    }
}

/// `Lock` = the whole amount unlocks at one date; `Vesting` = cliff and/or
/// periodic release (Streamflow's own lock-vs-vesting rule is mirrored).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LockKind {
    Lock,
    Vesting,
    #[serde(other)]
    Other,
}

/// Contract status, derived at request time from the schedule and the
/// on-chain withdrawn / cancelled / closed state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LockStatus {
    Active,
    Completed,
    Cancelled,
    Closed,
    #[serde(other)]
    Other,
}

/// Kind of unlock event: `Cliff` (cliff amount released), `Period` (one
/// periodic release, period ≥ 1h), `Final` (last release / end of schedule),
/// `Tranche` (Bonfida schedule tranche).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UnlockEventKind {
    Cliff,
    Period,
    Final,
    Tranche,
    #[serde(other)]
    Other,
}

/// Window for [`Token::unlocks`](crate::api::token::Token::unlocks)
/// (`within=`). Defaults to `7d` server-side when unset.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UnlockWindow {
    #[serde(rename = "1h")]
    H1,
    #[serde(rename = "6h")]
    H6,
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "3d")]
    D3,
    #[default]
    #[serde(rename = "7d")]
    D7,
    #[serde(rename = "14d")]
    D14,
    #[serde(rename = "30d")]
    D30,
    #[serde(rename = "90d")]
    D90,
}

impl UnlockWindow {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H1 => "1h",
            Self::H6 => "6h",
            Self::H24 => "24h",
            Self::D3 => "3d",
            Self::D7 => "7d",
            Self::D14 => "14d",
            Self::D30 => "30d",
            Self::D90 => "90d",
        }
    }
}

/// Sort order for [`Token::unlocks`](crate::api::token::Token::unlocks).
/// Defaults to [`UnlocksSort::Soonest`] server-side when unset.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum UnlocksSort {
    #[default]
    Soonest,
    LargestUsd,
    LargestPct,
}

impl UnlocksSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Soonest => "soonest",
            Self::LargestUsd => "largest_usd",
            Self::LargestPct => "largest_pct",
        }
    }
}

/// Mint facts + price attached to lock responses. `decimals` / `supply` come
/// from the SPL Mint account (most locked tokens are outside the pump/DEX
/// universe), `price_usd` / `market_cap_usd` from MadeOnSol price data — all
/// `None` when unknown. `facts_resolved` (per-mint response only) is `false`
/// while decimals/supply are still unresolved, in which case every ui / usd /
/// pct field on the rows is `None`.
#[derive(Debug, Clone, Deserialize)]
pub struct LockTokenInfo {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub decimals: Option<i64>,
    #[serde(default)]
    pub price_usd: Option<f64>,
    /// UI-scaled total supply (per-mint response only).
    #[serde(default)]
    pub supply: Option<f64>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    /// Per-mint response only; absent (`false`) on feed / unlock rows.
    #[serde(default)]
    pub facts_resolved: bool,
}

/// The next unlock event of a contract (or, in
/// [`TokenLocksSummary::next_unlock`], the nearest one across all contracts —
/// then `lock_account` names it).
#[derive(Debug, Clone, Deserialize)]
pub struct LockNextUnlock {
    /// ISO 8601.
    pub at: String,
    pub kind: UnlockEventKind,
    /// Base units as a decimal STRING — never a float.
    pub amount_raw: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amount_usd: Option<f64>,
    /// Only on the summary-level `next_unlock`.
    #[serde(default)]
    pub lock_account: Option<String>,
}

/// One on-chain lock / vesting contract with a live-derived view. Every
/// `*_raw` field is base units as a decimal STRING; the ui (`amount`,
/// `locked`, …), `*_usd` and `*_pct_of_supply` companions are `None` when
/// decimals or price are unknown.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenLock {
    /// The contract account (Streamflow stream / Jupiter `VestingEscrow` /
    /// Bonfida vesting account).
    pub lock_account: String,
    pub program: LockProgram,
    pub kind: LockKind,
    /// Derived at request time.
    pub status: LockStatus,
    pub mint: String,
    /// Creator / locker (Bonfida has none on-chain).
    #[serde(default)]
    pub sender: Option<String>,
    #[serde(default)]
    pub recipient: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    /// Deposited amount, base units.
    pub amount_raw: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amount_usd: Option<f64>,
    #[serde(default)]
    pub amount_pct_of_supply: Option<f64>,
    /// Still locked right now (amount − unlocked-so-far); `"0"` unless active.
    pub locked_raw: String,
    #[serde(default)]
    pub locked: Option<f64>,
    #[serde(default)]
    pub locked_usd: Option<f64>,
    #[serde(default)]
    pub locked_pct_of_supply: Option<f64>,
    pub unlocked_raw: String,
    #[serde(default)]
    pub unlocked: Option<f64>,
    /// Claimed so far.
    pub withdrawn_raw: String,
    #[serde(default)]
    pub withdrawn: Option<f64>,
    /// Unlocked but not yet withdrawn.
    pub claimable_raw: String,
    #[serde(default)]
    pub claimable: Option<f64>,
    #[serde(default)]
    pub start_at: Option<String>,
    #[serde(default)]
    pub cliff_at: Option<String>,
    /// Fully unlocked at; `None` = perpetual / no schedule.
    #[serde(default)]
    pub end_at: Option<String>,
    #[serde(default)]
    pub period_seconds: Option<i64>,
    /// `true` when period < 1h (per-second stream).
    #[serde(default)]
    pub continuous: bool,
    #[serde(default)]
    pub amount_per_period_raw: Option<String>,
    #[serde(default)]
    pub amount_per_period: Option<f64>,
    #[serde(default)]
    pub cliff_amount_raw: Option<String>,
    #[serde(default)]
    pub cliff_amount: Option<f64>,
    #[serde(default)]
    pub perpetual: bool,
    #[serde(default)]
    pub next_unlock: Option<LockNextUnlock>,
    /// The locker can cancel — funds are locked against the recipient, not
    /// the locker. A cancelable lock is a weaker promise.
    #[serde(default)]
    pub cancelable_by_sender: Option<bool>,
    #[serde(default)]
    pub cancelable_by_recipient: Option<bool>,
    #[serde(default)]
    pub transferable: Option<bool>,
    #[serde(default)]
    pub can_topup: Option<bool>,
    #[serde(default)]
    pub cancelled_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    /// Backfilled row with no on-chain creation time (Jupiter Lock).
    #[serde(default)]
    pub created_at_estimated: bool,
    #[serde(default)]
    pub tx_signature: Option<String>,
    /// Per-row token block — present on [`TokenLocksFeedResponse`] rows only
    /// (the per-mint response carries one [`TokenLocksResponse::token`]).
    #[serde(default)]
    pub token: Option<LockTokenInfo>,
}

/// Roll-up over every contract on the mint (the `status` / `program` filters
/// narrow the `locks` list, never the summary).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenLocksSummary {
    /// Exact count of contracts on the mint.
    pub lock_count: i64,
    /// `false` when the mint holds more than 5000 contracts — totals then
    /// cover the newest 5000 (`rows_considered`).
    #[serde(default = "default_true")]
    pub complete: bool,
    #[serde(default)]
    pub rows_considered: i64,
    #[serde(default)]
    pub active_count: i64,
    #[serde(default)]
    pub by_program: HashMap<String, i64>,
    #[serde(default)]
    pub by_kind: HashMap<String, i64>,
    #[serde(default)]
    pub distinct_lockers: i64,
    /// Base units as a decimal STRING.
    pub locked_raw: String,
    #[serde(default)]
    pub locked: Option<f64>,
    #[serde(default)]
    pub locked_usd: Option<f64>,
    #[serde(default)]
    pub locked_pct_of_supply: Option<f64>,
    pub deposited_raw: String,
    #[serde(default)]
    pub deposited: Option<f64>,
    #[serde(default)]
    pub deposited_usd: Option<f64>,
    /// Forward unlock schedule — total releasing in the next 7 days.
    pub unlocking_7d_raw: String,
    #[serde(default)]
    pub unlocking_7d: Option<f64>,
    #[serde(default)]
    pub unlocking_7d_usd: Option<f64>,
    #[serde(default)]
    pub unlocking_7d_pct_of_supply: Option<f64>,
    /// Forward unlock schedule — total releasing in the next 30 days.
    pub unlocking_30d_raw: String,
    #[serde(default)]
    pub unlocking_30d: Option<f64>,
    #[serde(default)]
    pub unlocking_30d_usd: Option<f64>,
    #[serde(default)]
    pub unlocking_30d_pct_of_supply: Option<f64>,
    /// Nearest next unlock across all active contracts (`lock_account` set).
    #[serde(default)]
    pub next_unlock: Option<LockNextUnlock>,
    /// Active contracts the sender can still cancel.
    #[serde(default)]
    pub active_cancelable_by_sender: i64,
}

fn default_true() -> bool {
    true
}

/// `GET /tokens/{mint}/locks` — every lock / vesting contract on a mint
/// (Streamflow, Jupiter Lock, Bonfida vesting) + summary. **LP locks are not
/// included.**
#[derive(Debug, Clone, Deserialize)]
pub struct TokenLocksResponse {
    pub mint: String,
    pub token: LockTokenInfo,
    pub summary: TokenLocksSummary,
    #[serde(default)]
    pub locks: Vec<TokenLock>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

/// Query params for [`Token::locks`](crate::api::token::Token::locks).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TokenLocksParams {
    /// Filter the list (the summary always covers all rows).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LockStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<LockProgram>,
    /// 1–500; default 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Query params for [`Token::locks_feed`](crate::api::token::Token::locks_feed).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TokenLocksFeedParams {
    /// ISO 8601 — only contracts created after this instant (use
    /// `pagination.next_since` to poll forward).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// ISO 8601 — page back: only contracts created before this instant
    /// (use `pagination.next_before`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint: Option<String>,
    /// Creator / locker wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<LockProgram>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<LockKind>,
    /// Post-filter (×4 over-fetch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LockStatus>,
    /// Deposited amount ≥ USD (needs a known price). Post-filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_usd: Option<f64>,
    /// Deposited amount ≥ this % of supply, 0–100. Post-filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pct_of_supply: Option<f64>,
    /// Include backfilled Jupiter Lock rows that have no on-chain creation
    /// time (`created_at_estimated`); excluded by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_estimated: Option<bool>,
    /// 1–100; default 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Time-cursor pagination shared by the lock feed and the fee-claim feed:
/// pass `next_since` back as `since` to poll forward, `next_before` as
/// `before` to page back.
#[derive(Debug, Clone, Deserialize)]
pub struct TimeCursorPagination {
    pub limit: u32,
    pub count: u32,
    pub has_more: bool,
    #[serde(default)]
    pub next_since: Option<String>,
    #[serde(default)]
    pub next_before: Option<String>,
}

/// WebSocket pointer attached to feed responses that are also pushed live —
/// `channel` is what to put in the subscribe frame (`token:locks`,
/// `token:fee_claims`), `url` the stream endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct StreamPointer {
    pub channel: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub token_endpoint: Option<String>,
    #[serde(default)]
    pub subscribe: Option<serde_json::Value>,
    #[serde(default)]
    pub note: Option<String>,
}

/// `GET /tokens/locks` — cross-token feed of NEW lock / vesting contracts,
/// newest first. Same rows as [`TokenLocksResponse::locks`] plus a per-row
/// [`TokenLock::token`] block. Pushed live as `token:lock` on the
/// `token:locks` channel ([`TokenLockEvent`]).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenLocksFeedResponse {
    #[serde(default)]
    pub locks: Vec<TokenLock>,
    pub pagination: TimeCursorPagination,
    #[serde(default)]
    pub stream: Option<StreamPointer>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

/// Query params for [`Token::unlocks`](crate::api::token::Token::unlocks).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TokenUnlocksParams {
    /// Window; default `7d`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub within: Option<UnlockWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<LockProgram>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<LockKind>,
    /// Next-event amount ≥ USD (needs a known price).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_usd: Option<f64>,
    /// Next-event amount ≥ this % of supply, 0–100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pct_of_supply: Option<f64>,
    /// Default `soonest`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<UnlocksSort>,
    /// 1–200; default 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// The `[from, to]` window an unlock query covered.
#[derive(Debug, Clone, Deserialize)]
pub struct UnlockWindowInfo {
    /// Echo of `within` (`"7d"` …).
    pub within: String,
    pub from: String,
    pub to: String,
}

/// The contract an unlock event belongs to — a subset of a [`TokenLock`] row.
#[derive(Debug, Clone, Deserialize)]
pub struct UnlockLockRef {
    pub lock_account: String,
    pub program: LockProgram,
    pub kind: LockKind,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub sender: Option<String>,
    #[serde(default)]
    pub recipient: Option<String>,
    /// Deposited amount, base units as a decimal STRING.
    pub amount_raw: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amount_usd: Option<f64>,
    /// Still locked right now, base units.
    pub locked_raw: String,
    #[serde(default)]
    pub locked: Option<f64>,
    #[serde(default)]
    pub locked_usd: Option<f64>,
    #[serde(default)]
    pub cliff_at: Option<String>,
    #[serde(default)]
    pub end_at: Option<String>,
    #[serde(default)]
    pub period_seconds: Option<i64>,
    #[serde(default)]
    pub continuous: bool,
    #[serde(default)]
    pub cancelable_by_sender: Option<bool>,
}

/// One upcoming unlock event: an active contract's NEXT release inside the
/// window (`amount_*`) plus that contract's total release over the whole
/// window (`window_amount_*`). Base units are decimal STRINGS; ui / usd /
/// pct are `None` when decimals or price are unknown.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenUnlock {
    /// ISO 8601.
    pub unlock_at: String,
    /// Seconds from the request instant until `unlock_at`.
    pub in_seconds: i64,
    pub event: UnlockEventKind,
    pub amount_raw: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amount_usd: Option<f64>,
    #[serde(default)]
    pub amount_pct_of_supply: Option<f64>,
    pub window_amount_raw: String,
    #[serde(default)]
    pub window_amount: Option<f64>,
    #[serde(default)]
    pub window_amount_usd: Option<f64>,
    #[serde(default)]
    pub window_amount_pct_of_supply: Option<f64>,
    pub mint: String,
    pub token: LockTokenInfo,
    pub lock: UnlockLockRef,
}

/// Pagination block of [`TokenUnlocksResponse`]. `total_in_window` counts
/// every matching event in the window, not just the returned page.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenUnlocksPagination {
    pub limit: u32,
    pub count: u32,
    pub total_in_window: i64,
    pub has_more: bool,
}

/// `GET /tokens/unlocks` — upcoming unlock events across all active
/// contracts inside a window. Continuous per-second streams (Streamflow
/// payroll) contribute only their cliff / final events.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenUnlocksResponse {
    pub window: UnlockWindowInfo,
    #[serde(default)]
    pub unlocks: Vec<TokenUnlock>,
    pub pagination: TokenUnlocksPagination,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

/// Payload of a `token:lock` stream event — one frame per NEW Streamflow /
/// Jupiter Lock / Bonfida lock or vesting contract, emitted the moment the
/// account is first seen (~seconds after the create tx). Delivered on the
/// `token:locks` WebSocket channel (PRO+). Updates (claims / cancels /
/// closes) are NOT pushed — poll
/// [`Token::locks`](crate::api::token::Token::locks) for the live state.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenLockEvent {
    pub lock_account: String,
    pub program: LockProgram,
    pub mint: String,
    pub kind: LockKind,
    #[serde(default)]
    pub sender: Option<String>,
    #[serde(default)]
    pub recipient: Option<String>,
    /// Base units as a decimal STRING.
    pub amount_raw: String,
    /// May be `None` on the very first sighting of a mint.
    #[serde(default)]
    pub decimals: Option<i64>,
    #[serde(default)]
    pub start_at: Option<String>,
    #[serde(default)]
    pub cliff_at: Option<String>,
    #[serde(default)]
    pub end_at: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tx_signature: Option<String>,
    #[serde(default)]
    pub slot: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
}

// ─── pump.fun creator-fee sharing (/tokens/{mint}/fee-shares, /tokens/fee-claims) — v0.26 ───

/// Where the `SharingConfig` in a [`TokenFeeSharesResponse`] came from:
/// `Stream` = our table (only NON-default splits are stored), `Chain` = a live
/// PDA read (so `is_default: true` — 100% to the creator — is a real answer).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeeConfigSource {
    Stream,
    Chain,
    #[serde(other)]
    Other,
}

/// pump.fun fee-event type. `CreatorClaim` (the plain creator vault claim —
/// per creator, carries no mint) is excluded from the feed unless requested
/// via [`TokenFeeClaimsParams::event_type`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeeEventType {
    SharesCreated,
    SharesUpdated,
    SharesReset,
    Distribution,
    SocialPdaCreated,
    SocialClaim,
    CreatorTransferred,
    CreatorClaim,
    #[serde(other)]
    Other,
}

impl FeeEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SharesCreated => "shares_created",
            Self::SharesUpdated => "shares_updated",
            Self::SharesReset => "shares_reset",
            Self::Distribution => "distribution",
            Self::SocialPdaCreated => "social_pda_created",
            Self::SocialClaim => "social_claim",
            Self::CreatorTransferred => "creator_transferred",
            Self::CreatorClaim => "creator_claim",
            Self::Other => "other",
        }
    }
}

/// Platform identity behind a `SocialFeePda` shareholder — fees earmarked
/// for e.g. an X account. `platform` 2 = X (`platform_label: "x"`); other
/// ids read `platform_<n>` until observed. `user_id` is the platform-native
/// numeric id, NOT the handle.
#[derive(Debug, Clone, Deserialize)]
pub struct FeeShareSocial {
    pub platform: i64,
    #[serde(default)]
    pub platform_label: Option<String>,
    pub user_id: String,
    /// Lifetime claimed by this identity, quote base units as a decimal STRING.
    #[serde(default)]
    pub lifetime_claimed_raw: Option<String>,
    #[serde(default)]
    pub lifetime_claimed: Option<f64>,
    #[serde(default)]
    pub lifetime_claimed_usd: Option<f64>,
    #[serde(default)]
    pub last_claimed_at: Option<String>,
}

/// One fee-share recipient — a current shareholder
/// ([`FeeSharingConfig::shareholders`]), a distribution recipient
/// ([`FeeDistributions::recipients`]) or a past recipient no longer in the
/// split ([`FeeDistributions::past_recipients`]; `share_bps` then `None`).
#[derive(Debug, Clone, Deserialize)]
pub struct FeeShareholder {
    pub address: String,
    #[serde(default)]
    pub share_bps: Option<i64>,
    #[serde(default)]
    pub share_pct: Option<f64>,
    /// The config admin (normally the coin creator).
    #[serde(default)]
    pub is_admin: bool,
    /// Address is a pump_fees `SocialFeePda` — fees earmarked for a platform
    /// identity ([`FeeShareholder::social`]).
    #[serde(default)]
    pub is_social_pda: bool,
    #[serde(default)]
    pub social: Option<FeeShareSocial>,
    /// Received via distributions since 2026-08-17, quote base units as a
    /// decimal STRING.
    #[serde(default)]
    pub received_raw: Option<String>,
    #[serde(default)]
    pub received: Option<f64>,
    #[serde(default)]
    pub received_usd: Option<f64>,
    #[serde(default)]
    pub payout_count: i64,
    #[serde(default)]
    pub last_payout_at: Option<String>,
}

/// The on-chain pump.fun `SharingConfig` of a coin (pump_fees PDA
/// `["sharing-config", mint]`).
#[derive(Debug, Clone, Deserialize)]
pub struct FeeSharingConfig {
    /// The SharingConfig PDA.
    pub sharing_config: String,
    #[serde(default)]
    pub admin: Option<String>,
    #[serde(default)]
    pub admin_revoked: Option<bool>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub version: Option<i64>,
    /// `Some(true)` = 100% to the admin — no redirect.
    #[serde(default)]
    pub is_default: Option<bool>,
    /// Share (bps) going to non-admin addresses.
    #[serde(default)]
    pub redirected_bps: i64,
    #[serde(default)]
    pub redirected_pct: f64,
    /// Share (bps) going to `SocialFeePda` addresses.
    #[serde(default)]
    pub social_bps: i64,
    #[serde(default)]
    pub social_pct: f64,
    #[serde(default)]
    pub shareholders: Vec<FeeShareholder>,
    pub source: FeeConfigSource,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Quote asset the fee amounts are denominated in (SOL unless a
/// stable-quoted coin).
#[derive(Debug, Clone, Deserialize)]
pub struct FeeQuote {
    pub symbol: String,
    pub decimals: i64,
    #[serde(default)]
    pub sol_usd: Option<f64>,
}

/// Roll-up of every `distribute_creator_fees` payout on the mint since
/// 2026-08-17 (pro-rata per shareholder).
#[derive(Debug, Clone, Deserialize)]
pub struct FeeDistributions {
    pub count: i64,
    /// Quote base units as a decimal STRING.
    pub total_raw: String,
    #[serde(default)]
    pub total: Option<f64>,
    #[serde(default)]
    pub total_usd: Option<f64>,
    #[serde(default)]
    pub last_at: Option<String>,
    /// Current shareholders with what each has received.
    #[serde(default)]
    pub recipients: Vec<FeeShareholder>,
    /// Addresses that received payouts but are no longer in the split.
    #[serde(default)]
    pub past_recipients: Vec<FeeShareholder>,
    #[serde(default)]
    pub payouts_considered: i64,
    #[serde(default)]
    pub payouts_truncated: bool,
}

/// One recent `distribute_creator_fees` payout.
#[derive(Debug, Clone, Deserialize)]
pub struct FeeRecentDistribution {
    pub at: String,
    pub tx_signature: String,
    /// Quote base units as a decimal STRING.
    pub amount_raw: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amount_usd: Option<f64>,
    /// Per-shareholder split of this payout (untyped — shape follows the
    /// decoded Anchor event).
    #[serde(default)]
    pub shareholders: Vec<serde_json::Value>,
    /// Transaction signer.
    #[serde(default)]
    pub actor: Option<String>,
}

/// `GET /tokens/{mint}/fee-shares` — pump.fun creator-fee sharing on a mint:
/// who receives what share of the creator fees, the distributions rollup and
/// the config change log. **Event history starts 2026-08-17.**
#[derive(Debug, Clone, Deserialize)]
pub struct TokenFeeSharesResponse {
    pub mint: String,
    /// `None` only when the live read failed on every RPC endpoint
    /// (`config_error` set).
    #[serde(default)]
    pub config: Option<FeeSharingConfig>,
    /// The SharingConfig PDA address.
    pub config_pda: String,
    #[serde(default)]
    pub config_error: Option<String>,
    pub quote: FeeQuote,
    pub distributions: FeeDistributions,
    /// Config changes + creator transfers, newest first (untyped rows).
    #[serde(default)]
    pub history: Vec<serde_json::Value>,
    #[serde(default)]
    pub recent_distributions: Vec<FeeRecentDistribution>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

/// Platform identity on a `social_claim` / `social_pda_created` fee event.
#[derive(Debug, Clone, Deserialize)]
pub struct FeeClaimSocial {
    /// Raw platform id (2 = X).
    pub platform: i64,
    #[serde(default)]
    pub platform_label: Option<String>,
    /// Platform-native numeric user id, not the handle.
    pub user_id: String,
    /// The `SocialFeePda` address.
    #[serde(default)]
    pub pda: Option<String>,
}

/// A shareholder as recorded on a config-change event.
#[derive(Debug, Clone, Deserialize)]
pub struct FeeClaimShareholder {
    pub address: String,
    pub share_bps: i64,
}

/// `distribution` only: the pro-rata amount paid to one shareholder.
#[derive(Debug, Clone, Deserialize)]
pub struct FeeClaimPayout {
    pub address: String,
    pub share_bps: i64,
    /// Quote base units as a decimal STRING.
    pub amount_raw: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amount_usd: Option<f64>,
}

/// One decoded pump.fun fee event on the feed.
#[derive(Debug, Clone, Deserialize)]
pub struct FeeClaimEvent {
    pub id: i64,
    #[serde(rename = "type")]
    pub event_type: FeeEventType,
    /// The event's own on-chain timestamp, ISO 8601.
    pub at: String,
    pub tx_signature: String,
    #[serde(default)]
    pub slot: Option<i64>,
    /// `None` for social claims and creator vault claims (per identity / per
    /// creator).
    #[serde(default)]
    pub mint: Option<String>,
    #[serde(default)]
    pub admin: Option<String>,
    /// Transaction signer.
    #[serde(default)]
    pub actor: Option<String>,
    /// Payout / claim recipient wallet, or the new creator.
    #[serde(default)]
    pub recipient: Option<String>,
    /// Quote base units as a decimal STRING (SOL lamports unless a
    /// stable-quoted coin).
    #[serde(default)]
    pub amount_raw: Option<String>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amount_usd: Option<f64>,
    /// Quote symbol (`"SOL"` …).
    #[serde(default)]
    pub quote: Option<String>,
    #[serde(default)]
    pub social: Option<FeeClaimSocial>,
    #[serde(default)]
    pub shareholders: Option<Vec<FeeClaimShareholder>>,
    /// `distribution` only: pro-rata amount per shareholder.
    #[serde(default)]
    pub payouts: Option<Vec<FeeClaimPayout>>,
    /// Full decoded Anchor event.
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
}

/// Query params for [`Token::fee_claims`](crate::api::token::Token::fee_claims).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TokenFeeClaimsParams {
    /// Comma list of event types (`"distribution,social_claim"`); default is
    /// every type except `creator_claim`. Serialised as `type=`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint: Option<String>,
    /// Payout / claim recipient wallet, or new creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// Transaction signer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    /// Raw platform id (2 = X).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_platform: Option<i64>,
    /// Platform-native numeric user id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_user_id: Option<String>,
    /// Amount floor in SOL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_sol: Option<f64>,
    /// ISO 8601 cursor (`pagination.next_since`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// ISO 8601 — page back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 1–100; default 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// `GET /tokens/fee-claims` — pump.fun fee-event feed, newest first. Pushed
/// live as `token:fee_claim` on the `token:fee_claims` channel
/// ([`TokenFeeClaimEvent`]). **History starts 2026-08-17.**
#[derive(Debug, Clone, Deserialize)]
pub struct TokenFeeClaimsResponse {
    #[serde(default)]
    pub events: Vec<FeeClaimEvent>,
    pub pagination: TimeCursorPagination,
    #[serde(default)]
    pub stream: Option<StreamPointer>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

/// Payload of a `token:fee_claim` stream event — one frame per stored
/// pump.fun fee event, emitted the moment the tx confirms. Delivered on the
/// `token:fee_claims` WebSocket channel (PRO+). NOTE: this is the writer's
/// raw row, not the enriched REST [`FeeClaimEvent`] — no `amount` /
/// `amount_usd`, and the social identity is flattened into
/// `social_platform` / `social_user_id` / `social_fee_pda`.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenFeeClaimEvent {
    pub id: i64,
    pub event_type: FeeEventType,
    pub tx_signature: String,
    #[serde(default)]
    pub slot: Option<i64>,
    /// The event's own on-chain timestamp, ISO 8601.
    pub block_time: String,
    /// `None` for social claims and creator vault claims.
    #[serde(default)]
    pub mint: Option<String>,
    #[serde(default)]
    pub sharing_config: Option<String>,
    #[serde(default)]
    pub admin: Option<String>,
    #[serde(default)]
    pub actor: Option<String>,
    #[serde(default)]
    pub recipient: Option<String>,
    /// Quote base units as a decimal STRING.
    #[serde(default)]
    pub amount_raw: Option<String>,
    #[serde(default)]
    pub quote_mint: Option<String>,
    #[serde(default)]
    pub social_platform: Option<i64>,
    #[serde(default)]
    pub social_user_id: Option<String>,
    #[serde(default)]
    pub social_fee_pda: Option<String>,
    #[serde(default)]
    pub shareholders: Option<serde_json::Value>,
}

// ─── Token surges & revivals (/tokens/surges, WS token:surges) — v0.27 ───

/// Kind of momentum fire. `Surge` = a token < 30 min old running hard vs its
/// launch MC; `Revival` = a token dormant ≥ 24 h that started trading again
/// (confirmed by real buys on the tape, never by a price mark). `Other`
/// catches kinds added after this crate shipped.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SurgeKind {
    Surge,
    Revival,
    #[serde(other)]
    Other,
}

impl SurgeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Surge => "surge",
            Self::Revival => "revival",
            Self::Other => "other",
        }
    }
}

/// Surge tier — each fires at most once per mint; tiers are independent (a
/// token can go straight to `Breakout`). Revivals carry no tier (`None`).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SurgeTier {
    /// ≤ 10 min old, ≥ $12k, ≥ 3× launch MC.
    Early,
    /// ≤ 30 min old, ≥ $30k, ≥ 6× launch MC AND ≥ 2× the lowest sample of
    /// the last 3 min — it is climbing now.
    Strong,
    /// ≤ 2 min old, ≥ $45k, ≥ 8× launch MC.
    Breakout,
    #[serde(other)]
    Other,
}

impl SurgeTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Early => "early",
            Self::Strong => "strong",
            Self::Breakout => "breakout",
            Self::Other => "other",
        }
    }
}

/// How the token's birth was established: `Sniper` = pre-confirm pump.fun
/// deploy (deshred), `Deployer` = confirmed on-chain create, `FirstSeen` =
/// the first trade the firehose observed (credible as a launch only on a
/// launchpad curve).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SurgeBirthSource {
    Sniper,
    Deployer,
    FirstSeen,
    #[serde(other)]
    Other,
}

/// `Launch` = the baseline is the first MC sample after birth (≤ 90 s);
/// `Late` = the engine first saw the token later (restart mid-life) — the
/// launch multiple is then NOT applied, USD floor + velocity only.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SurgeBaselineSource {
    Launch,
    Late,
    #[serde(other)]
    Other,
}

/// What the burst tape was measured on: 1-minute `Candles` (every DEX we
/// price) or live `WalletTrades` rows (pump-pipeline mints — used while the
/// open minute candle has not landed yet).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SurgeTapeSource {
    Candles,
    WalletTrades,
    #[serde(other)]
    Other,
}

/// Deployer reputation tier as carried on a fire (`Unranked` = no score yet).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SurgeDeployerTier {
    Elite,
    Good,
    Moderate,
    Rising,
    Cold,
    Unranked,
    #[serde(other)]
    Other,
}

impl SurgeDeployerTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Elite => "elite",
            Self::Good => "good",
            Self::Moderate => "moderate",
            Self::Rising => "rising",
            Self::Cold => "cold",
            Self::Unranked => "unranked",
            Self::Other => "other",
        }
    }
}

/// Risk flag a fire can carry — the honest half. Thresholds are echoed in
/// `definitions.risk_flags` on the response. `Other` catches flags added
/// after this crate shipped.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SurgeRiskFlag {
    /// ≥ 3 of the first-20 early buyers bought in the same block (surge only).
    BundledLaunch,
    /// < 8 unique buyers where wallet data exists, else < 10 buys on the tape.
    FewBuyers,
    /// ≥ 4 trades per wallet across < 15 wallets.
    WashPattern,
    /// Liquidity < $3k or liquidity / MC < 3%.
    ThinLiquidity,
    /// Deployer tier = cold.
    ColdDeployer,
    /// ≥ 3 early buyers are known sniper wallets.
    SniperHeavy,
    /// ≥ 50% of a ≥ 5-wallet early cohort has sold.
    EarlyBuyersExiting,
    /// More sells than buys on the tape at fire time.
    SellPressure,
    /// The price moved but no tape covering the window holds a parsed swap —
    /// real but unmeasured.
    NoTapeTrades,
    /// Revival with no pre-dormancy MC to compare against.
    NoPriorPrice,
    /// Mint authority not revoked.
    MintAuthorityActive,
    /// Token-2022 transfer fee > 0.
    TransferFee,
    #[serde(other)]
    Other,
}

impl SurgeRiskFlag {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BundledLaunch => "bundled_launch",
            Self::FewBuyers => "few_buyers",
            Self::WashPattern => "wash_pattern",
            Self::ThinLiquidity => "thin_liquidity",
            Self::ColdDeployer => "cold_deployer",
            Self::SniperHeavy => "sniper_heavy",
            Self::EarlyBuyersExiting => "early_buyers_exiting",
            Self::SellPressure => "sell_pressure",
            Self::NoTapeTrades => "no_tape_trades",
            Self::NoPriorPrice => "no_prior_price",
            Self::MintAuthorityActive => "mint_authority_active",
            Self::TransferFee => "transfer_fee",
            Self::Other => "other",
        }
    }
}

/// The burst tape — trades since birth (surge) or since the revival minute.
/// `available: false` = no tape covers the window yet (candle lag; every
/// count is `None`). `unique_buyers` / `unique_wallets` /
/// `trades_per_wallet` are `None` when the mint is outside wallet-trade
/// coverage (`wallet_data_available: false`) — never inferred.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SurgeTape {
    /// ISO 8601 start of the window.
    #[serde(default)]
    pub since: Option<String>,
    #[serde(default)]
    pub available: bool,
    #[serde(default)]
    pub source: Option<SurgeTapeSource>,
    #[serde(default)]
    pub buys: Option<i64>,
    #[serde(default)]
    pub sells: Option<i64>,
    #[serde(default)]
    pub trades: Option<i64>,
    #[serde(default)]
    pub buy_volume_usd: Option<f64>,
    #[serde(default)]
    pub sell_volume_usd: Option<f64>,
    #[serde(default)]
    pub volume_usd: Option<f64>,
    #[serde(default)]
    pub mev_volume_usd: Option<f64>,
    #[serde(default)]
    pub buy_sol: Option<f64>,
    #[serde(default)]
    pub sell_sol: Option<f64>,
    #[serde(default)]
    pub unique_buyers: Option<i64>,
    #[serde(default)]
    pub unique_wallets: Option<i64>,
    #[serde(default)]
    pub trades_per_wallet: Option<f64>,
    #[serde(default)]
    pub wallet_data_available: bool,
}

/// Tracked-KOL involvement on the tape (at most 10 `names`).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SurgeKol {
    #[serde(default)]
    pub buyers: i64,
    #[serde(default)]
    pub buys: i64,
    #[serde(default)]
    pub sells: i64,
    #[serde(default)]
    pub names: Vec<String>,
}

/// The first-20 early-buyer cohort at fire time.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SurgeEarlyBuyers {
    #[serde(default)]
    pub count: i64,
    /// Bought in the same block as ≥ 2 other early buyers.
    #[serde(default)]
    pub bundled: i64,
    #[serde(default)]
    pub cohort_sol: Option<f64>,
    /// Cohort wallets that have already sold.
    #[serde(default)]
    pub sold: i64,
    /// Early buyers that are known sniper wallets.
    #[serde(default)]
    pub sniper_wallets: i64,
}

/// Deployer reputation as carried on a fire ([`TokenSurgeEvent::deployer`]
/// is `None` when the deployer is unknown).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SurgeDeployer {
    #[serde(default)]
    pub wallet: Option<String>,
    #[serde(default)]
    pub tier: Option<SurgeDeployerTier>,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
    #[serde(default)]
    pub total_bonded: Option<i64>,
    #[serde(default)]
    pub total_deployed: Option<i64>,
    #[serde(default)]
    pub runner_rate: Option<f64>,
    #[serde(default)]
    pub labeled_tokens: Option<i64>,
    /// Last-10 outcome string, e.g. `"BDDBBDDDBD"` (B = bonded, D = dead).
    #[serde(default)]
    pub recent: Option<String>,
}

/// +1 h outcome, present on REST rows ≥ 65 min old (computed from candles
/// every 10 min). `priced_after_1h: false` = no candle in the hour — the
/// token stopped being priced, NOT zero. Never on the WebSocket event.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SurgeOutcome {
    #[serde(default)]
    pub computed_at: Option<String>,
    #[serde(default)]
    pub mc_usd_1h_after: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd_1h_after: Option<f64>,
    #[serde(default)]
    pub low_mc_usd_1h_after: Option<f64>,
    /// `mc_usd_1h_after ÷ market_cap_usd` at the fire.
    #[serde(default)]
    pub mc_1h_multiple: Option<f64>,
    /// `peak_mc_usd_1h_after ÷ market_cap_usd` at the fire.
    #[serde(default)]
    pub peak_1h_multiple: Option<f64>,
    #[serde(default)]
    pub priced_after_1h: bool,
}

/// One momentum fire — a row of [`TokenSurgesResponse::events`] and, minus
/// `outcome`, the payload of a `token:surge` / `token:revival` stream event
/// (see [`TokenSurgeStreamEvent`]). Both kinds share one shape: `tier` is
/// `None` on revivals; `dormant_hours` / `prev_mc_usd` /
/// `mc_vs_prev_multiple` are `None` on surges; `baseline_*` / `mc_multiple`
/// / `mc_change_3m_pct` are `None` on revivals. Nearly every scalar is an
/// `Option` — `None` means unknown, never zero.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenSurgeEvent {
    /// Row id. Always set on REST rows; may be `None` on a stream frame.
    #[serde(default)]
    pub id: Option<i64>,
    pub kind: SurgeKind,
    #[serde(default)]
    pub tier: Option<SurgeTier>,
    pub mint: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    /// Venue at birth / classification (what the first_seen rule was judged on).
    #[serde(default)]
    pub launchpad: Option<String>,
    /// Where it trades at fire time — a pump token that graduated is
    /// `pumpfun` / `pumpswap`.
    #[serde(default)]
    pub primary_dex: Option<String>,
    /// ISO 8601.
    pub fired_at: String,
    #[serde(default)]
    pub birth_at: Option<String>,
    #[serde(default)]
    pub birth_source: Option<SurgeBirthSource>,
    #[serde(default)]
    pub age_seconds: Option<i64>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    #[serde(default)]
    pub liquidity_to_mc_ratio: Option<f64>,
    #[serde(default)]
    pub price_usd: Option<f64>,
    /// Surge only: launch MC (`baseline_source: Launch`).
    #[serde(default)]
    pub baseline_mc_usd: Option<f64>,
    #[serde(default)]
    pub baseline_source: Option<SurgeBaselineSource>,
    /// Surge only: `market_cap_usd ÷ baseline_mc_usd`; `None` when the
    /// baseline is `Late`.
    #[serde(default)]
    pub mc_multiple: Option<f64>,
    /// Surge only: climb vs the lowest sample of the last 3 min, in percent.
    #[serde(default)]
    pub mc_change_3m_pct: Option<f64>,
    /// Revival only.
    #[serde(default)]
    pub dormant_hours: Option<f64>,
    /// Revival only: pre-dormancy candle close MC.
    #[serde(default)]
    pub prev_mc_usd: Option<f64>,
    /// Revival only: `market_cap_usd ÷ prev_mc_usd`.
    #[serde(default)]
    pub mc_vs_prev_multiple: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub pct_of_peak: Option<f64>,
    #[serde(default)]
    pub bonding_progress_pct: Option<f64>,
    #[serde(default)]
    pub is_bonded: Option<bool>,
    #[serde(default)]
    pub tape: SurgeTape,
    #[serde(default)]
    pub kol: SurgeKol,
    #[serde(default)]
    pub early_buyers: SurgeEarlyBuyers,
    /// `None` when the deployer is unknown.
    #[serde(default)]
    pub deployer: Option<SurgeDeployer>,
    /// Flat copies of `deployer.wallet` / `deployer.tier` (what the
    /// subscribe filter reads).
    #[serde(default)]
    pub deployer_wallet: Option<String>,
    #[serde(default)]
    pub deployer_tier: Option<SurgeDeployerTier>,
    #[serde(default)]
    pub mint_authority_revoked: Option<bool>,
    #[serde(default)]
    pub freeze_authority_revoked: Option<bool>,
    #[serde(default)]
    pub is_token_2022: Option<bool>,
    /// Unknown flags (added after this crate shipped) deserialize as
    /// [`SurgeRiskFlag::Other`].
    #[serde(default)]
    pub risk_flags: Vec<SurgeRiskFlag>,
    #[serde(default)]
    pub detail_url: Option<String>,
    /// `Some(false)` when the one-round-trip enrichment failed (tape / kol /
    /// early_buyers then carry zeros / `None`s).
    #[serde(default)]
    pub enrichment_available: Option<bool>,
    /// REST only — `None` until the fire is ≥ 65 min old.
    #[serde(default)]
    pub outcome: Option<SurgeOutcome>,
}

/// Payload of a `token:surge` / `token:revival` stream event (channel
/// `token:surges`, PRO+) — pushed by the surge-tracker the moment a fire is
/// confirmed. The same shape as the REST row; `outcome` is always `None`
/// on the stream (poll [`Token::surges`](crate::api::token::Token::surges)
/// for it once the fire is ≥ 65 min old).
pub type TokenSurgeStreamEvent = TokenSurgeEvent;

/// Query params for [`Token::surges`](crate::api::token::Token::surges).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TokenSurgesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<SurgeKind>,
    /// Surge only — the API answers 400 with `kind: Revival`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<SurgeTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint: Option<String>,
    /// ISO 8601 — only fires after this instant (use `pagination.next_since`
    /// to poll forward).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// ISO 8601 — page back: only fires before this instant (use
    /// `pagination.next_before`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
    /// Buys on the tape at fire time ≥.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buys: Option<u32>,
    /// Launchpad at birth, e.g. `"pumpfun"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launchpad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployer_tier: Option<SurgeDeployerTier>,
    /// Comma list of [`SurgeRiskFlag`] names (`"bundled_launch,sniper_heavy"`)
    /// — rows carrying ANY of them are dropped; an unknown flag is a 400
    /// with `known_flags[]`. Build it with [`SurgeRiskFlag::as_str`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_flags: Option<String>,
    /// Only rows with no risk flags at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_clean: Option<bool>,
    /// Also return per-(kind, tier) hit-rates ([`SurgeStats`]) over `days`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<bool>,
    /// Stats window, 1–30; default 7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u32>,
    /// 1–200; default 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// One per-(kind, tier) hit-rate row of [`SurgeStats`]. Only fires ≥ 65 min
/// old with a computed outcome count (`with_outcome`). Out-of-sample by
/// construction — the fire is recorded before the outcome exists.
#[derive(Debug, Clone, Deserialize)]
pub struct SurgeStatsRow {
    pub kind: SurgeKind,
    #[serde(default)]
    pub tier: Option<SurgeTier>,
    #[serde(default)]
    pub fires: i64,
    #[serde(default)]
    pub with_outcome: i64,
    /// MC one hour after the fire ≥ MC at the fire.
    #[serde(default)]
    pub up_1h: i64,
    #[serde(default)]
    pub up_1h_pct: Option<f64>,
    #[serde(default)]
    pub median_peak_multiple: Option<f64>,
    #[serde(default)]
    pub p75_peak_multiple: Option<f64>,
    #[serde(default)]
    pub median_mc_1h_multiple: Option<f64>,
    #[serde(default)]
    pub doubled_1h: i64,
    #[serde(default)]
    pub doubled_1h_pct: Option<f64>,
}

/// Per-(kind, tier) hit-rates returned with `stats: Some(true)`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SurgeStats {
    #[serde(default)]
    pub days: i64,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub rows: Vec<SurgeStatsRow>,
}

/// Echo of the filters a [`TokenSurgesResponse`] was served with.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SurgeFilters {
    #[serde(default)]
    pub kind: Option<SurgeKind>,
    #[serde(default)]
    pub tier: Option<SurgeTier>,
    #[serde(default)]
    pub mint: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub deployer_tier: Option<SurgeDeployerTier>,
    #[serde(default)]
    pub min_mc_usd: Option<f64>,
    #[serde(default)]
    pub max_mc_usd: Option<f64>,
    #[serde(default)]
    pub min_buys: Option<i64>,
    #[serde(default)]
    pub exclude_flags: Vec<String>,
    #[serde(default)]
    pub only_clean: bool,
}

/// `GET /tokens/surges` — token momentum fires (surges + revivals), newest
/// first, each with the burst tape, KOL involvement, early-buyer cohort,
/// deployer reputation, `risk_flags` and (≥ 65 min old) the +1 h `outcome`.
/// Pushed live as `token:surge` / `token:revival` on the `token:surges`
/// channel ([`TokenSurgeStreamEvent`]).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenSurgesResponse {
    #[serde(default)]
    pub events: Vec<TokenSurgeEvent>,
    pub pagination: TimeCursorPagination,
    #[serde(default)]
    pub filters: Option<SurgeFilters>,
    /// Only with `stats: Some(true)`.
    #[serde(default)]
    pub stats: Option<SurgeStats>,
    #[serde(default)]
    pub stream: Option<StreamPointer>,
    /// The live rule thresholds the response was produced under (`surge` /
    /// `revival` / `shared` / `risk_flags` / `tiers`), read straight from the
    /// engine — untyped, the block is documentation that cannot drift.
    #[serde(default)]
    pub definitions: Option<serde_json::Value>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

/// Server-side subscribe filters for the `token:surges` WebSocket channel —
/// serialise it as the `filters` object of the subscribe frame. `tiers`
/// applies to surges only (revivals always pass it); `exclude_flags` drops a
/// fire carrying ANY listed flag. Unset fields are omitted.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SurgeSubscribeFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<SurgeKind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<SurgeTier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launchpads: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_flags: Option<Vec<SurgeRiskFlag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployer_tier: Option<Vec<SurgeDeployerTier>>,
}

// ─── Token OHLC candles (/tokens/{mint}/candles) ────────────────────────────

/// Query params for [`Token::candles`](crate::api::token::Token::candles).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CandlesParams {
    /// Timeframe / bucket size (e.g. "1m"). Defaults server-side when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<String>,
    /// Max number of candles to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Lower bound of the time window (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Upper bound of the time window (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// A single OHLC candle. Each nullable field is `None` when the underlying data
/// was unavailable, and the ULTRA-only fields are absent for lower tiers.
#[derive(Debug, Clone, Deserialize)]
pub struct Candle {
    /// Bucket start time (ISO 8601).
    pub t: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    /// Total USD volume traded in the bucket.
    pub volume_usd: f64,
    /// Number of trades in the bucket.
    pub trades: i64,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    /// ULTRA — buy-side USD volume.
    #[serde(default)]
    pub buy_volume_usd: Option<f64>,
    /// ULTRA — sell-side USD volume.
    #[serde(default)]
    pub sell_volume_usd: Option<f64>,
    /// ULTRA — net USD flow (buy minus sell).
    #[serde(default)]
    pub net_volume_usd: Option<f64>,
    /// ULTRA — liquidity USD at bucket open.
    #[serde(default)]
    pub open_liquidity_usd: Option<f64>,
    /// ULTRA — liquidity USD at bucket close.
    #[serde(default)]
    pub close_liquidity_usd: Option<f64>,
    /// ULTRA — highest market cap USD in the bucket.
    #[serde(default)]
    pub high_mc_usd: Option<f64>,
    /// ULTRA — lowest market cap USD in the bucket.
    #[serde(default)]
    pub low_mc_usd: Option<f64>,
    /// ULTRA — buy trade count.
    #[serde(default)]
    pub buy_count: Option<i64>,
    /// ULTRA — sell trade count.
    #[serde(default)]
    pub sell_count: Option<i64>,
    /// ULTRA — MEV-attributed USD volume.
    #[serde(default)]
    pub volume_mev_usd: Option<f64>,
}

/// 1-minute OHLC candle series for a token (PRO/ULTRA).
#[derive(Debug, Clone, Deserialize)]
pub struct CandlesResponse {
    pub mint: String,
    /// Timeframe of each candle (e.g. "1m").
    pub timeframe: String,
    /// Lower bound of the returned window (ISO 8601).
    pub from: String,
    /// Upper bound of the returned window (ISO 8601).
    pub to: String,
    pub count: i64,
    /// Whether ULTRA net-flow fields are populated on the candles.
    pub net_flow_included: bool,
    pub candles: Vec<Candle>,
}

// ─── Token flow (/tokens/{mint}/flow) ───────────────────────────────────────

/// Query params for [`Token::token_flow`](crate::api::token::Token::token_flow).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TokenFlowParams {
    /// Lookback window — `"1h"` (default) or `"24h"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

/// Aggregated buy/sell flow for a token over a 1h or 24h window (PRO+).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenFlowResponse {
    pub mint: String,
    /// Window the figures cover — `"1h"` or `"24h"`.
    pub window: String,
    /// Window start time (ISO 8601).
    pub from: String,
    pub unique_wallets: f64,
    pub unique_buyers: f64,
    pub unique_sellers: f64,
    pub buy_count: f64,
    pub sell_count: f64,
    pub total_trades: f64,
    pub buy_sol: f64,
    pub sell_sol: f64,
    /// Net SOL flow — `buy_sol` minus `sell_sol`.
    pub net_sol: f64,
    pub trades_per_wallet: f64,
    /// v0.23.4 — trade-coverage disclosure; when `in_scope` is `false` the
    /// zero counts mean "not covered", not "no activity".
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
}

// ─── Token trade tape (/tokens/{mint}/trades, v0.22) ────────────────────────

/// Query params for [`Token::trades`](crate::api::token::Token::trades).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TokenTradesParams {
    /// 1–500; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// From `next_cursor` of a previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<WalletTradeAction>,
    /// Filter to a single wallet address (base58).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    /// Unix epoch seconds; default FULL history (capture starts 2026-04-12).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    /// Unix epoch seconds; default now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
}

/// One raw trade on a token's tape.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenTrade {
    pub tx_signature: String,
    pub wallet_address: String,
    /// `"buy"` | `"sell"`.
    pub action: String,
    pub sol_amount: f64,
    pub token_amount: f64,
    /// THIS TRADE's executed price: `sol_amount / token_amount`, so it reconciles
    /// exactly with the amounts on the same row and with the PnL endpoints.
    /// `sol_amount` is the wallet's net SOL movement, so this is the trader's
    /// all-in effective rate — swap fee and any account rent included, not the
    /// pool mid. `None` for dust and zero-SOL legs.
    ///
    /// Changed 2026-08-16: previously the canonical pool price, which disagreed
    /// with the row's own amounts by a 7.9% median. That value moved to
    /// [`TokenTrade::market_price_sol`].
    #[serde(default)]
    pub price_sol: Option<f64>,
    /// [`TokenTrade::price_sol`] in USD.
    #[serde(default)]
    pub price_usd: Option<f64>,
    /// Canonical pool price sampled near this trade's slot — one value per token
    /// per update, so every trade in the same slot shares it. Use for a per-token
    /// series; use `price_sol` for cost basis and PnL.
    #[serde(default)]
    pub market_price_sol: Option<f64>,
    /// [`TokenTrade::market_price_sol`] in USD.
    #[serde(default)]
    pub market_price_usd: Option<f64>,
    #[serde(default)]
    pub early_buyer_rank: Option<i64>,
    #[serde(default)]
    pub slot: Option<i64>,
    /// Unix epoch seconds.
    pub block_time: i64,
    /// ISO 8601 — `block_time` rendered as a timestamp.
    pub traded_at: String,
}

/// Echo of the filters a token-trades page was produced with.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenTradesFilters {
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub wallet: Option<String>,
    pub since: i64,
    pub until: i64,
}

/// Coverage honesty block: where the tape starts and what pipeline feeds it.
///
/// v0.23.4 — also returned (as an optional `coverage` field) on bundle, risk,
/// flow, cap-table, buyer-quality and the wallet
/// stats/pnl/positions/holdings/trades endpoints, with the new `in_scope`
/// probe.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenTradesCoverage {
    /// Unix epoch seconds — capture starts 2026-04-12.
    pub history_start: i64,
    /// e.g. `"pump.fun pipeline"` — trades outside that pipeline aren't on the tape.
    pub scope: String,
    /// v0.23.4 — `Some(true)` = persisted trades exist for this mint/wallet ·
    /// `Some(false)` = the subject sits outside the launchpad write-gate (read
    /// zeros as "not covered", NOT "no activity") · `None` = probe unavailable
    /// or an older cached response.
    #[serde(default)]
    pub in_scope: Option<bool>,
    /// Present when `in_scope` is `false`/`null` (and always on `/risk`) —
    /// human-readable explanation of what the coverage gap means.
    #[serde(default)]
    pub note: Option<String>,
}

/// Preferred name for the coverage honesty block — it is no longer
/// trades-specific (see [`TokenTradesCoverage`]).
pub type TradeCoverage = TokenTradesCoverage;

/// Response of [`Token::trades`](crate::api::token::Token::trades).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenTradesResponse {
    pub mint: String,
    pub trades: Vec<TokenTrade>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    pub filters: TokenTradesFilters,
    pub coverage: TokenTradesCoverage,
}

// ─── Token depth / price impact (/tokens/{mint}/depth, v0.23) ───────────────

/// Query params for [`Token::depth`](crate::api::token::Token::depth).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DepthParams {
    /// CSV of SOL buy sizes — max 8 values, each `> 0` and `<= 10000`,
    /// e.g. `"0.5,1,5,10"`. Server default when unset: `0.5,1,5,10`.
    /// Prefer [`DepthParams::from_sizes`] to build this from numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<String>,
}

impl DepthParams {
    /// Build params from numeric SOL buy sizes — joined into the CSV the API
    /// expects (`?sizes=0.5,1,5,10`).
    pub fn from_sizes(sizes: &[f64]) -> Self {
        Self {
            sizes: Some(
                sizes
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        }
    }
}

/// Slippage quote for one requested buy size against one pool.
#[derive(Debug, Clone, Deserialize)]
pub struct DepthQuote {
    /// The requested buy size, in SOL.
    pub size_sol: f64,
    /// Tokens received for that buy (UI units).
    pub tokens_out: f64,
    /// Average execution price paid, SOL per token.
    pub avg_price_sol: f64,
    /// Price impact vs spot, percent (rounded to 2 decimals server-side).
    pub price_impact_pct: f64,
}

/// SOL required to move a pool's spot price up by 1% / 5% / 10%
/// (closed-form constant-product, fee-adjusted).
#[derive(Debug, Clone, Deserialize)]
pub struct DepthToMovePrice {
    #[serde(rename = "1pct")]
    pub pct_1: f64,
    #[serde(rename = "5pct")]
    pub pct_5: f64,
    #[serde(rename = "10pct")]
    pub pct_10: f64,
}

/// Per-pool depth breakdown: spot price, slippage quotes per buy size, and
/// how much SOL it takes to move the price 1/5/10%.
#[derive(Debug, Clone, Deserialize)]
pub struct DepthPool {
    pub pool_address: String,
    pub dex: String,
    pub quote_mint: String,
    /// Pool model the depth was computed under — `"constant_product"` or `"curve"`.
    pub pool_model: String,
    /// Pool liquidity in USD (rounded). `None` when unknown.
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    /// `true` when the pool saw a swap within the last hour.
    pub is_active: bool,
    /// Always `true` on entries in `pools` (unsupported pools are split into
    /// `unsupported_pools` instead).
    pub depth_available: bool,
    /// Same as `pool_model` — the model the math used.
    pub model: String,
    /// Swap fee applied, percent (e.g. `0.25`).
    pub fee_pct: f64,
    /// `"stream"` (DB reserves from the firehose) or `"live_rpc"` (curve
    /// virtual reserves read live on-chain).
    pub source: String,
    /// Age of the reserves snapshot in milliseconds. `0` for `live_rpc`.
    pub reserves_age_ms: i64,
    /// Current spot price, SOL per token.
    pub spot_price_sol: f64,
    /// One slippage quote per requested size, in `sizes_sol` order.
    pub quotes: Vec<DepthQuote>,
    pub to_move_price: DepthToMovePrice,
}

/// A pool depth could not be computed for, with the machine-readable `reason`
/// (e.g. `"concentrated_liquidity_depth_not_supported"`, `"pool_model_unknown"`,
/// `"curve_graduated_use_amm_pool"`, `"reserves_unavailable"`).
#[derive(Debug, Clone, Deserialize)]
pub struct DepthUnsupportedPool {
    pub pool_address: String,
    pub dex: String,
    pub quote_mint: String,
    /// `None` when the pool hasn't been classified yet (`reason = "pool_model_unknown"`).
    #[serde(default)]
    pub pool_model: Option<String>,
    /// Pool liquidity in USD (rounded). `None` when unknown.
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    /// `true` when the pool saw a swap within the last hour.
    pub is_active: bool,
    /// Why depth isn't available for this pool.
    pub reason: String,
}

/// Response of [`Token::depth`](crate::api::token::Token::depth).
///
/// When the mint has no tracked pools at all, `found` is `false`, `pools` and
/// `unsupported_pools` are empty, and `sol_usd` / `primary_pool` / `note` are
/// absent (`None`).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenDepthResponse {
    pub mint: String,
    /// `true` when at least one pool has computable depth.
    pub found: bool,
    /// SOL/USD price used to convert stable-quoted pools. `None` when
    /// unavailable (or when the mint has no pools).
    #[serde(default)]
    pub sol_usd: Option<f64>,
    /// The SOL buy sizes the quotes were computed for (deduped, ascending).
    pub sizes_sol: Vec<f64>,
    /// Address of the deepest pool with depth available. `None` when no pool
    /// qualified (or when the mint has no pools).
    #[serde(default)]
    pub primary_pool: Option<String>,
    pub pools: Vec<DepthPool>,
    pub unsupported_pools: Vec<DepthUnsupportedPool>,
    /// Methodology caveat string. Absent when the mint has no pools.
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Signal Scorecard (/signals) ────────────────────────────────────────────

/// Query params for [`Signals::performance`](crate::api::signals::Signals::performance).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SignalPerformanceParams {
    /// When `Some(true)`, append the per-day snapshot series (`history`) for
    /// drift inspection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<bool>,
}

/// A single per-bucket reliability row within a signal's scorecard.
#[derive(Debug, Clone, Deserialize)]
pub struct SignalBucket {
    /// Bucket label (e.g. a value or threshold band for the signal).
    pub bucket: String,
    /// Out-of-sample hit rate for this bucket (0–1).
    #[serde(default)]
    pub hit_rate: Option<f64>,
    /// Base rate the bucket is compared against (0–1).
    #[serde(default)]
    pub base_rate: Option<f64>,
    /// Lift over the base rate (`hit_rate / base_rate`).
    #[serde(default)]
    pub lift: Option<f64>,
    /// Number of samples backing this bucket.
    pub sample_n: i64,
    /// Length of the outcome window in days. Present on history rows.
    #[serde(default)]
    pub window_days: Option<i64>,
    /// Lower bound of the out-of-sample test window (ISO 8601). Present on history rows.
    #[serde(default)]
    pub test_from: Option<String>,
    /// Upper bound of the out-of-sample test window (ISO 8601). Present on history rows.
    #[serde(default)]
    pub test_to: Option<String>,
}

/// One daily snapshot of a signal's per-bucket scorecard (drift series).
/// Only present when [`SignalPerformanceParams::history`] is `Some(true)`.
#[derive(Debug, Clone, Deserialize)]
pub struct SignalHistoryEntry {
    /// When this snapshot was computed (ISO 8601).
    pub as_of: String,
    pub buckets: Vec<SignalBucket>,
}

/// Out-of-sample, machine-readable reliability for a named signal (any tier).
/// Each nullable field is `None` when the underlying data was unavailable.
#[derive(Debug, Clone, Deserialize)]
pub struct SignalPerformance {
    /// Signal name (e.g. "coordination_count").
    pub signal: String,
    /// Metric family (e.g. "forward_return"). Absent before any data is computed.
    #[serde(default)]
    pub metric_type: Option<String>,
    /// Outcome label being measured (e.g. "peak_2x_6h"). Absent before any data is computed.
    #[serde(default)]
    pub outcome: Option<String>,
    /// Human-readable description of the backtest methodology.
    #[serde(default)]
    pub methodology: Option<String>,
    /// When the latest snapshot was computed (ISO 8601). Absent before any data is computed.
    #[serde(default)]
    pub as_of: Option<String>,
    /// Length of the outcome window in days.
    #[serde(default)]
    pub window_days: Option<i64>,
    /// Overall base rate for the signal (0–1).
    #[serde(default)]
    pub base_rate: Option<f64>,
    /// Lower bound of the out-of-sample test window (ISO 8601).
    #[serde(default)]
    pub test_from: Option<String>,
    /// Upper bound of the out-of-sample test window (ISO 8601).
    #[serde(default)]
    pub test_to: Option<String>,
    /// Per-bucket reliability rows for the latest snapshot.
    pub buckets: Vec<SignalBucket>,
    /// Per-day snapshot series — only present when `history=true` was requested.
    #[serde(default)]
    pub history: Option<Vec<SignalHistoryEntry>>,
    /// Note set when no performance data has been computed yet.
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

/// A catalog entry describing one available signal.
#[derive(Debug, Clone, Deserialize)]
pub struct SignalCatalogEntry {
    /// Signal name — pass to [`Signals::performance`](crate::api::signals::Signals::performance).
    pub name: String,
    /// Human-readable description of the backtest methodology.
    pub methodology: String,
    /// Fully-qualified URL to fetch this signal's live efficacy.
    pub performance_endpoint: String,
}

/// Discovery index for the Signal Scorecard (any tier).
#[derive(Debug, Clone, Deserialize)]
pub struct SignalsCatalog {
    /// Catalog name.
    pub name: String,
    /// Catalog description.
    pub description: String,
    /// The available signals and how to fetch each one's efficacy.
    pub signals: Vec<SignalCatalogEntry>,
    /// Link to human-readable docs.
    pub docs: String,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Wallet Tracker ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct WatchlistAddParams {
    pub wallet_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WatchlistUpdateParams {
    /// New label for the wallet, or `None` to clear it.
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletEntry {
    pub wallet_address: String,
    pub label: Option<String>,
    pub added_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatchlistAddResponse {
    pub wallet_address: String,
    pub label: Option<String>,
    pub added_at: String,
    pub remaining: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatchlistResponse {
    pub wallets: Vec<WalletEntry>,
    pub count: u32,
    pub limit: u32,
    pub remaining: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerEvent {
    pub id: String,
    pub wallet_address: String,
    pub label: Option<String>,
    pub event_type: WalletTrackerEventType,
    pub action: WalletTrackerAction,
    pub block_time: i64,
    pub block_time_iso: String,
    pub token_mint: Option<String>,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
    pub sol_amount: f64,
    pub token_amount: Option<f64>,
    pub price_per_token_sol: Option<f64>,
    pub counterparty: Option<String>,
    pub tx_signature: Option<String>,
    pub program: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WalletTrackerTradesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<WalletTrackerAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<WalletTrackerEventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerTradesResponse {
    pub events: Vec<WalletTrackerEvent>,
    pub count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerWalletStats {
    pub wallet_address: String,
    pub label: Option<String>,
    pub swap_count: u32,
    pub buys: u32,
    pub sells: u32,
    pub sol_bought: f64,
    pub sol_sold: f64,
    pub last_event_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WalletTrackerSummaryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<WalletTrackerSummaryPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerSummaryResponse {
    pub wallets: Vec<WalletTrackerWalletStats>,
    pub period: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerDeleteResponse {
    pub success: bool,
}

// ─── Sniper: deshred pre-confirm pump.fun deploy feed (PRO + ULTRA) ─────────

/// Slot-window launch-snipe rollup (v0.22): buys landing in slots
/// `[deploy-1 .. deploy+3]`. Appears on sniper deploys (`footprint`) and in
/// token risk inputs (`sniper_footprint`).
///
/// `data_available = false` means the mint has no rows in the trade pipeline
/// (write-gate) — "not observable", NOT "0 snipes". `supply_pct` is
/// pump.fun-only (fixed 1B supply denominator) and `None` otherwise.
#[derive(Debug, Clone, Deserialize)]
pub struct SniperFootprint {
    pub buys: i64,
    pub buyers: i64,
    pub sol: f64,
    #[serde(default)]
    pub supply_pct: Option<f64>,
    /// How many of the window's buys came from known sniper wallets.
    pub sniper_wallet_buys: i64,
    pub data_available: bool,
    /// ISO 8601 — when the rollup was computed.
    pub as_of: String,
}

/// A pump.fun deploy detected via shred-level ("deshred") reconstruction,
/// ~500ms before the chain confirms it. Detection is pre-execution, so
/// `confirmed_on_chain` is `None` until reconciled.
#[derive(Debug, Clone, Deserialize)]
pub struct SniperDeploy {
    pub mint: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub deployer_wallet: String,
    pub signature: String,
    pub slot: i64,
    pub detected_at: String,
    pub detection_region: String,
    pub deployer_tier: Option<String>,
    pub deployer_bond_rate: Option<f64>,
    pub deployer_total_bonded: Option<i64>,
    pub deployer_recent: Option<String>,
    /// Fraction of the deployer's labeled tokens that ran (peak >=60min after deploy) vs dumped.
    #[serde(default)]
    pub deployer_runner_rate: Option<f64>,
    /// Confidence denominator; gate on >=3.
    #[serde(default)]
    pub deployer_labeled_tokens: Option<i64>,
    pub confirmed_on_chain: Option<bool>,
    pub confirmed_at: Option<String>,
    /// v0.22 — slot-window snipe rollup. `None` for deploys younger than the
    /// rollup's ~10-min settle window or outside the trade-pipeline write-gate
    /// (absent, not zero).
    #[serde(default)]
    pub footprint: Option<SniperFootprint>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SniperRecentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployer_tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bond_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// ULTRA: narrow to your custom deployer watchlist (any tier).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchlist: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SniperRecentResponse {
    pub deploys: Vec<SniperDeploy>,
    pub count: u32,
    pub data_age_seconds: Option<i64>,
    #[serde(default)]
    pub watchlist_empty: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SniperByDeployerResponse {
    pub deployer: String,
    pub deploys: Vec<SniperDeploy>,
    pub count: u32,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SniperByDeployerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SniperWatchlistEntry {
    pub deployer_wallet: String,
    pub label: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SniperWatchlistResponse {
    pub deployers: Vec<SniperWatchlistEntry>,
    pub count: u32,
    pub limit: u32,
    pub remaining: u32,
}

/// Add one (`wallet`) or many (`wallets`, max 50) deployers, with an optional label.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SniperWatchlistAddParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SniperWatchlistAddResponse {
    pub added: u32,
    #[serde(default)]
    pub deployers: Vec<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SniperWatchlistRemoveResponse {
    pub removed: String,
}

// ─── Universal wallet (PRO+) ────────────────────────────────────────────────
// New 2026-05-20. Works on any Solana wallet — not just curated KOLs. Backed
// by FIFO cost-basis math over the last 90 days of token_trades. Cached in
// `wallet_analyses` with dynamic TTL (5min/1h/24h). Cache hits don't count
// against your daily quota.

#[derive(Debug, Clone, Deserialize)]
pub struct WalletStats {
    pub first_seen: String,
    pub last_seen: String,
    pub total_trades: u64,
    pub buys: u64,
    pub sells: u64,
    pub bought_sol: f64,
    pub sold_sol: f64,
    pub unique_tokens: u64,
    /// Lookback window in days — currently 90.
    pub window_days: u32,
}

/// Dump-cluster cohort counts for a wallet (rolling 42-day window, recomputed
/// daily — up to ~48h stale). `as_of` is the computation timestamp.
#[derive(Debug, Clone, Deserialize)]
pub struct WalletDumpCluster {
    pub dump_cohorts: i64,
    pub runner_cohorts: i64,
    pub total_cohorts: i64,
    pub as_of: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletFlags {
    pub is_kol: bool,
    pub kol_name: Option<String>,
    /// True if the wallet appears in `mv_alpha_wallets`.
    pub is_alpha_tracked: bool,
    /// Text enum `"none"` | `"low"` | `"medium"` | `"high"`; `None` when the
    /// wallet isn't alpha-tracked. Sourced from the alpha classifier.
    ///
    /// v0.22 type FIX: this was previously (mis)typed as `Option<f64>` — the
    /// wire value is a string enum, and a server bug made it always `null`
    /// before 2026-07-10; it now carries real values.
    pub bot_confidence: Option<String>,
    pub alpha_win_rate: Option<f64>,
    pub alpha_net_pnl_sol: Option<f64>,
    pub alpha_tokens_traded: Option<u64>,
    pub is_deployer: bool,
    pub deployer_tokens_deployed: Option<u64>,
    pub deployer_bonding_rate: Option<f64>,
    // v0.22 reputation flags. Scope caveat: all three derive from the pump.fun
    // trade pipeline — `false` means "not observed", NOT "verified clean".
    /// ≥5 tokens early-bought & ≥80% flipped <5min. Behavior-updated (~12min
    /// cron), so it can clear if the wallet reforms. Pipeline-scoped.
    #[serde(default)]
    pub is_sniper: bool,
    /// Bought >1 token in the same block, EVER — a lifetime flag.
    #[serde(default)]
    pub is_bundler: bool,
    /// ≥5 dump cohorts + 0 runner cohorts in a rolling 42-day window
    /// (recomputed daily, up to ~48h stale).
    #[serde(default)]
    pub is_dumper: bool,
    /// Raw dump-cluster cohort counts behind `is_dumper`; `None` when the
    /// wallet has no dump-cluster record.
    #[serde(default)]
    pub dump_cluster: Option<WalletDumpCluster>,
    /// v0.23.4 — semantics reminder carried in-band: `is_sniper` /
    /// `is_bundler` / `is_dumper` are launchpad-pipeline scoped, so `false`
    /// means "not observed in covered trades", not "verified clean".
    #[serde(default)]
    pub coverage_note: Option<String>,
}

// v1.8.1 enrichments — additive, all Option<...> so old SDK builds keep
// deserializing responses that omit them.

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTopToken {
    pub token_mint: String,
    pub token_symbol: Option<String>,
    pub buys: u64,
    pub sells: u64,
    pub sol_in: f64,
    pub sol_out: f64,
    pub realized_pnl_sol: f64,
    pub current_mc_usd: Option<f64>,
    pub peak_mc_usd: Option<f64>,
    pub last_traded_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTradingStyle {
    pub total_trades: u64,
    pub avg_trade_size_sol: f64,
    /// 0–1: fraction of trades placed with `early_buyer_rank ≤ 10`.
    pub sniper_rate: f64,
    pub early_entries: u64,
    /// 0–1: fraction of tokens with both buys and sells.
    pub round_trip_rate: f64,
    pub tokens_with_round_trips: u64,
    pub median_hold_minutes: Option<f64>,
    /// "buy" | "sell" | "balanced".
    pub dominant_action: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletDeployerTierEntry {
    /// "elite" | "good" | "rising" | "moderate" | "cold" | "unranked".
    pub tier: String,
    pub count: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletDeployerBreakdown {
    pub total_tokens: u64,
    pub tracked_deployers: u64,
    pub by_tier: Vec<WalletDeployerTierEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletRecentTrade {
    pub token_mint: String,
    pub token_symbol: Option<String>,
    /// "buy" | "sell".
    pub action: String,
    pub sol_amount: f64,
    pub block_time: i64,
    pub traded_at: String,
    pub tx_signature: String,
}

// ─── Wallet derived stats (v1.9) ────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct WalletStandoutTrade {
    pub token_mint: String,
    #[serde(default)]
    pub token_symbol: Option<String>,
    pub pnl_sol: f64,
    pub sol_in: f64,
    pub sol_out: f64,
    pub roi_pct: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletBiggestMiss {
    pub token_mint: String,
    #[serde(default)]
    pub token_symbol: Option<String>,
    pub actual_sol_out: f64,
    pub potential_sol_at_ath: f64,
    pub missed_sol: f64,
    pub ath_mc_usd: f64,
    #[serde(default)]
    pub sold_at_mc_usd: Option<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WalletVerdictTone {
    Green,
    Red,
    Amber,
    Muted,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletVerdict {
    pub label: String,
    pub description: String,
    pub tone: WalletVerdictTone,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletDerivedStats {
    #[serde(default)]
    pub win_rate: Option<f64>,
    #[serde(default)]
    pub roi_pct: Option<f64>,
    pub total_realized_pnl_sol: f64,
    #[serde(default)]
    pub best_trade: Option<WalletStandoutTrade>,
    #[serde(default)]
    pub worst_trade: Option<WalletStandoutTrade>,
    #[serde(default)]
    pub biggest_miss: Option<WalletBiggestMiss>,
    #[serde(default)]
    pub verdict: Option<WalletVerdict>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletStatsResponse {
    pub address: String,
    /// None when the wallet has no trades in the 90-day window but does
    /// appear in one of the flag tables.
    pub stats: Option<WalletStats>,
    pub flags: WalletFlags,
    /// Top traded tokens with realized PnL (v1.8.1+).
    #[serde(default)]
    pub top_tokens: Vec<WalletTopToken>,
    /// Trading-style signals (v1.8.1+).
    #[serde(default)]
    pub trading_style: Option<WalletTradingStyle>,
    /// Pump.fun deployer-tier distribution (v1.8.1+).
    #[serde(default)]
    pub deployer_breakdown: Option<WalletDeployerBreakdown>,
    /// Last 10 raw trades with symbols joined (v1.8.1+).
    #[serde(default)]
    pub recent_trades: Vec<WalletRecentTrade>,
    /// Derived analytics: win rate, ROI, best/worst trade, biggest miss, verdict (v1.9+).
    #[serde(default)]
    pub derived: Option<WalletDerivedStats>,
    /// v0.23.4 — trade-coverage disclosure; when `in_scope` is `false` an
    /// empty `stats` block means "outside the write-gate", not "never traded".
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletBestWorstTrade {
    pub token_mint: String,
    pub realized_sol: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletPnlSummary {
    pub realized_sol: f64,
    pub unrealized_sol: f64,
    pub total_pnl_sol: f64,
    pub total_bought_sol: f64,
    pub total_sold_sol: f64,
    /// Closed-position win count (not per-trade).
    pub wins: u64,
    pub losses: u64,
    pub win_rate: Option<f64>,
    /// Gross wins / gross losses. None when no losses (undefined math).
    pub profit_factor: Option<f64>,
    pub avg_hold_minutes: Option<u64>,
    pub median_hold_minutes: Option<u64>,
    /// Running peak-to-trough drawdown on the realized SOL curve.
    pub max_drawdown_sol: f64,
    pub open_positions_count: u64,
    pub closed_positions_count: u64,
    pub total_tokens_traded: u64,
    pub best_realized: Option<WalletBestWorstTrade>,
    pub worst_realized: Option<WalletBestWorstTrade>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletPnlCurvePoint {
    /// YYYY-MM-DD (UTC).
    pub date: String,
    pub day_pnl: f64,
    pub cumulative_pnl: f64,
    pub trades: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletClosedPosition {
    pub token_mint: String,
    pub buy_count: u64,
    pub sell_count: u64,
    pub bought_sol: f64,
    pub sold_sol: f64,
    pub pnl_sol: f64,
    /// `realized_sol / total_bought_sol × 100`.
    pub roi_pct: Option<f64>,
    /// First buy → last sell, in minutes.
    pub hold_minutes: Option<u64>,
    pub result: String,
    pub first_trade: Option<String>,
    pub last_trade: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletOpenPosition {
    pub token_mint: String,
    pub token_amount: f64,
    pub cost_basis_sol: f64,
    pub avg_entry_price_sol: f64,
    /// Live from mc-tracker. None if the mint has no current price.
    pub current_price_sol: Option<f64>,
    pub current_value_sol: Option<f64>,
    pub unrealized_sol: Option<f64>,
    pub unrealized_pct: Option<f64>,
    pub first_buy_at: Option<String>,
    pub buys_in_position: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletPnlNotes {
    /// Cost basis is observable only from this timestamp onwards.
    pub cost_basis_observable_from: String,
    /// Present when the 50k-trade hard cap was hit.
    #[serde(default)]
    pub truncated_trades: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletPnlResponse {
    pub address: String,
    pub window_days: u32,
    pub summary: WalletPnlSummary,
    /// Sparse daily UTC buckets — only days with at least one realized event.
    pub pnl_curve: Vec<WalletPnlCurvePoint>,
    /// Sorted by `pnl_sol` DESC — best winners first.
    pub closed_positions: Vec<WalletClosedPosition>,
    pub open_positions: Vec<WalletOpenPosition>,
    pub notes: WalletPnlNotes,
    #[serde(default)]
    pub cache_hit: Option<bool>,
    /// Only present on cache hits.
    #[serde(default)]
    pub computed_at: Option<String>,
    /// Only present on cache misses.
    #[serde(default)]
    pub ttl_seconds: Option<u64>,
    /// v0.23.4 — trade-coverage disclosure; `in_scope: Some(false)` means the
    /// PnL is built from zero covered trades, not that the wallet never traded.
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletPositionsResponse {
    pub address: String,
    pub positions: Vec<WalletOpenPosition>,
    #[serde(default)]
    pub cache_hit: Option<bool>,
    #[serde(default)]
    pub computed_at: Option<String>,
    #[serde(default)]
    pub ttl_seconds: Option<u64>,
    /// v0.23.4 — trade-coverage disclosure (`None` on older cached responses).
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletTradeAction {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WalletTradesParams {
    /// 1–500; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// From `next_cursor` of a previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<WalletTradeAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_mint: Option<String>,
    /// Unix epoch seconds; default now-90d.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    /// Unix epoch seconds; default now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrade {
    pub tx_signature: String,
    pub token_mint: String,
    pub action: String,
    pub sol_amount: f64,
    pub token_amount: f64,
    /// This trade's executed price — `sol_amount / token_amount`. Added
    /// 2026-08-16; this route returned amounts and no price before. Same
    /// definition as [`TokenTrade::price_sol`].
    #[serde(default)]
    pub price_sol: Option<f64>,
    /// [`WalletTrade::price_sol`] in USD.
    #[serde(default)]
    pub price_usd: Option<f64>,
    /// Canonical pool price near this trade's slot — NOT this trade's price.
    #[serde(default)]
    pub market_price_sol: Option<f64>,
    /// [`WalletTrade::market_price_sol`] in USD.
    #[serde(default)]
    pub market_price_usd: Option<f64>,
    pub block_time: i64,
    pub traded_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTradesFilters {
    pub action: Option<String>,
    pub token_mint: Option<String>,
    pub since: i64,
    pub until: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTradesResponse {
    pub address: String,
    pub trades: Vec<WalletTrade>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    pub filters: WalletTradesFilters,
    /// v0.23.4 — trade-coverage disclosure; an empty tape with `in_scope:
    /// Some(false)` means "outside the write-gate", not "never traded".
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
}

// ─── Wallet batch classify (/wallet/batch/classify, v0.22) ──────────────────

/// Request body for [`Wallet::batch_classify`](crate::api::wallet::Wallet::batch_classify).
#[derive(Debug, Clone, Serialize)]
pub struct WalletBatchRequest {
    /// 1–100 base58 wallet addresses. Duplicates are removed server-side.
    pub wallets: Vec<String>,
}

/// Reputation flags for one wallet in a batch-classify response. Field values
/// match the `flags` block of [`WalletStatsResponse`] exactly.
///
/// Scope caveat: `is_sniper` / `is_bundler` / `is_dumper` derive from the
/// pump.fun trade pipeline — `false` means "not observed", NOT "verified
/// clean". `is_bundler` is a lifetime flag; `is_dumper` uses a rolling 42-day
/// window (recomputed daily, up to ~48h stale).
#[derive(Debug, Clone, Deserialize)]
pub struct WalletClassification {
    pub address: String,
    pub is_sniper: bool,
    pub is_bundler: bool,
    pub is_dumper: bool,
    pub is_kol: bool,
    #[serde(default)]
    pub kol_name: Option<String>,
    /// Text enum `"none"` | `"low"` | `"medium"` | `"high"`; `None` when the
    /// wallet isn't alpha-tracked.
    #[serde(default)]
    pub bot_confidence: Option<String>,
    /// Raw dump-cluster cohort counts behind `is_dumper`; `None` when the
    /// wallet has no dump-cluster record.
    #[serde(default)]
    pub dump_cluster: Option<WalletDumpCluster>,
}

/// Response of [`Wallet::batch_classify`](crate::api::wallet::Wallet::batch_classify).
#[derive(Debug, Clone, Deserialize)]
pub struct WalletBatchClassifyResponse {
    /// One entry per unique input wallet, in de-duplicated input order.
    pub wallets: Vec<WalletClassification>,
    /// Number of unique wallets classified.
    pub count: u32,
    /// ISO 8601 — when the batch was computed.
    pub as_of: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WalletHoldingsParams {
    /// 1–500; default 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Minimum USD value per holding; default 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    pub mint: String,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub amount: f64,
    /// Raw base-unit amount as a decimal string (no precision loss).
    pub amount_raw: String,
    pub decimals: u32,
    /// "spl" | "token2022".
    pub token_program: String,
    pub price_usd: Option<f64>,
    pub value_usd: Option<f64>,
    pub market_cap_usd: Option<f64>,
    pub is_bonded: Option<bool>,
    pub trade_derived_amount: Option<f64>,
    /// On-chain amount minus trade-derived net position — exposes non-swap
    /// flows (airdrops, insider funding, wallet-hopping).
    pub transfer_delta: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletHoldingsSummary {
    pub token_accounts: u32,
    pub non_zero: u32,
    pub returned: u32,
    pub priced: u32,
    pub total_value_usd: f64,
    pub truncated: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletHoldingsResponse {
    pub address: String,
    pub sol_balance: f64,
    pub holdings: Vec<Holding>,
    pub summary: WalletHoldingsSummary,
    pub verified_at: String,
    pub trade_window_days: u32,
    #[serde(default)]
    pub cache_hit: Option<bool>,
    #[serde(default)]
    pub ttl_seconds: Option<u64>,
    /// v0.23.4 — trade-coverage disclosure for the trade-derived enrichments
    /// (`transfer_delta`, cost-basis fields); the holdings themselves are live
    /// on-chain reads and unaffected.
    #[serde(default)]
    pub coverage: Option<TokenTradesCoverage>,
}

// ─── Tools ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct ToolsSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tool {
    pub name: String,
    pub slug: String,
    pub tagline: String,
    pub website_url: String,
    #[serde(default)]
    pub logo_url: Option<String>,
    pub categories: Vec<String>,
    #[serde(default)]
    pub pricing_model: Option<String>,
    #[serde(default)]
    pub average_rating: Option<f64>,
    #[serde(default)]
    pub review_count: Option<u32>,
    #[serde(default)]
    pub health_score: Option<f64>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolsSearchResponse {
    pub tools: Vec<Tool>,
    pub count: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Streaming ──────────────────────────────────────────────────────────────

/// Your WebSocket streaming token, as returned by
/// [`Stream::get_token`](crate::api::stream::Stream::get_token) and
/// [`Stream::rotate_token`](crate::api::stream::Stream::rotate_token).
///
/// **Stream tokens do not expire** (since 2026-08-27). `get_token` returns the
/// same value on every call; it only stops working when your subscription
/// lapses or you explicitly rotate it (`{"rotate": true}` — the previous value
/// then stays valid for 60 s so live sockets can reconnect). A WebSocket close
/// code `4001` means "mint a new token", never "the timer ran out".
#[derive(Debug, Clone, Deserialize)]
pub struct StreamToken {
    pub token: String,
    /// Always `None` since 2026-08-27 — stream tokens do not expire; kept for
    /// wire compatibility. Do not schedule refreshes on it.
    #[serde(default)]
    pub expires_at: Option<String>,
    /// Always `None` since 2026-08-27 — stream tokens do not expire; kept for
    /// wire compatibility.
    #[serde(default)]
    pub next_refresh_at: Option<String>,
    /// `Some(true)` when this call replaced an existing token (a
    /// `{"rotate": true}` request — see
    /// [`Stream::rotate_token`](crate::api::stream::Stream::rotate_token));
    /// `Some(false)` when the existing token was returned unchanged or was
    /// minted for the first time. `None` from servers older than 2026-08-27.
    #[serde(default)]
    pub rotated: Option<bool>,
    /// Human-readable lifetime statement from the server: the token does not
    /// expire, is identical on every call, and dies only on subscription lapse
    /// or an explicit rotation. `None` from servers older than 2026-08-27.
    #[serde(default)]
    pub lifetime: Option<String>,
    pub ws_url: String,
    /// Only present for ULTRA-tier subscribers.
    #[serde(default)]
    pub dex_ws_url: Option<String>,
    pub usage: String,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

/// A single live WebSocket session for your account, as returned by
/// [`Stream::sessions`](crate::api::stream::Stream::sessions).
#[derive(Debug, Clone, Deserialize)]
pub struct StreamSession {
    /// Numeric session id (serialized as a string). Pass to
    /// [`Stream::kill_session`](crate::api::stream::Stream::kill_session).
    pub id: String,
    /// Originating service: `"ws-streaming"` or `"dex-stream"`.
    pub service: String,
    pub tier: String,
    pub channels: Vec<String>,
    pub connected_at: String,
    #[serde(default)]
    pub remote_ip: Option<String>,
    /// Messages pushed to this socket so far.
    pub messages_sent: u64,
}

/// Response of [`Stream::sessions`](crate::api::stream::Stream::sessions).
#[derive(Debug, Clone, Deserialize)]
pub struct StreamSessionsResponse {
    pub sessions: Vec<StreamSession>,
    pub count: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

/// Response of [`Stream::kill_session`](crate::api::stream::Stream::kill_session).
#[derive(Debug, Clone, Deserialize)]
pub struct StreamSessionEvicted {
    pub evicted: bool,
    pub id: String,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Webhooks ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct WebhookCreateParams {
    pub url: String,
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WebhookUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Webhook {
    pub id: i64,
    pub url: String,
    pub events: Vec<String>,
    pub filters: Option<HashMap<String, serde_json::Value>>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookListResponse {
    pub webhooks: Vec<Webhook>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookDeleteResponse {
    pub success: bool,
}

// ─── /me — v0.8 (server-side v1.7, 2026-05-12) ───────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ApiTier {
    Basic,
    Trader,
    Pro,
    Ultra,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeQuotaWindow {
    pub limit: u64,
    pub used: u64,
    pub remaining: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeDailyQuota {
    pub limit: u64,
    pub used: u64,
    pub remaining: u64,
    pub resets_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeBurstQuota {
    pub limit: u64,
    pub used: u64,
    pub remaining: u64,
    pub window_seconds: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeQuota {
    pub daily: MeDailyQuota,
    pub burst: MeBurstQuota,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeSubscription {
    pub status: String,
    pub billing_cycle: String,
    pub current_period_end: Option<String>,
    pub started_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeFeatureSlot {
    pub limit: u32,
    #[serde(default)]
    pub used: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeFeatureUsage {
    pub webhooks: MeFeatureSlot,
    pub ws_connections: MeFeatureSlot,
    pub dex_connections: MeFeatureSlot,
    pub copytrade_wallets: MeFeatureSlot,
    pub copytrade_rules: MeFeatureSlot,
    pub coordination_rules: MeFeatureSlot,
    pub first_touch_subscriptions: MeFeatureSlot,
    pub wallet_tracker_watchlist: MeWatchlistSlot,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeWatchlistSlot {
    #[serde(default)]
    pub used: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeResponse {
    pub subscriber: String,
    pub tier: String,
    pub tier_label: String,
    pub subscription: Option<MeSubscription>,
    pub quota: MeQuota,
    pub features: MeFeatureUsage,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── /tokens (directory list) — v0.8 ─────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct TokensListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc: Option<f64>,
    /// Default 2000. Set Some(0.0) to disable the dust floor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_liq: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_h: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_dex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_revoked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_token2022: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_lp_burnt_pct: Option<f64>,
    /// Post-filter: organic-volume floor in last 1h.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_volume_1h_usd: Option<f64>,
    /// Post-filter: MEV/bot share ceiling as % of total.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mev_share_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mc_change_1h_min_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mc_change_1h_max_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Lower bound on `liquidity_to_mc_ratio` (`liquidity_usd / market_cap`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_liq_mc_ratio: Option<f64>,
    /// Upper bound on `liquidity_to_mc_ratio`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_liq_mc_ratio: Option<f64>,
    /// Filter tokens by deployer reputation tier (e.g. `"elite"`, `"good"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployer_tier: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenSummary {
    pub mint: String,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub price_usd: Option<f64>,
    pub market_cap_usd: Option<f64>,
    pub fdv_usd: Option<f64>,
    pub liquidity_usd: Option<f64>,
    pub primary_dex: Option<String>,
    pub authorities_revoked: bool,
    pub lp_burnt_pct: Option<f64>,
    pub is_token_2022: bool,
    pub last_trade_time: Option<String>,
    pub mc_change_5m_pct: Option<f64>,
    pub mc_change_1h_pct: Option<f64>,
    pub organic_volume_1h_usd: Option<f64>,
    pub mev_share_pct: Option<f64>,
    /// Liquidity-to-MC ratio (`liquidity_usd / market_cap`).
    pub liquidity_to_mc_ratio: Option<f64>,
    /// Deployer reputation tier (e.g. `"elite"`, `"good"`, `"unranked"`).
    pub deployer_tier: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokensListPagination {
    pub limit: u32,
    pub offset: u32,
    pub returned: u32,
    pub has_more: bool,
    pub post_filtered: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokensListResponse {
    pub tokens: Vec<TokenSummary>,
    pub pagination: TokensListPagination,
    pub filters: serde_json::Value,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Price alerts (v1.9) ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PriceAlertDeliveryMode {
    Webhook,
    Websocket,
    Both,
}

impl PriceAlertDeliveryMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Webhook => "webhook",
            Self::Websocket => "websocket",
            Self::Both => "both",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PriceAlertStatus {
    Watching,
    Dipped,
    Recovered,
    Expired,
}

impl PriceAlertStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Watching => "watching",
            Self::Dipped => "dipped",
            Self::Recovered => "recovered",
            Self::Expired => "expired",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PriceAlertCreateParams {
    /// Solana mint address.
    pub token_mint: String,
    /// Drop % threshold (0.01–99.99). Alert fires when MC drops below baseline x (1 - drop_pct/100).
    pub drop_pct: f64,
    /// Recovery % threshold (0.01–1000). After dip fires, alert fires again when MC rises
    /// above dip_low x (1 + recovery_pct/100). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_pct: Option<f64>,
    /// Optional label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Default: "webhook".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<PriceAlertDeliveryMode>,
    /// Required when delivery_mode is "webhook" or "both". Must be HTTPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PriceAlertUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<PriceAlertDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceAlert {
    pub id: i64,
    #[serde(default)]
    pub name: Option<String>,
    pub token_mint: String,
    #[serde(default)]
    pub token_symbol: Option<String>,
    pub baseline_mc_usd: f64,
    pub drop_pct: f64,
    #[serde(default)]
    pub recovery_pct: Option<f64>,
    pub status: PriceAlertStatus,
    #[serde(default)]
    pub dip_low_mc_usd: Option<f64>,
    #[serde(default)]
    pub dip_fired_at: Option<String>,
    pub delivery_mode: PriceAlertDeliveryMode,
    #[serde(default)]
    pub webhook_url: Option<String>,
    pub is_active: bool,
    pub expires_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceAlertListResponse {
    pub alerts: Vec<PriceAlert>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceAlertCreateResponse {
    pub alert: PriceAlert,
    /// One-time HMAC secret. Save it — will not be shown again.
    #[serde(default)]
    pub webhook_secret: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceAlertGetResponse {
    pub alert: PriceAlert,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceAlertUpdateResponse {
    pub alert: PriceAlert,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceAlertDeleteResponse {
    pub deleted: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceAlertEvent {
    pub id: i64,
    pub alert_id: i64,
    /// "dip" or "recovery".
    pub event_type: String,
    pub fired_at: String,
    pub token_mint: String,
    pub baseline_mc_usd: f64,
    pub current_mc_usd: f64,
    #[serde(default)]
    pub drop_pct_actual: Option<f64>,
    #[serde(default)]
    pub dip_low_mc_usd: Option<f64>,
    #[serde(default)]
    pub recovery_pct_actual: Option<f64>,
    pub delivered: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PriceAlertEventsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceAlertEventsResponse {
    pub events: Vec<PriceAlertEvent>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Almost-bonded (/tokens/almost-bonded) — v0.18 ──────────────────────────

/// Sort order for [`Token::almost_bonded`](crate::api::token::Token::almost_bonded).
/// Defaults to [`AlmostBondedSort::VelocityDesc`] server-side when unset.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum AlmostBondedSort {
    /// Fastest bonding-progress velocity first (default).
    #[default]
    VelocityDesc,
    /// Closest to graduation first.
    ProgressDesc,
    /// Shortest estimated time-to-bond first.
    EtaAsc,
}

impl AlmostBondedSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VelocityDesc => "velocity_desc",
            Self::ProgressDesc => "progress_desc",
            Self::EtaAsc => "eta_asc",
        }
    }
}

/// Query params for [`Token::almost_bonded`](crate::api::token::Token::almost_bonded).
/// Unset fields are omitted from the query string.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AlmostBondedParams {
    /// Lower bound on bonding progress (percent, 0–100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_progress: Option<f64>,
    /// Upper bound on bonding progress (percent, 0–100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_progress: Option<f64>,
    /// Lower bound on bonding-progress velocity (percentage points per minute).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_velocity_pct_per_min: Option<f64>,
    /// Upper bound on token age in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_minutes: Option<f64>,
    /// Filter by deployer reputation tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployer_tier: Option<DeployerTier>,
    /// Only tokens whose mint+freeze authorities are revoked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_revoked: Option<bool>,
    /// Lower bound on liquidity (USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_liq: Option<f64>,
    /// Sort order — defaults to `velocity_desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<AlmostBondedSort>,
    /// Max number of tokens to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// A pre-bond pump.fun token near graduation. Nullable fields are `None` when
/// the underlying data was unavailable.
#[derive(Debug, Clone, Deserialize)]
pub struct AlmostBondedToken {
    pub mint: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    /// Bonding-curve progress toward graduation (percent, 0–100).
    pub progress_pct: f64,
    /// Bonding-progress velocity (percentage points per minute).
    #[serde(default)]
    pub velocity_pct_per_min: Option<f64>,
    /// Estimated minutes until graduation at the current velocity.
    #[serde(default)]
    pub eta_minutes: Option<f64>,
    /// Whether progress has stalled (no recent forward movement).
    pub stalled: bool,
    /// Real SOL reserves currently in the bonding curve.
    #[serde(default)]
    pub real_sol_reserves: Option<f64>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    /// Whether mint+freeze authorities are revoked.
    pub authorities_revoked: bool,
    /// Deployer reputation tier (e.g. `"elite"`, `"good"`, `"unranked"`).
    #[serde(default)]
    pub deployer_tier: Option<String>,
    #[serde(default)]
    pub age_minutes: Option<f64>,
}

/// Pre-bond pump.fun tokens near graduation, ranked by velocity (PRO/ULTRA).
#[derive(Debug, Clone, Deserialize)]
pub struct AlmostBondedResponse {
    pub tokens: Vec<AlmostBondedToken>,
    pub filters: serde_json::Value,
    pub returned: u32,
    pub note: String,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}
