trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64;
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Taxable for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Income::TAX_RATE
    }
}

#[derive(Debug)]
struct Bonus {
    amount: f64,
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;


    fn tax_bill(&self) -> f64 {
        self.amount * Bonus::TAX_RATE
    }
}


fn practice_1() {
    let income = Income { amount: 3000.50};
    println!("Total tax owed: R{:.2}", income.tax_bill());

    let bonus = Bonus { amount: 3000.50};
    println!("Total tax owed: R{:.2}", bonus.tax_bill());

}
