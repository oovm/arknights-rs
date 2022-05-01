#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/arknights-rs/dev/.github/logo/rhodes.png")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/arknights-rs/dev/.github/logo/rhodes.png")]

mod cost;
mod doctor;
mod elite;
mod errors;
mod specialization;

pub use self::{
    cost::{LevelUpCost, LevelUpCostDB},
    doctor::DoctorLevelDB,
    elite::EliteCost,
    errors::{Error, Result},
};
