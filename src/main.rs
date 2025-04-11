trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance = self.balance + amount;
        println!(
            "{} deposited succesfully, balance is {}",
            amount, self.balance
        )
    }
    fn withdraw(&mut self, amount: f64) {
        self.balance = self.balance - amount;
        println!(
            "{} withdrawed succesfully, balance is {}",
            amount, self.balance
        )
    }
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account_1 = BankAccount {
        holder_name: "Toly".to_string(),
        account_number: 1000,
        balance: 234.00,
    };
    let mut account_2 = BankAccount {
        holder_name: "Seb".to_string(),
        account_number: 102882,
        balance: 9884.00,
    };

    println!("account_1 balance {}", account_1.balance().to_string());

    account_1.deposit(3000.0);

    println!("account_1 balance {}", account_1.balance().to_string());

    account_1.withdraw(250.0);

    println!("account_1 balance {}", account_1.balance().to_string());

    println!("account_2 balance {}", account_2.balance().to_string());

    account_2.deposit(3000.0);

    println!("account_2 balance {}", account_2.balance().to_string());

    account_2.withdraw(250.0);

    println!("account_2 balance {}", account_2.balance().to_string());
}
