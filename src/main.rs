fn calculate_notional(volume_mwh: f64, price_mwh: f64) -> f64 {
    volume_mwh * price_mwh
}

fn main() {

    let volume = 100.0;
    let price = 235.0;

    let notional = calculate_notional(volume, price);

    println!("Energy Futures Calculator");
    println!("Volume: {} MWh", volume);
    println!("Price: R$ {}", price);
    println!("Contract Notional: R$ {}", notional);
}