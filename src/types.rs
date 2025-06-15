use serde::{Deserialize, Serialize};

/// Time in force for orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    Day,
    Ioc,
    Fok,
    Gtc,
}

/// Order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// Price type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriceType {
    Limit,
    Market,
    MarketOnClose,
    LimitOnClose,
}

/// Market type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketType {
    Stock,
    Future,
    Option,
}

/// Buy/Sell action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BSAction {
    Buy,
    Sell,
}

/// Future/Option market type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FutOptMarketType {
    Future,
    Option,
}

/// Future/Option order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FutOptOrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// Future/Option price type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FutOptPriceType {
    Limit,
    Market,
}

/// Call/Put option type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallPut {
    Call,
    Put,
}

/// Trigger content for conditional orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerContent {
    Price,
    Volume,
}

/// Trading type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradingType {
    Normal,
    DayTrade,
    Margin,
    Short,
}

/// Condition operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Equal,
}

/// Condition order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionOrderType {
    Stop,
    StopLimit,
    Oco,
}

/// Condition price type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionPriceType {
    Limit,
    Market,
}

/// Condition market type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionMarketType {
    Stock,
    Future,
    Option,
}

/// Stop sign
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopSign {
    Up,
    Down,
}

/// Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Buy,
    Sell,
}

/// Condition status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionStatus {
    Pending,
    Triggered,
    Cancelled,
    Expired,
}

/// History status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryStatus {
    Filled,
    PartiallyFilled,
    Cancelled,
    Rejected,
}

/// Order structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub symbol: String,
    pub quantity: u32,
    pub price: Option<f64>,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub action: BSAction,
}

/// Condition structure  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub symbol: String,
    pub operator: Operator,
    pub value: f64,
    pub trigger_content: TriggerContent,
}

/// Conditional order structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionOrder {
    pub condition: Condition,
    pub order: Order,
    pub order_type: ConditionOrderType,
}

/// Future/Option order structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutOptOrder {
    pub symbol: String,
    pub quantity: u32,
    pub price: Option<f64>,
    pub order_type: FutOptOrderType,
    pub action: BSAction,
}

/// Future/Option conditional order structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutOptConditionOrder {
    pub condition: Condition,
    pub order: FutOptOrder,
}

/// Account information returned from login
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account ID
    pub account_id: String,
    /// Account name
    pub account_name: String,
    /// Account type (e.g., "stock", "future", "option")  
    pub account_type: String,
    /// Account status
    pub status: String,
    /// Currency
    pub currency: String,
    /// Available balance
    pub available_balance: Option<f64>,
    /// Total balance
    pub total_balance: Option<f64>,
}

/// Login credentials structure
#[derive(Debug, Clone)]
pub struct LoginCredentials {
    /// Personal ID for login
    pub personal_id: String,
    /// Password
    pub password: String,
    /// Certificate path
    pub cert_path: String,
    /// Certificate password (optional)
    pub cert_pass: Option<String>,
}