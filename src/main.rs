mod contract;
mod risk;
mod clearing;

use contract::{ContractStatus, EnergyContract};
use clearing::show_status;
use risk::{
    calculate_margin_requirement,
    calculate_notional,
    calculate_pnl,
    calculate_shortfall,
};

fn main() {
    let contracts = vec![
        EnergyContract {
            token_id: 1001,
            buyer: String::from("Industrial Client A"),
            volume_mwh: 100.0,
            buy_price: 220.0,
            sell_price: 245.0,
            status: ContractStatus::Active,
            collateral_posted: 800.0,
            margin_requirement_rate: 0.05,
        },
        EnergyContract {
            token_id: 1002,
            buyer: String::from("Generator B"),
            volume_mwh: 150.0,
            buy_price: 210.0,
            sell_price: 230.0,
            status: ContractStatus::Settled,
            collateral_posted: 2000.0,
            margin_requirement_rate: 0.05,
        },
        EnergyContract {
            token_id: 1003,
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

    let guarantee_fund = 1500.0;

    println!("Energy Futures Clearing Pool");
    println!("----------------------------");

    for contract in contracts.iter() {
        println!("");
        println!("Buyer: {}", contract.buyer);
        println!("Token ID: {}", contract.token_id);
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

    if guarantee_fund >= total_shortfall {
        println!("Guarantee Fund absorbs shortfall.");
    } else {
        println!("Residual systemic risk remains.");
    }
}