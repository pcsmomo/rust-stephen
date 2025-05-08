#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_holder(holder: String) {
    println!("{}", holder);
}

// fn main_error() {
//     let account = Account::new(1, String::from("Noah"));

//     // this is ok
//     println!("{:?}", account);
//     println!("{:?}", account);

//     // this is not ok because the account is moved
//     print_account(account); // value moved
//     print_account(account); // error
// }

// fn main_error2() {
//     let bank = Bank::new();

//     let other_bank = bank;

//     println!("{:?}", bank);
// }

fn change_account(account: &mut Account) {
    account.balance = 10;
}

fn main() {
    let mut account = Account::new(1, String::from("Noah"));

    let account_ref = &mut account;

    account.balance = 100;

    change_account(account_ref);

    println!("{:#?}", account);
}
