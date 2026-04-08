fn practice_18() {

    let stock = ["nvda", "", "aapl", "", "msft", "goog"];

    let capitalized_stock: Vec<String> = stock.iter().filter(|stock| !stock.is_empty()).map(|stock| stock.to_uppercase()).collect();
    println!("{:?}", capitalized_stock);

    let capitalized_stock: Vec<String> = stock.iter()
    .filter_map(|stock| {
        if stock.is_empty() {
            None
        } else {
            Some(stock.to_uppercase())
        }
    }).collect();
    println!("{:?}", capitalized_stock);
}
