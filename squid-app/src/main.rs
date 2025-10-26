use std::fs;

use macroquad::prelude::*;
use mlua::{Function, Lua, Result};
use squid_app::api::UiApi;

#[macroquad::main("Lua + Macroquad Demo")]
async fn main() -> Result<()> {
    let lua = Lua::new();
    UiApi::add_api_to_lua(&lua);

    let code = fs::read_to_string("gui/main.lua")?;
    lua.load(code).exec()?;
    loop {
        clear_background(WHITE);

        if let Ok(update) = lua.globals().get::<Function>("update") {
            update.call::<()>(())?;
        }
        next_frame().await;
    }
}
