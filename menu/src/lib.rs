#![cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]

mod errors;
mod menu;
mod hud;
mod vertex;
mod scene;
mod world; //need this ?
mod lights;
mod game_shaders;
mod scene_layout;

pub use self::errors::{Error, Result};
pub use self::menu::{create, Menu, MenuConfig};

pub const SHADER_ROOT: &str = "assets/shaders";



 

