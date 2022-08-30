use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Bill{
    name: String,
    amount: f64,
}

pub struct Bills{
    inner:  HashMap<String, Bill>,
}

impl Bills{
    fn new() -> Self{
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill){ //&mut self since we are making modification
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool { //&mut self since we are making modification
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool{
        match self.inner.get_mut(name) { //get_mut is used to mutate the item in HashMap
            Some(bill) => {
                bill.amount = amount;
                true
            },
            None => false,
        }
    }
}

fn get_input()-> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("Please enter your data again!");
    }

    let input = buffer.trim().to_owned();
    if &input == ""{
        None
    }else{
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop{
        let input = match get_input(){
            Some(input) => input,
            None => return None,
        };
        if &input == ""{
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input{
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

mod menu{
    use crate::{Bill, Bills, get_input, get_bill_amount};

    pub fn add_bill(bills: &mut Bills){
        println!("Bill Name: ");
        let input_name = match get_input(){
            Some(input) => input,
            None => return,
        };

        let input_amount = match get_bill_amount(){
            Some(input) => input,
            None => return,
        };

        let bill = Bill { 
            name: input_name, 
            amount: input_amount,
        };

        bills.add(bill);
        println!("Bills Added");
    }

    pub fn view_bills(bills: &Bills){
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }

    pub fn remove_bill(bills: &mut Bills){
        for bill in bills.get_all(){
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove: ");
        let bill_name = match get_input(){
            Some(name) => name,
            None => return,
        };

        if bills.remove(&bill_name){
            println!("Bill removed!");
        }else{
            println!("Bill Not Found!");
        }
    }

    pub fn update_bills(bills: &mut Bills){
        for bill in bills.get_all(){
            println!("{:?}", bill);
        }
        println!("Enter bill name to Edit: ");
        let bill_name = match get_input(){
            Some(name) => name,
            None => return,
        };

        let new_amount = match get_bill_amount(){
            Some(input) => input,
            None => return,
        };

        if bills.update(&bill_name, new_amount){
            println!("{:?} is Edited with new amount: {:?}", bill_name, new_amount);
        }else{
            println!("Bill not found!");
        }
    }
}

enum MainMenu{
    AddBill,
    ViewBill,
    RemoveBill,
    EditBill,
}

impl MainMenu{
    fn from_str(input: &str)-> Option<MainMenu> {
        match input{
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBill),
            "3" => Some(MainMenu::RemoveBill),
            "4" => Some(MainMenu::EditBill),
            _ => None
        }
    }

    fn show(){
        println!("");
        println!("-----Bill Manager-----");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("3. Delete Bill");
        println!("4. Edit Bill");
        println!("");
        println!("Enter the selection: ");
    }
}

fn init_program() -> Option<()> {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::EditBill) => menu::update_bills(&mut bills),
            None => break,
        }
    }
    None
}

fn main() {
    init_program();
}
