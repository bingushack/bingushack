pub mod module;
pub mod setting;

mod client;
mod mapping;

pub use self::{
    client::{BoxedBingusModule, Client},
    setting::BingusSettings,
};
