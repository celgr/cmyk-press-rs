use crate::{EDGE_CLAMP, EDGE_MIRROR};

#[derive(Clone)]
pub(crate) struct Frame {
    pub(crate) pixels: Vec<Rgba>,
    pub(crate) w: usize,
    pub(crate) h: usize,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Rgba {
    pub(crate) rgb: [f32; 3],
    pub(crate) a: f32,
}

impl Rgba {
    pub(crate) fn transparent() -> Self {
        Self {
            rgb: [0.0, 0.0, 0.0],
            a: 0.0,
        }
    }
}

pub(crate) fn rgba_from_straight(red: f32, green: f32, blue: f32, alpha: f32) -> Rgba {
    let a = alpha.clamp(0.0, 1.0);
    Rgba {
        rgb: [
            red.clamp(0.0, 1.0) * a,
            green.clamp(0.0, 1.0) * a,
            blue.clamp(0.0, 1.0) * a,
        ],
        a,
    }
}

pub(crate) fn unpremultiply(px: Rgba) -> Rgba {
    if px.a <= 0.0001 {
        return Rgba::transparent();
    }
    Rgba {
        rgb: straight_rgb_from_rgba(px),
        a: px.a,
    }
}

pub(crate) fn straight_rgb_from_rgba(px: Rgba) -> [f32; 3] {
    if px.a <= 0.0001 {
        return [0.0, 0.0, 0.0];
    }
    [
        (px.rgb[0] / px.a).clamp(0.0, 1.0),
        (px.rgb[1] / px.a).clamp(0.0, 1.0),
        (px.rgb[2] / px.a).clamp(0.0, 1.0),
    ]
}

#[cfg(test)]
pub(crate) fn sample_pixel(src: &Frame, x: usize, y: usize) -> Rgba {
    if src.w == 0 || src.h == 0 {
        return Rgba::transparent();
    }
    src.pixels[(y.min(src.h - 1) * src.w) + x.min(src.w - 1)]
}

#[inline]
pub(crate) fn sample_pixel_in_bounds(src: &Frame, x: usize, y: usize) -> Rgba {
    src.pixels[y * src.w + x]
}

pub(crate) fn sample_bilinear(src: &Frame, x: f32, y: f32, edge_mode: i32) -> Rgba {
    let w = src.w;
    let h = src.h;
    if w == 0 || h == 0 {
        return Rgba::transparent();
    }
    let Some((x, y)) = edge_sample_position(x, y, w, h, edge_mode) else {
        return Rgba::transparent();
    };

    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(w - 1);
    let y1 = (y0 + 1).min(h - 1);
    let tx = x - x0 as f32;
    let ty = y - y0 as f32;

    let a = sample_pixel_in_bounds(src, x0, y0);
    let b = sample_pixel_in_bounds(src, x1, y0);
    let c = sample_pixel_in_bounds(src, x0, y1);
    let d = sample_pixel_in_bounds(src, x1, y1);
    let top = mix_rgba(a, b, tx);
    let bottom = mix_rgba(c, d, tx);
    mix_rgba(top, bottom, ty)
}

pub(crate) fn sample_nearest(src: &Frame, x: f32, y: f32, edge_mode: i32) -> Rgba {
    let w = src.w;
    let h = src.h;
    if w == 0 || h == 0 {
        return Rgba::transparent();
    }
    let Some((x, y)) = edge_sample_position(x, y, w, h, edge_mode) else {
        return Rgba::transparent();
    };
    sample_pixel_in_bounds(src, x.round() as usize, y.round() as usize)
}

fn edge_sample_position(
    x: f32,
    y: f32,
    width: usize,
    height: usize,
    edge_mode: i32,
) -> Option<(f32, f32)> {
    match edge_mode {
        EDGE_CLAMP => Some((
            x.clamp(0.0, (width - 1) as f32),
            y.clamp(0.0, (height - 1) as f32),
        )),
        EDGE_MIRROR => Some((mirror_coordinate(x, width), mirror_coordinate(y, height))),
        _ if x < 0.0 || y < 0.0 || x > (width - 1) as f32 || y > (height - 1) as f32 => None,
        _ => Some((x, y)),
    }
}

fn mirror_coordinate(value: f32, len: usize) -> f32 {
    if len <= 1 {
        return 0.0;
    }
    let max = (len - 1) as f32;
    let period = max * 2.0;
    let wrapped = value.rem_euclid(period);
    if wrapped > max {
        period - wrapped
    } else {
        wrapped
    }
}

pub(crate) fn mix_rgba(a: Rgba, b: Rgba, t: f32) -> Rgba {
    Rgba {
        rgb: mix_rgb(a.rgb, b.rgb, t),
        a: a.a + (b.a - a.a) * t.clamp(0.0, 1.0),
    }
}

pub(crate) fn mix_rgb(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

pub(crate) fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub(crate) fn hash_u32(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^= value >> 16;
    value
}

pub(crate) fn random_signed(seed: u32, plate_id: u32, axis_id: u32) -> f32 {
    let h = hash_u32(seed ^ plate_id.wrapping_mul(31) ^ axis_id);
    let normalized = h as f32 / u32::MAX as f32;
    normalized * 2.0 - 1.0
}

pub(crate) fn to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn straight_pixels_are_premultiplied_for_internal_sampling() {
        let px = rgba_from_straight(1.0, 0.0, 0.0, 0.25);
        assert_eq!(px.rgb, [0.25, 0.0, 0.0]);
        assert_eq!(px.a, 0.25);
    }

    #[test]
    fn internal_premultiplied_pixels_are_unpremultiplied_for_output() {
        let rgb = straight_rgb_from_rgba(Rgba {
            rgb: [0.25, 0.0, 0.0],
            a: 0.25,
        });
        assert_eq!(rgb, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn byte_conversion_clamps_to_u8_range() {
        assert_eq!(to_u8(-1.0), 0);
        assert_eq!(to_u8(2.0), 255);
    }
}
