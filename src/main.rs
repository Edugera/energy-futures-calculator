fn calculate_notional(volume_mwh: f64, price_mwh: f64) -> f64 {
    volume_mwh * price_mwh
}

fn calculate_pnl(
    buy_price: f64,
    sell_price: f64,
    volume_mwh: f64
) -> f64 {

    (sell_price - buy_price) * volume_mwh
}

fn calculate_risk_exposure(notional: f64, risk_percent: f64) -> f64 {
    notional * risk_percent
}

fn main() {

    let volume = 100.0;

    let buy_price = 220.0;
    let sell_price = 245.0;

    let risk_percent = 0.05;

    let notional =
        calculate_notional(volume,buy_price);

    let pnl =
        calculate_pnl(
            buy_price,
            sell_price,
            volume
        );
    
    let risk_exposure = calculate_risk_exposure(notional, risk_percent);
    
    println!("Energy Futures Calculator");
    println!("-------------------------");

    println!("Volume: {} MWh",volume);
    println!("Buy Price: R$ {}",buy_price);
    println!("Sell Price: R$ {}",sell_price);

    println!("Contract Notional: R$ {}",notional);
    println!("PnL: R$ {}",pnl);

    println!("Risk Expposure 5%: R$ {}", risk_exposure);

    if pnl > 0.0 {
        println!("Trade Result: Profitable trade");
    } else if pnl < 0.0 {
        println!("Trade Result: Loss trade");
    } else {
        println!("Trade Result: Break-even");
    }

}

