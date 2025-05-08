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

fn make_and_print_account() -> &Account {
    let account = Account::new(1, String::from("Noah"));

    println!("{:#?}", account);

    &account
}

fn main() {
    let account_ref = make_and_print_account();
    println!("{:#?}", account_ref);
}
