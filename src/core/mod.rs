mod consts;
mod remote;
mod types;
mod utils;
mod web;

pub use consts::*;
pub use remote::list_remote;
pub use types::*;
pub use utils::*;
pub use web::{get_index, install};
