mod balances;
mod system;

mod types {
    pub type AccountID = String;
    pub type Balance = u128;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet<types::AccountID, types::Balance>,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    // initialize some accounts
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    // start emulating a block
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    runtime.system.inc_nonce(&alice);
    let _ = runtime
        .balances
        .transfer(&alice, &bob, 30)
        .map_err(|e| println!("Error: {:?}", e));

    // second transaction
    runtime.system.inc_nonce(&alice);
    let _ = runtime
        .balances
        .transfer(&alice, &charlie, 20)
        .map_err(|e| println!("Error: {:?}", e));

    println!("{:#?}", runtime);
}
