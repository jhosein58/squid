use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use macroquad::{prelude::*, texture};
use mlua::{FromLua, Lua, Table};
use squid_core::{Event, EventData};
use squid_engine::StreamContext;

macro_rules! lua_fn {
    ($lua:expr, $table:expr, $name:expr, $closure:expr) => {{
        let f = $lua.create_function($closure).unwrap();
        $table.set($name, f).unwrap();
    }};
}

macro_rules! lua_fn_async {
    ($lua:expr, $table:expr, $name:expr, $closure:expr) => {{
        let f = $lua.create_async_function($closure).unwrap();
        $table.set($name, f).unwrap();
    }};
}

pub struct RuntimeApi {
    texture_cache: Arc<Mutex<HashMap<String, Texture2D>>>,
}

impl RuntimeApi {
    pub fn new() -> Self {
        Self {
            texture_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn get_or<T>(tbl: &mlua::Table, val: &str, default: T) -> T
    where
        T: FromLua,
    {
        tbl.get::<T>(val).unwrap_or(default)
    }
    pub fn parse_color_table(color_table: &mlua::Table) -> macroquad::color::Color {
        let r = Self::get_or(color_table, "r", 0.0) / 255.0;
        let g = Self::get_or(color_table, "g", 0.0) / 255.0;
        let b = Self::get_or(color_table, "b", 0.0) / 255.0;
        let a = Self::get_or(color_table, "a", 255.0) / 255.0;
        macroquad::color::Color::new(r, g, b, a)
    }

    pub fn draw_simple_rect(x: f32, y: f32, w: f32, h: f32, color: macroquad::color::Color) {
        draw_rectangle(x, y, w, h, color);
    }
    pub fn draw_rounded_rect(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: macroquad::color::Color,
    ) {
        let rt = render_target(w as u32, h as u32);

        set_camera(&Camera2D {
            target: vec2(w / 2.0, h / 2.0),
            zoom: vec2(2.0 / w, -2.0 / h),
            ..Default::default()
        });
        rt.texture.set_filter(FilterMode::Linear);

        set_camera(&Camera2D {
            target: vec2(w / 2.0, h / 2.0),
            zoom: vec2(2.0 / w, -2.0 / h),
            render_target: Some(rt.clone()),
            ..Default::default()
        });

        clear_background(Color::new(0., 0., 0., 0.));

        draw_rectangle(radius, 0., w - 2.0 * radius, h, WHITE);
        draw_rectangle(0., radius, w, h - 2.0 * radius, WHITE);
        draw_circle(radius, radius, radius, WHITE);
        draw_circle(w - radius, radius, radius, WHITE);
        draw_circle(radius, h - radius, radius, WHITE);
        draw_circle(w - radius, h - radius, radius, WHITE);

        set_default_camera();

        draw_texture_ex(
            &rt.texture,
            x,
            y,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );
    }

    pub fn draw_bordered_rounded_rect(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        border_width: f32,
        border_color: Color,
        color: Color,
    ) {
        if radius > 0. {
            if border_width > 0. && border_color.a != 0. {
                Self::draw_rounded_rect(x, y, w, h, radius, border_color);

                let inner_x = x + border_width;
                let inner_y = y + border_width;
                let inner_w = w - border_width * 2.0;
                let inner_h = h - border_width * 2.0;

                let inner_radius = (radius - border_width).max(0.0);

                Self::draw_rounded_rect(inner_x, inner_y, inner_w, inner_h, inner_radius, color);
            } else {
                Self::draw_rounded_rect(x, y, w, h, radius, color);
            }
        } else {
            if border_width > 0. && border_color.a != 0. {
                Self::draw_simple_rect(x, y, w, h, border_color);

                let inner_x = x + border_width;
                let inner_y = y + border_width;
                let inner_w = w - border_width * 2.0;
                let inner_h = h - border_width * 2.0;

                Self::draw_simple_rect(inner_x, inner_y, inner_w, inner_h, color);
            } else {
                Self::draw_simple_rect(x, y, w, h, color);
            }
        }
    }

    pub fn add_api_to_lua(&mut self, lua: &Lua, app_state: Arc<StreamContext>) {
        let engine = lua.create_table().unwrap();
        lua.globals().set("engine", engine.clone()).unwrap();

        // --- draw_rect ---
        lua_fn!(lua, engine, "draw_rect", |_,
                                           (prop, color): (
            mlua::Table,
            mlua::Table
        )| {
            let x = Self::get_or(&prop, "x", 0.);
            let y = Self::get_or(&prop, "y", 0.);
            let w = Self::get_or(&prop, "width", 0.);
            let h = Self::get_or(&prop, "height", 0.);

            let c = Self::parse_color_table(&color);

            Self::draw_simple_rect(x, y, w, h, c);

            Ok(())
        });

        // --- draw_rounded_rect ---
        lua_fn!(lua, engine, "draw_rounded_rect", |_,
                                                   (prop, color): (
            mlua::Table,
            mlua::Table
        )| {
            let x = Self::get_or(&prop, "x", 0.);
            let y = Self::get_or(&prop, "y", 0.);
            let w = Self::get_or(&prop, "width", 0.);
            let h = Self::get_or(&prop, "height", 0.);
            let radius = Self::get_or(&prop, "radius", 0.);
            let color = Self::parse_color_table(&color);

            Self::draw_rounded_rect(x, y, w, h, radius, color);

            Ok(())
        });

        // --- draw_bordered_rounded_rect ---
        lua_fn!(lua, engine, "draw_bordered_rounded_rect", |_,
                                                            (
            prop,
            color,
            border_color,
        ): (
            mlua::Table,
            mlua::Table,
            mlua::Table
        )| {
            let x = Self::get_or(&prop, "x", 0.);
            let y = Self::get_or(&prop, "y", 0.);
            let w = Self::get_or(&prop, "width", 0.);
            let h = Self::get_or(&prop, "height", 0.);
            let border_width = Self::get_or(&prop, "border_width", 0.);
            let radius = Self::get_or(&prop, "radius", 0.);
            let color = Self::parse_color_table(&color);
            let border_color = Self::parse_color_table(&border_color);

            Self::draw_bordered_rounded_rect(x, y, w, h, radius, border_width, border_color, color);

            Ok(())
        });

        // --- get_width ---
        lua_fn!(lua, engine, "get_screen_width", |_, ()| Ok(screen_width()));

        // --- get_height ---
        lua_fn!(
            lua,
            engine,
            "get_screen_height",
            |_, ()| Ok(screen_height())
        );

        // --- get_mouse_pos ---
        lua_fn!(lua, engine, "get_mouse_pos", |_, ()| {
            let (x, y) = mouse_position();
            Ok((x, y))
        });

        // --- is_mouse_down(btn) ---
        lua_fn!(lua, engine, "is_mouse_down", |_, btn: String| {
            let pressed = match btn.as_str() {
                "left" => is_mouse_button_down(MouseButton::Left),
                "right" => is_mouse_button_down(MouseButton::Right),
                "middle" => is_mouse_button_down(MouseButton::Middle),
                _ => false,
            };
            Ok(pressed)
        });

        // --- draw_circle ---
        lua_fn!(lua, engine, "draw_circle", |_,
                                             (prop, color): (
            mlua::Table,
            mlua::Table
        )| {
            let x = prop.get::<f32>("x").unwrap_or(0.);
            let y = prop.get::<f32>("y").unwrap_or(0.);
            let r = prop.get::<f32>("radius").unwrap_or(0.);

            let red = color.get::<f32>("r").unwrap_or(0.) / 255.0;
            let green = color.get::<f32>("g").unwrap_or(0.) / 255.0;
            let blue = color.get::<f32>("b").unwrap_or(0.) / 255.0;
            let alpha = color.get::<f32>("a").unwrap_or(255.) / 255.0;

            let c = Color::new(red, green, blue, alpha);
            draw_circle(x, y, r, c);

            Ok(())
        });

        // --- draw_line ---
        lua_fn!(lua, engine, "draw_line", |_,
                                           (prop, color): (
            mlua::Table,
            mlua::Table
        )| {
            let x1 = prop.get::<f32>("x1").unwrap_or(0.);
            let y1 = prop.get::<f32>("y1").unwrap_or(0.);
            let x2 = prop.get::<f32>("x2").unwrap_or(0.);
            let y2 = prop.get::<f32>("y2").unwrap_or(0.);
            let thickness = prop.get::<f32>("thickness").unwrap_or(1.);

            let red = color.get::<f32>("r").unwrap_or(0.) / 255.0;
            let green = color.get::<f32>("g").unwrap_or(0.) / 255.0;
            let blue = color.get::<f32>("b").unwrap_or(0.) / 255.0;
            let alpha = color.get::<f32>("a").unwrap_or(255.) / 255.0;

            let c = Color::new(red, green, blue, alpha);
            draw_line(x1, y1, x2, y2, thickness, c);

            Ok(())
        });

        // --- draw_text ---
        lua_fn!(lua, engine, "draw_text", |_,
                                           (text, prop, color): (
            String,
            mlua::Table,
            mlua::Table
        )| {
            let x = prop.get::<f32>("x").unwrap_or(0.);
            let y = prop.get::<f32>("y").unwrap_or(0.);
            let size = prop.get::<f32>("size").unwrap_or(20.);

            let red = color.get::<f32>("r").unwrap_or(255.) / 255.0;
            let green = color.get::<f32>("g").unwrap_or(255.) / 255.0;
            let blue = color.get::<f32>("b").unwrap_or(255.) / 255.0;
            let alpha = color.get::<f32>("a").unwrap_or(255.) / 255.0;

            let c = Color::new(red, green, blue, alpha);
            draw_text(&text, x, y, size, c);

            Ok(())
        });

        // --- is_key_down ---
        lua_fn!(lua, engine, "is_key_down", |_, key: String| {
            use macroquad::input::KeyCode;
            let k = key.to_lowercase();

            let result = match k.to_lowercase().as_str() {
                "a" => is_key_down(KeyCode::A),
                "b" => is_key_down(KeyCode::B),
                "c" => is_key_down(KeyCode::C),
                "d" => is_key_down(KeyCode::D),
                "e" => is_key_down(KeyCode::E),
                "f" => is_key_down(KeyCode::F),
                "g" => is_key_down(KeyCode::G),
                "h" => is_key_down(KeyCode::H),
                "i" => is_key_down(KeyCode::I),
                "j" => is_key_down(KeyCode::J),
                "k" => is_key_down(KeyCode::K),
                "l" => is_key_down(KeyCode::L),
                "m" => is_key_down(KeyCode::M),
                "n" => is_key_down(KeyCode::N),
                "o" => is_key_down(KeyCode::O),
                "p" => is_key_down(KeyCode::P),
                "q" => is_key_down(KeyCode::Q),
                "r" => is_key_down(KeyCode::R),
                "s" => is_key_down(KeyCode::S),
                "t" => is_key_down(KeyCode::T),
                "u" => is_key_down(KeyCode::U),
                "v" => is_key_down(KeyCode::V),
                "w" => is_key_down(KeyCode::W),
                "x" => is_key_down(KeyCode::X),
                "y" => is_key_down(KeyCode::Y),
                "z" => is_key_down(KeyCode::Z),

                "0" => is_key_down(KeyCode::Key0),
                "1" => is_key_down(KeyCode::Key1),
                "2" => is_key_down(KeyCode::Key2),
                "3" => is_key_down(KeyCode::Key3),
                "4" => is_key_down(KeyCode::Key4),
                "5" => is_key_down(KeyCode::Key5),
                "6" => is_key_down(KeyCode::Key6),
                "7" => is_key_down(KeyCode::Key7),
                "8" => is_key_down(KeyCode::Key8),
                "9" => is_key_down(KeyCode::Key9),

                "up" => is_key_down(KeyCode::Up),
                "down" => is_key_down(KeyCode::Down),
                "left" => is_key_down(KeyCode::Left),
                "right" => is_key_down(KeyCode::Right),
                "space" => is_key_down(KeyCode::Space),
                "enter" => is_key_down(KeyCode::Enter),
                "escape" => is_key_down(KeyCode::Escape),
                "tab" => is_key_down(KeyCode::Tab),
                "backspace" => is_key_down(KeyCode::Backspace),
                "delete" => is_key_down(KeyCode::Delete),
                "shift" => is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift),
                "ctrl" => is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl),
                "alt" => is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt),

                "," => is_key_down(KeyCode::Comma),
                "." => is_key_down(KeyCode::Period),
                "/" => is_key_down(KeyCode::Slash),
                ";" => is_key_down(KeyCode::Semicolon),
                "'" => is_key_down(KeyCode::Apostrophe),
                "[" => is_key_down(KeyCode::LeftBracket),
                "]" => is_key_down(KeyCode::RightBracket),
                "-" => is_key_down(KeyCode::Minus),
                "=" => is_key_down(KeyCode::Equal),
                "\\" => is_key_down(KeyCode::Backslash),
                "`" => is_key_down(KeyCode::GraveAccent),

                "f1" => is_key_down(KeyCode::F1),
                "f2" => is_key_down(KeyCode::F2),
                "f3" => is_key_down(KeyCode::F3),
                "f4" => is_key_down(KeyCode::F4),
                "f5" => is_key_down(KeyCode::F5),
                "f6" => is_key_down(KeyCode::F6),
                "f7" => is_key_down(KeyCode::F7),
                "f8" => is_key_down(KeyCode::F8),
                "f9" => is_key_down(KeyCode::F9),
                "f10" => is_key_down(KeyCode::F10),
                "f11" => is_key_down(KeyCode::F11),
                "f12" => is_key_down(KeyCode::F12),

                "kp0" => is_key_down(KeyCode::Kp0),
                "kp1" => is_key_down(KeyCode::Kp1),
                "kp2" => is_key_down(KeyCode::Kp2),
                "kp3" => is_key_down(KeyCode::Kp3),
                "kp4" => is_key_down(KeyCode::Kp4),
                "kp5" => is_key_down(KeyCode::Kp5),
                "kp6" => is_key_down(KeyCode::Kp6),
                "kp7" => is_key_down(KeyCode::Kp7),
                "kp8" => is_key_down(KeyCode::Kp8),
                "kp9" => is_key_down(KeyCode::Kp9),
                "kpadd" => is_key_down(KeyCode::KpAdd),
                "kpsubtract" => is_key_down(KeyCode::KpSubtract),
                "kpmultiply" => is_key_down(KeyCode::KpMultiply),
                "kpdivide" => is_key_down(KeyCode::KpDivide),
                "kpenter" => is_key_down(KeyCode::KpEnter),
                "kpdecimal" => is_key_down(KeyCode::KpDecimal),

                _ => false,
            };

            Ok(result)
        });

        // --- get_pressed_keys ---
        lua_fn!(lua, engine, "get_pressed_keys", |_, ()| {
            use macroquad::input::get_keys_down;
            let keys: Vec<String> = get_keys_down().iter().map(|k| format!("{:?}", k)).collect();
            Ok(keys)
        });

        // --- get_delta_time ---
        lua_fn!(lua, engine, "get_delta_time", |_, ()| {
            Ok(get_frame_time())
        });

        // --- draw_waveform ---
        lua_fn!(lua, engine, "draw_waveform", |_,
                                               (prop, color): (
            mlua::Table,
            mlua::Table
        )| {
            let x = prop.get::<f32>("x").unwrap_or(0.);
            let y = prop.get::<f32>("y").unwrap_or(0.);
            let w = prop.get::<f32>("width").unwrap_or(0.);
            let h = prop.get::<f32>("height").unwrap_or(0.);
            let thickness = prop.get::<f32>("thickness").unwrap_or(1.);

            let data = prop.get::<Vec<f32>>("data")?;

            let red = color.get::<f32>("r").unwrap_or(0.) / 255.0;
            let green = color.get::<f32>("g").unwrap_or(0.) / 255.0;
            let blue = color.get::<f32>("b").unwrap_or(0.) / 255.0;
            let alpha = color.get::<f32>("a").unwrap_or(255.) / 255.0;
            let c = Color::new(red, green, blue, alpha);

            if data.len() < 2 {
                return Ok(());
            }

            let step_x = w / (data.len() - 1) as f32;

            for (i, points) in data.windows(2).enumerate() {
                let p1_val = points[0].clamp(-1.0, 1.0);
                let p2_val = points[1].clamp(-1.0, 1.0);

                let x1 = x + (i as f32 * step_x);
                let y1 = y + ((-p1_val + 1.0) / 2.0) * h;

                let x2 = x + ((i + 1) as f32 * step_x);
                let y2 = y + ((-p2_val + 1.0) / 2.0) * h;

                draw_line(x1, y1, x2, y2, thickness, c);
            }

            Ok(())
        });

        let shared_st = app_state.clone();
        // --- send_note_on_event ---
        lua_fn!(
            lua,
            engine,
            "send_note_on_event",
            move |_, (note): (f32)| {
                let _ = shared_st.events.push(Event {
                    timing: 0,
                    data: EventData::NoteOn {
                        note: note as u8,
                        velocity: 0,
                    },
                });
                Ok(())
            }
        );

        let shared_st = app_state.clone();
        // --- send_note_off_event ---
        lua_fn!(lua, engine, "send_note_off_event", move |_, note: f32| {
            let _ = shared_st.events.push(Event {
                timing: 0,
                data: EventData::NoteOff { note: note as u8 },
            });
            Ok(())
        });

        let texture_cache = self.texture_cache.clone();
        // --- load_texture ---
        lua_fn!(lua, engine, "load_texture", move |_, path: String| {
            let mut texture_cache = texture_cache.lock().unwrap();
            if let None = texture_cache.get(&path) {
                let bytes = std::fs::read(&path).expect("cannot read image file");
                let texture = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
                texture.set_filter(FilterMode::Linear);
                texture_cache.insert(path, texture);
            }
            Ok(())
        });

        let texture_cache = self.texture_cache.clone();
        // --- draw_texture ---
        lua_fn!(lua, engine, "draw_texture", move |_, prop: Table| {
            let texture_cache = texture_cache.lock().unwrap();
            let path = Self::get_or(&prop, "path", String::new());
            let x = Self::get_or(&prop, "x", 0.0);
            let y = Self::get_or(&prop, "y", 0.0);
            let width = Self::get_or(&prop, "width", 0.0);
            let height = Self::get_or(&prop, "height", 0.0);

            if let Some(texture) = texture_cache.get(&path) {
                draw_texture_ex(
                    texture,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(width, height)),
                        ..Default::default()
                    },
                );
            }

            Ok(())
        });
    }
}
