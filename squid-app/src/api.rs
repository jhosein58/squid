use macroquad::prelude::*;
use mlua::Lua;
pub struct UiApi;

impl UiApi {
    pub fn add_api_to_lua(lua: &Lua) {
        // --- draw_rect ---
        let draw_rect = lua
            .create_function(|_, (prop, color): (mlua::Table, mlua::Table)| {
                let x = prop.get::<f32>("x").unwrap_or(0.);
                let y = prop.get::<f32>("y").unwrap_or(0.);
                let w = prop.get::<f32>("width").unwrap_or(0.);
                let h = prop.get::<f32>("height").unwrap_or(0.);

                let r = color.get::<f32>("r").unwrap_or(0.) / 255.0;
                let g = color.get::<f32>("g").unwrap_or(0.) / 255.0;
                let b = color.get::<f32>("b").unwrap_or(0.) / 255.0;
                let a = color.get::<f32>("a").unwrap_or(255.) / 255.0;

                let c = Color::new(r, g, b, a);
                draw_rectangle(x, y, w, h, c);

                Ok(())
            })
            .unwrap();
        lua.globals().set("draw_rect", draw_rect).unwrap();

        // --- get_width ---
        let get_screen_width = lua.create_function(|_, ()| Ok(screen_width())).unwrap();
        lua.globals()
            .set("get_screen_width", get_screen_width)
            .unwrap();

        // --- get_height ---
        let get_screen_height = lua.create_function(|_, ()| Ok(screen_height())).unwrap();
        lua.globals()
            .set("get_screen_height", get_screen_height)
            .unwrap();

        // --- get_mouse_pos ---
        let get_mouse_pos = lua
            .create_function(|_, ()| {
                let (x, y) = mouse_position();
                Ok((x, y))
            })
            .unwrap();
        lua.globals().set("get_mouse_pos", get_mouse_pos).unwrap();

        // --- is_mouse_down(btn) ---
        let is_mouse_down = lua
            .create_function(|_, btn: u8| {
                let pressed = match btn {
                    0 => is_mouse_button_down(MouseButton::Left),
                    1 => is_mouse_button_down(MouseButton::Right),
                    2 => is_mouse_button_down(MouseButton::Middle),
                    _ => false,
                };
                Ok(pressed)
            })
            .unwrap();
        lua.globals().set("is_mouse_down", is_mouse_down).unwrap();
    }
}
