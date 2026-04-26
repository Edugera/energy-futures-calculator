use crate::contract::EnergyContract;

pub fn calculate_notional(contract: &EnergyContract) -> f64 {
    contract.volume_mwh * contract.buy_price
}

pub fn calculate_pnl(contract: &EnergyContract) -> f64 {
    (contract.sell_price - contract.buy_price) * contract.volume_mwh
}

pub fn calculate_margin_requirement(contract: &EnergyContract) -> f64 {
    calculate_notional(contract) * contract.margin_requirement_rate
}

pub fn calculate_shortfall(contract: &EnergyContract) -> f64 {
    let margin = calculate_margin_requirement(contract);

    if contract.collateral_posted >= margin {
        0.0
    } else {
        margin - contract.collateral_posted
    }
}