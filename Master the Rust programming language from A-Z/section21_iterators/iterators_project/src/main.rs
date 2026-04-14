#![allow(unused, dead_code)]
use std::{collections::{self, HashMap}, env};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> CustomerOrder {
        CustomerOrder { product, quantity, shipped }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    order: Vec<CustomerOrder>
}



fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];
 
    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];


    for order in orders.iter() {
        if order.product == Product::Blender {
            println!("{:#?}", order);
        }
    }

    println!();
    println!("......................................");
    println!();

    let filter_microves = orders
        .iter()
        .filter(|search| {
            search.product == Product::Microwave});

    let extract_quantity = filter_microves
        .map(|mut total| {
            total.quantity 
        });

    let mut total: u32 = 0; 
    for sum in extract_quantity {
        total = total + sum;
    }
    println!("The total microves quanity is: {:?}", total);

    println!();
    println!("......................................");
    println!();

    let args = env::args().skip(1).take(1);
    //if args >= 5 {
    //
    //    let search =orders
    //    .iter()
    //    .filter(|search| search.quantity => 5;).collections::<Vec<CustomerOrder>;
    //}


    
    
    let search: Vec<&CustomerOrder> =orders
        .iter()
        .filter(|search| {search.quantity >= 5})
        .collect();

    let search_2: Vec<&CustomerOrder> =orders
        .iter()
        .filter(|search| {search.quantity >= 2})
        .collect();
            


    for arg in args {
        if arg >= 5.to_string() { println!("{:#?}", search)
            } else {
                        println!("{:#?}", search_2);
                   }
    }


    println!();
    println!("......................................");
    println!();


    let inventory = HashMap::new();
    let unshipped = orders
        .iter()
        .filter(|locate| locate.shipped == false).map(|x| {
            let mut num = 0;

            
            inventory.insert(x.product, x.quantity)

        });

    
    

    let inventory = HashMap::new();
    inventory.insert(unshipped_prod, unshipped_quantity)
    
    



    
}





