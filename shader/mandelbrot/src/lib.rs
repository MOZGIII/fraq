#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
#![deny(warnings)]

use fraq_shader_shared::ShaderConstants;
#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

#[cfg(not(target_arch = "spirv"))]
pub const SHADER_CODE: &[u8] = include_bytes!(env!("fraq_shader_mandelbrot.spv"));

use spirv_std::glam::{vec2, vec4, Vec2, Vec4};

pub fn uvscale(mut uv: Vec2) -> Vec2 {
    uv = uv * 2.5;
    uv.x = uv.x - 0.5;
    uv
}

pub fn fs(constants: &ShaderConstants, frag_coord: Vec2) -> Vec4 {
    let mut uv = constants.uv(frag_coord);
    uv = constants.adjust_uv_for_position(uv);
    mandelbrot(uv)
}

pub fn mandelbrot(uv: Vec2) -> Vec4 {
    let scaled = uvscale(uv);
    let color = fraq_math_mandelbrot::point(scaled.x, scaled.y);

    match color {
        Some(color) => {
            let color = ((color as f32) / 1000.0).clamp(0.0, 1.0);

            let color = color * 3.0;

            vec4(
                color.clamp(0.0, 1.0),
                (color - 1.0).clamp(0.0, 1.0),
                (color - 2.0).clamp(0.0, 1.0),
                1.0,
            )
        }
        None => vec4(0.0, 0.0, 0.0, 0.0),
    }
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] in_frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
    *output = fs(constants, frag_coord);
}

#[spirv(vertex)]
pub fn main_vs(#[spirv(vertex_index)] vert_idx: i32, #[spirv(position)] builtin_pos: &mut Vec4) {
    // Create a "full screen triangle" by mapping the vertex index.
    // ported from https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
    let uv = vec2(((vert_idx << 1) & 2) as f32, (vert_idx & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *builtin_pos = pos.extend(0.0).extend(1.0);
}
