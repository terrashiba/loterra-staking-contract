pub mod contract;
pub mod state;

mod global;
mod math;
mod taxation;
mod user;

pub mod claim;
pub mod msg;
#[cfg(test)]
mod testing;
#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points_with_migration!(contract);