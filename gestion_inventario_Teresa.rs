use std::collections::HashMap;
use std::io;
use std::io::{Write, BufRead};

// Define the structure for the products
#[derive(Clone, Debug)]
struct Product {
    name: String,
    code: String,
    cost_price: f64,
    sale_price: f64,
    quantity: i16,
    provider: String,
}

struct HashTable {
    table: HashMap<String, Vec<(String, Product)>>,
}

impl HashTable {
    // Constructor of a new hash table
    fn new() -> Self {
        HashTable {
            table: HashMap::new(),
        }
    }

    // Function to insert key-value pair into hash table
    fn insert(&mut self, key: String, value: Product) {
        let entry = self.table.entry(key.clone()).or_insert_with(Vec::new);
        if !entry.is_empty() {
            println!("Repeated product.");
        }
        entry.push((key, value));
    }

    // Function to search a value by its key in the hash table
    fn get(&self, key: &str) -> Vec<&Product> {
        if let Some(bucket) = self.table.get(key) {
            return bucket.iter().map(|(_, v)| v).collect();
        }
        Vec::new()
    }

    // Function to print all entries in the hash table
    fn print_all(&self) {
        clear_screen();
        println!("                      All the products\n");
        for (key, bucket) in &self.table {
            for (k, v) in bucket {
                println!("{:?}", v);
            }
        }
    }

    // Function to modify a value by its key in the hash table
    fn modify(&mut self, key: &str, new_value: Product) -> bool {
        if let Some(bucket) = self.table.get_mut(key) {
            if bucket.len() > 1 {
                println!("Multiple products found with the same key. Choose which one to modify:");
                for (i, (_, product)) in bucket.iter().enumerate() {
                    println!("{}. {:?}", i + 1, product);
                }
                let choice = read_choice(bucket.len());
                if let Some((_, v)) = bucket.get_mut(choice - 1) {
                    *v = new_value;
                    return true;
                }
            } else if let Some((_, v)) = bucket.first_mut() {
                *v = new_value;
                return true;
            }
        }
        false
    }

    // Function to remove a value by its key from the hash table
    fn remove(&mut self, key: &str) {
        if let Some(bucket) = self.table.get_mut(key) {
            if bucket.len() > 1 {
                println!("Multiple products found with the same key. Choose which one to remove:");
                for (i, (_, product)) in bucket.iter().enumerate() {
                    println!("{}. {:?}", i + 1, product);
                }
                let choice = read_choice(bucket.len());
                bucket.remove(choice - 1);
            } else {
                bucket.clear();
            }
        }
    }
}

fn read_choice(max: usize) -> usize {
    loop {
        print!("Enter your choice (1-{}): ", max);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice >= 1 && choice <= max {
                return choice;
            }
        }
        println!("Invalid choice. Please try again.");
    }
}

fn main() {
    // Create a new hash table
    let mut hash_table = HashTable::new();

    // Menu
    loop {
        clear_screen();
        println!("          STATIONERY.COM product inventory\n");
        println!("      1. Consult");
        println!("      2. Register");
        println!("      3. Modify");
        println!("      4. Delete");
        println!("      5. Leave");
        print!("                      Option: ");
        io::stdout().flush().unwrap(); // Ensures that the message is printed before waiting for input
        let input = read_string();

        // Remove any white space at the beginning and end of the entry
        let input = input.trim();

        // Convert input to an integer
        let choice: i32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        clear_screen();
        // Process user choice
        match choice {
            1 => {
                if !hash_table.table.is_empty() {
                    loop {
                        clear_screen();
                        println!("                      Consult\n");
                        println!("      1. Show all");
                        println!("      2. Show one");
                        println!("      3. Return to menu");
                        print!("                      Option: ");
                        io::stdout().flush().unwrap();
                        let input = read_string();
                        let input = input.trim();
                        let choice: i32 = match input.parse() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Please enter a valid number.");
                                continue;
                            }
                        };
                        match choice {
                            1 => {
                                hash_table.print_all();
                                wait_for_enter();
                            }
                            2 => {
                                clear_screen();
                                print!("Product to show: ");
                                io::stdout().flush().unwrap();
                                let input2 = read_string();
                                let products = hash_table.get(&input2);
                                if !products.is_empty() {
                                    for product in products {
                                        println!("{:?}", product);
                                    }
                                } else {
                                    println!("{} not found in the inventory", input2);
                                }
                                wait_for_enter();
                            }
                            3 => {
                                println!("Leaving...");
                                wait_for_enter();
                                break;
                            }
                            _ => println!("Invalid option."),
                        }
                    }
                } else {
                    println!("Empty inventory, register products first.");
                    wait_for_enter();
                }
            }
            2 => {
                print!("Product to register: ");
                io::stdout().flush().unwrap();
                let product_name = read_string();
                print!("Enter code: ");
                io::stdout().flush().unwrap();
                let code = read_string();
                print!("Enter cost price: ");
                io::stdout().flush().unwrap();
                let cost_price = read_f64();
                print!("Enter sale price: ");
                io::stdout().flush().unwrap();
                let sale_price = read_f64();
                print!("Enter quantity: ");
                io::stdout().flush().unwrap();
                let quantity = read_i16();
                print!("Enter provider: ");
                io::stdout().flush().unwrap();
                let provider = read_string();
                // Create a new product with the provided name and default values for the other fields
                let new_product = Product {
                    name: product_name.clone(),
                    code,
                    cost_price,
                    sale_price,
                    quantity,
                    provider,
                };
                // Insert the new product into the hash table
                hash_table.insert(product_name.clone(), new_product);
                println!("Product '{}' was registered.", product_name);
                wait_for_enter();
            }
            3 => {
                if !hash_table.table.is_empty() {
                    print!("Product to modify: ");
                    io::stdout().flush().unwrap();
                    let product_name = read_string();
                    if !hash_table.get(&product_name).is_empty() {
                        let name = product_name.clone();
                        print!("Enter new code: ");
                        io::stdout().flush().unwrap();
                        let code = read_string();
                        print!("Enter new cost price: ");
                        io::stdout().flush().unwrap();
                        let cost_price = read_f64();
                        print!("Enter new sale price: ");
                        io::stdout().flush().unwrap();
                        let sale_price = read_f64();
                        print!("Enter new quantity: ");
                        io::stdout().flush().unwrap();
                        let quantity = read_i16();
                        print!("Enter new provider: ");
                        io::stdout().flush().unwrap();
                        let provider = read_string();
                        // Modify the product
                        let new_product = Product {
                            name,
                            code,
                            cost_price,
                            sale_price,
                            quantity,
                            provider,
                        };
                        if hash_table.modify(&product_name, new_product) {
                            println!("Product modified successfully.");
                        } else {
                            println!("Error modifying product.");
                        }
                    } else {
                        println!("Product not found.");
                    }
                    wait_for_enter();
                } else {
                    println!("Empty inventory, register products first.");
                    wait_for_enter();
                }
            }
            4 => {
                if !hash_table.table.is_empty() {
                    print!("Product to delete: ");
                    io::stdout().flush().unwrap();
                    let product_name = read_string();
                    if !hash_table.get(&product_name).is_empty() {
                        hash_table.remove(&product_name);
                        println!("Product deleted successfully.");
                    } else {
                        println!("Product not found.");
                    }
                    wait_for_enter();
                } else {
                    println!("Empty inventory, register products first.");
                    wait_for_enter();
                }
            }
            5 => {
                println!("Exiting the program...");
                break;
            }
            _ => println!("Invalid option, please enter a valid number."),
        }
    }
}

fn clear_screen() {
    #[cfg(unix)]{
        let _ = std::process::Command::new("clear").status();
    }
    #[cfg(windows)]{
        let _ = std::process::Command::new("cmd").arg("/c").arg("cls").status();
    }
}

fn read_string() -> String {
    loop {
        let mut input_string = String::new();
        io::stdin().lock().read_line(&mut input_string).expect("Error");
        let trimmed_input = input_string.trim();
        if !trimmed_input.is_empty() {
            return trimmed_input.to_string();
        } else {
            println!("Blank spaces are not accepted.");
        }
    }
}

fn read_f64() -> f64 {
    loop {
        let input_string = read_string();
        match input_string.parse::<f64>() {
            Ok(num) if num >= 0.0 => return num,
            Ok(_) => println!("Negative numbers are not accepted."),
            Err(_) => println!("Invalid number."),
        }
    }
}

fn read_i16() -> i16 {
    loop {
        let input_string = read_string();
        match input_string.parse::<i16>() {
            Ok(num) if num >= 0 => return num,
            Ok(_) => println!("Negative numbers are not accepted."),
            Err(_) => println!("Invalid number."),
        }
    }
}

fn wait_for_enter() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}