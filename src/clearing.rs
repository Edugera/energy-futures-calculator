use crate::contract::{ContractStatus, EnergyContract};

pub fn show_status(contract: &EnergyContract) {
    match contract.status {
        ContractStatus::Active => println!("Status: Active"),
        ContractStatus::Settled => println!("Status: Settled"),
        ContractStatus::Defaulted => println!("Status: Defaulted"),
    }
}