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

fn main() {
    let account = Account::new(1, String::from("Noah"));

    let account_ref = &account;
    let account_ref2 = &account;

    print_account(account_ref);
    print_account(account_ref2);

    println!("{:#?}", account);
}
