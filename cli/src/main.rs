mod utils;
pub use utils::*;

mod networks;
pub use networks::*;

mod user_args;
pub use user_args::*;

mod configuration;
pub use configuration::*;

fn main() {
    let user_args = get_args();

    GeyserConfig::new(user_args).init().start_validator();
}
