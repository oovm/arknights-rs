#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://img.nga.178.com/attachments/mon_202008/19/-klbw3Q5-e364K1iT1kSe8-e8.png")]
#![doc(html_favicon_url = "https://img.nga.178.com/attachments/mon_202008/19/-klbw3Q5-e364K1iT1kSe8-e8.png")]

mod cost;
mod doctor;
mod elite;
mod errors;
mod specialization;

pub use self::{
    cost::{LevelUpCost, LevelUpCostDB},
    doctor::DoctorLevelUpDB,
    elite::EliteCost,
    errors::{Error, Result},
};
