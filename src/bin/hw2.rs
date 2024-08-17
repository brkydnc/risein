use std::num::Saturating;

trait Account {
    fn deposit(&mut self, amount: usize);
    fn withdraw(&mut self, amount: usize);
    fn balance(&mut self) -> usize;
}

#[allow(dead_code)]
struct BankAccount {
    holder_name: String,
    account_number: usize,
    balance: Saturating<usize>,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: usize) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: usize) {
        self.balance -= amount;
    }

    fn balance(&mut self) -> usize {
        self.balance.0
    }
}

fn main() {
    let mut berkay = BankAccount {
        holder_name: String::from("Berkay Dinc"),
        account_number: 1337,
        balance: Saturating(42),
    };

    let mut john = BankAccount {
        holder_name: String::from("John Doe"),
        account_number: 420,
        balance: Saturating(13),
    };

    berkay.deposit(100);
    john.withdraw(500);

    println!("Berkay's new balance: {}", berkay.balance());
    println!("John's new balance: {}", john.balance());
}
