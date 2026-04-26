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

fn calculate_shortfall(contract: &EnergyContract) -> f64 {
    let margin = calculate_margin_requirement(contract);

    if contract.collateral_posted >= margin {
        0.0
    } else {
        margin - contract.collateral_posted
    }
}

fn show_status(contract: &EnergyContract) {
    match contract.status {
        ContractStatus::Active => println!("Status: Active"),
        ContractStatus::Settled => println!("Status: Settled"),
        ContractStatus::Defaulted => println!("Status: Defaulted"),
    }
}

fn main() {
    let contracts = vec![
        EnergyContract {
            buyer: String::from("Industrial Client A"),
            volume_mwh: 100.0,
            buy_price: 220.0,
            sell_price: 245.0,
            status: ContractStatus::Active,
            collateral_posted: 800.0,
            margin_requirement_rate: 0.05,
        },
        EnergyContract {
            buyer: String::from("Generator B"),
            volume_mwh: 150.0,
            buy_price: 210.0,
            sell_price: 230.0,
            status: ContractStatus::Settled,
            collateral_posted: 2000.0,
            margin_requirement_rate: 0.05,
        },
        EnergyContract {
            buyer: String::from("Retailer C"),
            volume_mwh: 80.0,
            buy_price: 260.0,
            sell_price: 240.0,
            status: ContractStatus::Defaulted,
            collateral_posted: 300.0,
            margin_requirement_rate: 0.05,
        },
    ];

    let mut total_notional = 0.0;
    let mut total_pnl = 0.0;
    let mut total_margin = 0.0;
    let mut total_collateral = 0.0;
    let mut total_shortfall = 0.0;

    println!("Energy Futures Clearing Pool");
    println!("----------------------------");

    for contract in contracts.iter() {
        println!("");
        println!("Buyer: {}", contract.buyer);
        show_status(contract);

        let notional = calculate_notional(contract);
        let pnl = calculate_pnl(contract);
        let margin = calculate_margin_requirement(contract);
        let shortfall = calculate_shortfall(contract);

        println!("Notional: R$ {}", notional);
        println!("PnL: R$ {}", pnl);
        println!("Margin Requirement: R$ {}", margin);
        println!("Collateral Posted: R$ {}", contract.collateral_posted);
        println!("Shortfall: R$ {}", shortfall);

        total_notional += notional;
        total_pnl += pnl;
        total_margin += margin;
        total_collateral += contract.collateral_posted;
        total_shortfall += shortfall;
    }

    println!("");
    println!("Pool Summary");
    println!("------------");
    println!("Total Notional: R$ {}", total_notional);
    println!("Total PnL: R$ {}", total_pnl);
    println!("Total Margin Requirement: R$ {}", total_margin);
    println!("Total Collateral Posted: R$ {}", total_collateral);
    println!("Total Shortfall: R$ {}", total_shortfall);

    if total_shortfall > 0.0 {
        println!("Pool Risk Status: clearing pool protection required.");
    } else {
        println!("Pool Risk Status: fully collateralized.");
    }
}