// struct Event {
//     c: u32,
//     d: u32,
//     value: u32,
// }

// impl Event {
//     pub fn new(c: u32, d: u32, value: u32) -> Event {
//         Event { c, d, value }
//     }
// }

pub struct Account {
    account_id: u32,
    c_total: u32,
    d_total: u32,
    balance: i32,
}

impl Account {
    pub fn new(account_id: u32) -> Account {
        Account {
            account_id,
            c_total: 0,
            d_total: 0,
            balance: 0,
        }
    }
    pub fn credit(&mut self, value: u32) {
        self.c_total += value;
        self.balance -= value as i32;
    }
    pub fn debit(&mut self, value: u32) {
        self.d_total += value;
        self.balance += value as i32;
    }
    pub fn credit_from(&mut self, account: &mut Account, value: u32) {
        self.debit(value);
        account.credit(value);
    }
    pub fn get_account_id(&self) -> u32 {
        self.account_id
    }
    pub fn get_balance(&self) -> i32 {
        self.balance
    }
    pub fn get_c_total(&self) -> u32 {
        self.c_total
    }
    pub fn get_d_total(&self) -> u32 {
        self.d_total
    }
}

/// Print account details
///
/// Helper function, print details of a given account.
/// For testing purpose.
/// 
/// # Examples
/// let bank = Account::new(3811);
/// print_account_details(bank);
pub fn print_account_details(account: Account) {
    println!(
        "Account ({}) => balance: {}, debit total: {}, credit total: {}",
        account.get_account_id(),
        account.get_balance(),
        account.get_d_total(),
        account.get_c_total(),
    );
}
