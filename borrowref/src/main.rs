//References and Borrowing

// -I- Immutable Reference 
// fn main() {
//     let x: i32 = 5;
//     let r: &i32 = &x;
//     println!("the value of x is {}", x);
//     println!("the value of r is {}", r);
// }

// -II- Mutable reference
// fn main() {
//     let mut x: i32 = 5;
//     let r: &mut i32 = &mut x;
//     *r +=1;
//     *r -=3;

//    //println!("the value of x is {}", x);
//     println!("the value of r is {}", *r);
// }

// Borrowing and struct
fn main(){
    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();
    // Mutable borrow to withdraw money
    account.withdraw(50.0);
    //
    account.check_balance();
}

// suppose we have structure reprensenting a bank account

struct BankAccount {
    owner: String,
    balance: f64,
}
// We need to make sure that we don’t have mutable access to the account for updating the balance
// at the same time as immutable access for reading the owner’s name, for example.

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {:.2}", self.owner, self.balance);
    }
}