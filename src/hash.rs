use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Account{
    username: String,
    password: String,
}
#[derive(PartialEq, Eq, Hash, Debug)]
struct AccountInfo{
    name: String,
    email: String,
}

type Accounts = HashMap<Account, AccountInfo>;

pub fn run() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "thiya".to_string(),
        password: "password123".to_string(),
    };

    let account_info = AccountInfo {
        name: "Santhiyaa".to_string(),
        email: "san@email.com".to_string(),
    };

    accounts.insert(account, account_info);
    // println!("{:#?}", accounts);

    login(&accounts, "thiya".to_string(), "password123".to_string());
    // login(&accounts, "j.everyman".to_string(), "password123".to_string());

}

fn login(accounts: &Accounts, username: String, password: String) {

    let logon = Account {
        username,
        password
    };
    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful Login!..");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        },
        _ => println!("Login Failed..."),
    }
}