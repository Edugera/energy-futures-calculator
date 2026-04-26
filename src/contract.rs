pub enum ContractStatus {
    Active,
    Settled,
    Defaulted,
}

pub struct EnergyContract {
    pub token_id: u64,
    pub buyer: String,
    pub volume_mwh: f64,
    pub buy_price: f64,
    pub sell_price: f64,
    pub status: ContractStatus,
    pub collateral_posted: f64,
    pub margin_requirement_rate: f64,
}