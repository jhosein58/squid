use midly::MetaMessage;
use mlua::prelude::*;
use std::{
    cmp::min,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use futures::executor::block_on;
use macroquad::{prelude::*, texture};
use mlua::{FromLua, Lua, Table};
use squid_core::{Event, EventData};
use squid_engine::StreamContext;

pub enum ShapeStyle {
    RoundedRect { radius: f32 },
    Capsule,
    Circle,
}

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
    font_cache: Arc<Mutex<HashMap<String, Font>>>,
}

impl RuntimeApi {
    pub fn new() -> Self {
        Self {
            texture_cache: Arc::new(Mutex::new(HashMap::new())),
            font_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn get_or<T>(tbl: &mlua::Table, val: &str, default: T) -> T
    where
        T: FromLua,
    {
        tbl.get::<T>(val).unwrap_or(default)
    }
    pub fn parse_color_table(color_table: &mlua::Table, default_a: f32) -> macroquad::color::Color {
        let r = Self::get_or(color_table, "r", 0.0) / 255.0;
        let g = Self::get_or(color_table, "g", 0.0) / 255.0;
        let b = Self::get_or(color_table, "b", 0.0) / 255.0;
        let a = Self::get_or(color_table, "a", default_a) / 255.0;
        macroquad::color::Color::new(r, g, b, a)
    }

    pub fn draw_simple_rect(x: f32, y: f32, w: f32, h: f32, color: macroquad::color::Color) {
        draw_rectangle(x, y, w, h, color);
    }

    fn draw_internal_fast_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color) {
        if w <= 0.0 || h <= 0.0 {
            return;
        }

        let r = radius.min(w * 0.5).min(h * 0.5);
        if r < 0.1 {
            draw_rectangle(x, y, w, h, color);
            return;
        }

        const CORNER_SEGMENTS: usize = 12;
        const PI: f32 = std::f32::consts::PI;
        const HALF_PI: f32 = std::f32::consts::FRAC_PI_2;

        let mut vertices: Vec<Vertex> = Vec::with_capacity(4 * (CORNER_SEGMENTS + 1) + 1);
        let mut indices: Vec<u16> = Vec::with_capacity(4 * CORNER_SEGMENTS * 3);

        vertices.push(Vertex {
            position: vec3(x + w / 2.0, y + h / 2.0, 0.0),
            uv: vec2(0.0, 0.0),
            color: [color.r as u8, color.g as u8, color.b as u8, color.a as u8],
            normal: vec4(0.0, 0.0, 1.0, 0.0),
        });
        let center_idx = 0u16;

        let mut add_corner = |cx: f32, cy: f32, start_angle: f32| {
            for i in 0..=CORNER_SEGMENTS {
                let t = i as f32 / CORNER_SEGMENTS as f32;
                let ang = start_angle + t * HALF_PI;
                vertices.push(Vertex {
                    position: vec3(cx + ang.cos() * r, cy + ang.sin() * r, 0.0),
                    uv: vec2(0.0, 0.0),
                    color: [color.r as u8, color.g as u8, color.b as u8, color.a as u8],
                    normal: vec4(0.0, 0.0, 1.0, 0.0),
                });
            }
        };

        add_corner(x + r, y + r, PI);
        add_corner(x + w - r, y + r, -HALF_PI);
        add_corner(x + w - r, y + h - r, 0.0);
        add_corner(x + r, y + h - r, HALF_PI);

        let perim_count = vertices.len() as u16 - 1;
        for i in 1..perim_count {
            indices.extend_from_slice(&[center_idx, i, i + 1]);
        }
        indices.extend_from_slice(&[center_idx, perim_count, 1]);

        draw_mesh(&Mesh {
            vertices,
            indices,
            texture: None,
        });
    }

    pub fn draw_rect_advanced(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        border_width: f32,
        color: Color,
        border_color: Color,
    ) {
        if w <= 0.0 || h <= 0.0 {
            return;
        }

        let has_border = border_width > 0.0 && border_color.a > 0.0;
        let has_radius = radius >= 0.1;

        match (has_border, has_radius) {
            (false, false) => {
                if color.a > 0.0 {
                    let c = Color::new(
                        color.r / 255.,
                        color.g / 255.,
                        color.b / 255.,
                        color.a / 255.,
                    );
                    Self::draw_simple_rect(x, y, w, h, c);
                }
            }

            (false, true) => {
                if color.a > 0.0 {
                    Self::draw_internal_fast_rounded_rect(x, y, w, h, radius, color);
                }
            }

            (true, false) => {
                let c = Color::new(
                    color.r / 255.,
                    color.g / 255.,
                    color.b / 255.,
                    color.a / 255.,
                );
                let bc = Color::new(
                    border_color.r / 255.,
                    border_color.g / 255.,
                    border_color.b / 255.,
                    border_color.a / 255.,
                );
                draw_rectangle(x, y, w, h, bc);

                let inner_w = w - border_width * 2.0;
                let inner_h = h - border_width * 2.0;
                if color.a > 0.0 && inner_w > 0.0 && inner_h > 0.0 {
                    draw_rectangle(x + border_width, y + border_width, inner_w, inner_h, c);
                }
            }

            (true, true) => {
                let inner_x = x + border_width;
                let inner_y = y + border_width;
                let inner_w = (w - border_width * 2.0).max(0.0);
                let inner_h = (h - border_width * 2.0).max(0.0);
                let inner_radius = (radius - border_width).max(0.0);

                if color.a > 0.0 && inner_w > 0.0 && inner_h > 0.0 {
                    Self::draw_internal_fast_rounded_rect(
                        inner_x,
                        inner_y,
                        inner_w,
                        inner_h,
                        inner_radius,
                        color,
                    );
                }

                let r_outer = radius.min(w * 0.5).min(h * 0.5);
                let r_inner = (r_outer - border_width).max(0.0);

                const CORNER_SEGMENTS: usize = 12;
                const PI: f32 = std::f32::consts::PI;
                const HALF_PI: f32 = std::f32::consts::FRAC_PI_2;

                let mut vertices: Vec<Vertex> = Vec::with_capacity((CORNER_SEGMENTS + 1) * 4 * 2);
                let mut indices: Vec<u16> =
                    Vec::with_capacity(CORNER_SEGMENTS * 4 * 2 * 3 + 4 * 2 * 3);

                let border_color_u8 = [
                    border_color.r as u8,
                    border_color.g as u8,
                    border_color.b as u8,
                    border_color.a as u8,
                ];
                let normal_v4 = vec4(0.0, 0.0, 1.0, 0.0);

                let corners_outer = [
                    (x + r_outer, y + r_outer, PI),
                    (x + w - r_outer, y + r_outer, -HALF_PI),
                    (x + w - r_outer, y + h - r_outer, 0.0),
                    (x + r_outer, y + h - r_outer, HALF_PI),
                ];

                let corners_inner = [
                    (inner_x + r_inner, inner_y + r_inner, PI),
                    (inner_x + inner_w - r_inner, inner_y + r_inner, -HALF_PI),
                    (
                        inner_x + inner_w - r_inner,
                        inner_y + inner_h - r_inner,
                        0.0,
                    ),
                    (inner_x + r_inner, inner_y + inner_h - r_inner, HALF_PI),
                ];

                for i in 0..4 {
                    let (cx_out, cy_out, start_angle) = corners_outer[i];
                    let (cx_in, cy_in, _) = corners_inner[i];
                    for j in 0..=CORNER_SEGMENTS {
                        let t = j as f32 / CORNER_SEGMENTS as f32;
                        let ang = start_angle + t * HALF_PI;
                        let cos_a = ang.cos();
                        let sin_a = ang.sin();
                        vertices.push(Vertex {
                            position: vec3(cx_out + cos_a * r_outer, cy_out + sin_a * r_outer, 0.0),
                            uv: vec2(0.0, 0.0),
                            color: border_color_u8,
                            normal: normal_v4,
                        });
                        vertices.push(Vertex {
                            position: vec3(cx_in + cos_a * r_inner, cy_in + sin_a * r_inner, 0.0),
                            uv: vec2(0.0, 0.0),
                            color: border_color_u8,
                            normal: normal_v4,
                        });
                    }
                }

                let verts_per_corner = (CORNER_SEGMENTS + 1) * 2;
                for c in 0..4 {
                    let base_idx = (c * verts_per_corner) as u16;
                    for i in 0..(CORNER_SEGMENTS as u16) {
                        let current = base_idx + i * 2;
                        indices.extend_from_slice(&[current, current + 1, current + 2]);
                        indices.extend_from_slice(&[current + 1, current + 3, current + 2]);
                    }
                    let next_c = (c + 1) % 4;
                    let current_end_outer = base_idx + (verts_per_corner as u16) - 2;
                    let current_end_inner = base_idx + (verts_per_corner as u16) - 1;
                    let next_start_outer = (next_c * verts_per_corner) as u16;
                    let next_start_inner = next_start_outer + 1;
                    indices.extend_from_slice(&[
                        current_end_outer,
                        current_end_inner,
                        next_start_outer,
                    ]);
                    indices.extend_from_slice(&[
                        current_end_inner,
                        next_start_inner,
                        next_start_outer,
                    ]);
                }
                draw_mesh(&Mesh {
                    vertices,
                    indices,
                    texture: None,
                });
            }
        }
    }

    pub fn add_api_to_lua(&mut self, lua: &Lua, app_state: Arc<StreamContext>) {
        let engine = lua.create_table().unwrap();
        lua.globals().set("engine", engine.clone()).unwrap();

        // --- draw_rect ---
        lua_fn!(lua, engine, "draw_rect", move |_,
                                                (
            x,
            y,
            w,
            h,
            r,
            bw,
            c_r,
            c_g,
            c_b,
            c_a,
            bc_r,
            bc_g,
            bc_b,
            bc_a,
            shape,
        ): (
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            String
        )| {
            let color = Color::new(c_r, c_g, c_b, c_a);
            let border_color = Color::new(bc_r, bc_g, bc_b, bc_a);

            if shape == "circle" || shape == "capsule" || shape == "c" {
                Self::draw_rect_advanced(x, y, w, h, (w / 2.).min(h / 2.), bw, color, border_color)
            } else {
                Self::draw_rect_advanced(x, y, w, h, r, bw, color, border_color)
            }

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

        let font_cache = self.font_cache.clone();
        // --- load_font ---
        lua_fn!(lua, engine, "load_font", move |_, path: String| {
            let mut font_cache = font_cache.lock().unwrap();

            if let None = font_cache.get(&path) {
                let font = block_on(async { load_ttf_font(&path).await.unwrap() });
                font_cache.insert(path, font);
            }
            Ok(())
        });

        let font_cache = self.font_cache.clone();
        // --- measure_text ---
        lua_fn!(lua, engine, "measure_text", move |lua,
                                                   (text, size, font): (
            String,
            f32,
            String
        )| {
            let font_cache = font_cache.lock().unwrap();
            let font = font_cache.get(&font);

            let dim = measure_text(&text, font, size as u16, 1.0);

            let result = lua.create_table()?;
            result.set("width", dim.width)?;
            result.set("height", dim.height)?;
            result.set("offset_y", dim.offset_y)?;
            Ok(result)
        });

        let font_cache = self.font_cache.clone();
        // --- draw_text ---
        lua_fn!(lua, engine, "draw_text", move |_,
                                                (
            text,
            font,
            x,
            y,
            s,
            c_r,
            c_g,
            c_b,
            c_a,
        ): (
            String,
            String,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
        )| {
            let c = Color::new(c_r / 255.0, c_g / 255.0, c_b / 255.0, c_a / 255.0);

            let font_cache = font_cache.lock().unwrap();
            draw_text_ex(
                &text,
                x,
                y,
                TextParams {
                    font: font_cache.get(&font),
                    font_size: s as u16,
                    color: c,
                    ..Default::default()
                },
            );

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

        use midly::{Smf, TrackEventKind};
        // --- read_midi_file ---

        lua_fn!(lua, engine, "read_midi_file", move |lua, file: String| {
            let bytes = std::fs::read(&file).map_err(|e| LuaError::external(e))?;
            let smf = Smf::parse(&bytes).map_err(|e| LuaError::external(e))?;

            let ppq = match smf.header.timing {
                midly::Timing::Metrical(t) => t.as_int() as f32,
                _ => 480.0,
            };

            // همه رویدادها در یک لیست
            let mut all_events = Vec::new();

            for track in &smf.tracks {
                let mut abs_tick: u32 = 0;
                for event in track {
                    abs_tick += event.delta.as_int();
                    match event.kind {
                        TrackEventKind::Midi {
                            channel: _,
                            message,
                        } => match message {
                            midly::MidiMessage::NoteOn { key, vel } => {
                                let kind = if vel > 0 { "on" } else { "off" };
                                all_events.push((
                                    abs_tick,
                                    kind.to_string(),
                                    key.as_int(),
                                    vel.as_int(),
                                ));
                            }
                            midly::MidiMessage::NoteOff { key, .. } => {
                                all_events.push((abs_tick, "off".to_string(), key.as_int(), 0));
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }

            // مرتب‌سازی بر اساس زمان
            all_events.sort_by_key(|e| e.0);

            // تبدیل به جدول Lua
            let res = lua.create_table()?;
            let mut idx = 1;
            for (tick, kind, note, vel) in all_events {
                let ev = lua.create_table()?;
                ev.set("time_beats", tick as f32 / ppq)?;
                ev.set("kind", kind.clone())?;
                ev.set("note", note)?;
                if kind == "on" {
                    ev.set("velocity", vel)?;
                }
                res.set(idx, ev)?;
                idx += 1;
            }

            Ok(res)
        });
    }
}
