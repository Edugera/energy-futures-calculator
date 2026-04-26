enum ContractStatus {
    Active,
    Settled,
    Defaulted,
}

struct EnergyContract {
    buyer: String,
    volume_mwh: f64,
    buy_price: f64,
    sell_price: f64,
    status: ContractStatus,
    collateral_posted: f64,
    margin_requirement_rate: f64,
}

fn calculate_notional(contract: &EnergyContract) -> f64 {
    contract.volume_mwh * contract.buy_price
}

fn calculate_pnl(contract: &EnergyContract) -> f64 {
    (contract.sell_price - contract.buy_price) * contract.volume_mwh
}

fn calculate_margin_requirement(contract: &EnergyContract) -> f64 {
    calculate_notional(contract) * contract.margin_requirement_rate
}

fn is_margin_sufficient(contract: &EnergyContract) -> bool {
    contract.collateral_posted >= calculate_margin_requirement(contract)
}

fn show_status(contract: &EnergyContract) {
    match contract.status {
        ContractStatus::Active => println!("Status: Active"),
        ContractStatus::Settled => println!("Status: Settled"),
        ContractStatus::Defaulted => println!("Status: Defaulted"),
    }
}

fn default_waterfall(contract: &EnergyContract) {
    let required_margin = calculate_margin_requirement(contract);

    if contract.collateral_posted >= required_margin {
        println!("Default Waterfall: no default. Collateral is sufficient.");
    } else {
        let shortfall = required_margin - contract.collateral_posted;

        println!("Default Waterfall: default risk detected.");
        println!("Margin Shortfall: R$ {}", shortfall);
        println!("Action 1: use posted collateral.");
        println!("Action 2: request additional margin.");
        println!("Action 3: trigger clearing pool protection.");
    }
}

fn main() {
    let contract = EnergyContract {
        buyer: String::from("Industrial Client A"),
        volume_mwh: 100.0,
        buy_price: 220.0,
        sell_price: 245.0,
        status: ContractStatus::Active,
        collateral_posted: 800.0,
        margin_requirement_rate: 0.05,
    };

    let notional = calculate_notional(&contract);
    let pnl = calculate_pnl(&contract);
    let margin_requirement = calculate_margin_requirement(&contract);

    println!("Energy Futures Clearing Prototype");
    println!("---------------------------------");

    show_status(&contract);

    println!("Buyer: {}", contract.buyer);
    println!("Volume: {} MWh", contract.volume_mwh);
    println!("Buy Price: R$ {}", contract.buy_price);
    println!("Sell Price: R$ {}", contract.sell_price);
    println!("Notional: R$ {}", notional);
    println!("PnL: R$ {}", pnl);

    println!("Collateral Posted: R$ {}", contract.collateral_posted);
    println!("Margin Requirement: R$ {}", margin_requirement);

    if is_margin_sufficient(&contract) {
        println!("Margin Check: sufficient collateral.");
    } else {
        println!("Margin Check: insufficient collateral.");
    }

    default_waterfall(&contract);
}