use std::io;
use yuhcee_ep04_traits_modules::product::Product;
use yuhcee_ep04_traits_modules::order_status::{OrderStatus, print_status};

fn main() {
    println!("Enter product name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Enter product price:");
    let mut price = String::new();
    io::stdin().read_line(&mut price).expect("Failed to read line");
    let price: f64 = price.trim().parse().expect("Please type a number!");

    println!("Enter product stock:");
    let mut stock = String::new();
    io::stdin().read_line(&mut stock).expect("Failed to read line");
    let stock: u32 = stock.trim().parse().expect("Please type a number!");

    let product = Product {
        name: name.trim().to_string(),
        price,
        stock,
    };

    println!("Product: {}, Price: ${}, In Stock: {}", product.name, product.price, product.is_in_stock());

    println!("Enter order status (Pending, Shipped, Delivered):");
    let mut status_input = String::new();
    io::stdin().read_line(&mut status_input).expect("Failed to read line");
    let status = match status_input.trim() {
        "Pending" => OrderStatus::Pending,
        "Shipped" => OrderStatus::Shipped,
        "Delivered" => OrderStatus::Delivered,
        _ => {
            println!("Invalid status, defaulting to Pending");
            OrderStatus::Pending
        }
    };

    print_status(status);
}