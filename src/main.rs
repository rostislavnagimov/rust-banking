trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be greater than zero".to_string());
        }

        self.balance = self.balance + amount;

        println!(
            "{} deposited succesfully, balance is {}",
            amount, self.balance
        );

        Ok(())
    }
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be greater than zero".to_string());
        }

        if amount > self.balance {
            return Err("Insufficent funds".to_string());
        }
        self.balance = self.balance - amount;
        println!(
            "{} withdrawed succesfully, balance is {}",
            amount, self.balance
        );

        Ok(())
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

    match account_1.deposit(3000.0) {
        Ok(()) => println!("Deposit successful!"),
        Err(msg) => println!("Deposit failed: {}", msg),
    };

    println!("account_1 balance {}", account_1.balance().to_string());

    match account_1.withdraw(250.0) {
        Ok(()) => println!("Withdrawal successful!"),
        Err(msg) => println!("Withdrawal failed: {}", msg),
    }

    println!("account_1 balance {}", account_1.balance().to_string());

    println!("account_2 balance {}", account_2.balance().to_string());

    match account_1.deposit(200.0) {
        Ok(()) => println!("Deposit successful!"),
        Err(msg) => println!("Deposit failed: {}", msg),
    };

    println!("account_2 balance {}", account_2.balance().to_string());

    match account_1.withdraw(250.0) {
        Ok(()) => println!("Withdrawal successful!"),
        Err(msg) => println!("Withdrawal failed: {}", msg),
    }

    println!("account_2 balance {}", account_2.balance().to_string());
}
