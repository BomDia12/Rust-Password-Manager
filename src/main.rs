pub mod interfaces;
pub mod types;
pub mod encryption;
pub mod persistency;
pub mod suggest_password;

use interfaces::cli;
use types::Entry;

fn main() {
    let master_password = "Test Password";
    let mut data: Vec<Entry> = Vec::new();

    cli::login_menu(&master_password, &mut data);
}
