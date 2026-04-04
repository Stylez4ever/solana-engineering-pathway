trait Investment<T> {
    fn amount(&self) -> T;

    //fn set_amount(&mut self, new_amount: f64); 

    fn double_amount(&mut self);  
    
}

trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn double_amount(&mut self) {
        self.amount = self.amount * 2.0
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    value: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    fn double_amount(&mut self) {
        self.value = self.value * 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.10;
}

#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}

impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    fn double_amount(&mut self) {
        self.minutes = self.minutes * 2;
    }
}


fn practice_4() {
    let mut income = Income { amount: 3000.50};
    println!("Total tax owed: R{:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed: R{:.2}", income.tax_bill());


    let mut bonus = Bonus { value: 3000.50};
    println!("Total tax owed: R{:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Total tax owed: R{:.2}", bonus.tax_bill());

    let mut weekend = QualityTime { minutes: 120};
    weekend.double_amount();
    println!("Relaxation time: {:#?} min", weekend);
}

