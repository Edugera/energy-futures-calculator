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
}

fn calculate_notional(contract: &EnergyContract) -> f64 {
    contract.volume_mwh * contract.buy_price
}

fn calculate_pnl(contract: &EnergyContract) -> f64 {
    (contract.sell_price - contract.buy_price)
        * contract.volume_mwh
}

fn calculate_risk(contract: &EnergyContract) -> f64 {
    calculate_notional(contract) * 0.05
}

fn show_status(contract: &EnergyContract) {

    match contract.status {

        ContractStatus::Active =>
            println!("Status: Active"),

        ContractStatus::Settled =>
            println!("Status: Settled"),

        ContractStatus::Defaulted =>
            println!("Status: Defaulted"),
    }

}

fn main() {

    let contract = EnergyContract {
        buyer: String::from("Industrial Client A"),
        volume_mwh:100.0,
        buy_price:220.0,
        sell_price:245.0,
        status: ContractStatus::Active,
    };

    let notional=
        calculate_notional(&contract);

    let pnl=
        calculate_pnl(&contract);

    let risk=
        calculate_risk(&contract);

    show_status(&contract);
    
    println!("Energy Futures Contract");
    println!("----------------------");

    println!("Buyer: {}",contract.buyer);

    println!(
      "Volume: {} MWh",
      contract.volume_mwh
    );

    println!(
      "Buy Price: R$ {}",
      contract.buy_price
    );

    println!(
      "Sell Price: R$ {}",
      contract.sell_price
    );

    println!("Notional: R$ {}",notional);

    println!("PnL: R$ {}",pnl);

    println!("Risk Exposure: R$ {}",risk);

}

