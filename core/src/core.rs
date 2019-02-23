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

/// Account struct  
/// Represents a ledger account.
pub struct Account {
    /// Account ID. It's given by the accountant,
    /// but it behaves like a consant.
    account_id: u32,

    /// Debit total  
    /// Rolling up the debit total.
    d_total: u32,

    /// Credit total  
    /// Rolling up the credit total.
    c_total: u32,

    /// Balance  
    /// debit total - credit total.
    balance: i32,
}

/// Implement Account functionality
///
/// # Examples
///
///```rust
/// // Create two accounts:
/// // bank_account, using 38 as ID,
/// // immaterial_assets, using 11 as ID,
/// // then transfer value from bank_account to immaterial_assets.
/// let bank_account = Account::new(38);
/// let immaterial_assets = Account::new(11);
/// bank_account.credit_from(immaterial_assets, 1000);
/// ```
impl Account {
    /// Create new instance of Account
    ///
    /// It's a public function  
    /// Return a new Account with initial values.
    ///
    /// # Examples
    /// ```rust
    /// // create a bank_account using ID 38 // Hungarian example.
    /// let bank_account = Account::new(38);
    /// ...
    /// ```
    pub fn new(account_id: u32) -> Account {
        Account {
            /// account_id is given by calling ::new(account_id)
            account_id,
            /// 0 as init value. It is going to get its current value after  
            /// loading data from ledger database.
            d_total: 0,
            /// 0 as init value. It is going to get its current value after  
            /// loading data from ledger database.
            c_total: 0,
            /// 0 as init value. It is going to get its current value after  
            /// loading data from ledger database.
            balance: 0,
        }
    }
    /// Private function, create credit  
    /// It means it makes a credit side transaction,  
    /// So once e.g under the Hungarian laws.: debit 16, credit 38, value 1000,  
    /// means:  
    /// Account::new(16).debit(1000);  
    /// Account::new(38).credit(1000);  
    fn credit(&mut self, value: u32) {
        self.c_total += value;
        self.balance -= value as i32;
    }
    /// Private function, create debit  
    /// It means it makes a debit side transaction,  
    /// So once e.g under the Hungarian laws.: debit 16, credit 38, value 1000,  
    /// means:  
    /// Account::new(16).debit(1000);  
    /// Account::new(38).credit(1000);  
    fn debit(&mut self, value: u32) {
        self.d_total += value;
        self.balance += value as i32;
    }
    /// Credit from
    ///
    /// Once you have and account selected,  
    /// you can call credit_from() to transfer a given amount of value from  
    /// another account.
    ///
    /// # Examples
    /// ```rust
    /// /**
    ///  * account_a.get_balance() => 2000;
    ///  * account_b.get_balance() => 4000;
    ///  */
    /// let account_a = Account::new(1);
    /// let account_b = Account::new(3);
    ///
    /// /**
    ///  * Let's move 1000 from
    ///  * account_b to account_a;
    ///  */
    /// account_a.credit_from(account_b, 1000);
    ///
    /// /**
    ///  * Now the new balance(s):
    ///  *
    ///  * account_a.get_balance() => 3000;
    ///  * account_b.get_balance() => 3000;
    ///  */
    /// ```
    pub fn credit_from(&mut self, account: &mut Account, value: u32) {
        self.debit(value);
        account.credit(value);
    }
    pub fn get_account_id(&self) -> u32 {
        self.account_id
    }
    /// Get account balance,  
    /// returns `i32`.
    ///
    /// # Examples
    /// ```rust
    /// let account_a = Account::new(1);
    /// let account_b = Account::new(2);
    ///
    /// account_a.credit_from(account_b,1000);
    /// account_a.get_balance() // => 1000
    /// ```
    pub fn get_balance(&self) -> i32 {
        self.balance
    }
    pub fn get_c_total(&self) -> u32 {
        self.c_total
    }
    pub fn get_d_total(&self) -> u32 {
        self.d_total
    }

    /// Print account details
    ///
    /// Helper function, print details of a given account.  
    /// For testing purpose.
    ///
    /// # Examples
    /// ```rust
    /// let bank = Account::new(3811);
    /// bank.print_details();
    /// ```
    pub fn print_details(&self) {
        println!(
            "Account ({}) => balance: {}, debit total: {}, credit total: {}",
            self.get_account_id(),
            self.get_balance(),
            self.get_d_total(),
            self.get_c_total(),
        )
    }
}
