//! Ported to Rust from <https://github.com/Tw1ddle/Sky-Shader/blob/master/src/shaders/glsl/sky.fragment>

#![cfg_attr(target_arch = "spirv", no_std, feature(lang_items))]

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::{vec2, Vec2};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,

    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl ShaderConstants {
    pub fn uv(&self, frag_coord: Vec2) -> Vec2 {
        let screen = vec2(self.width as f32, self.height as f32);
        let mut uv = (frag_coord - 0.5 * screen) / screen.y;
        uv.y = -uv.y;
        uv
    }

    pub fn adjust_uv_for_position(&self, mut uv: Vec2) -> Vec2 {
        uv *= self.zoom;
        uv += vec2(self.x, self.y);
        uv
    }
}
