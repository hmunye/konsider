mod handlers;
mod login;
mod utils;

pub use handlers::*;
pub use login::*;
pub use utils::*;

// Name for each cookie
pub const AUTH_TOKEN: &str = "auth-token";
