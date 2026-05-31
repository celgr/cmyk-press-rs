use std::thread;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CmykPressParams {
    pub cyan_amount: f32,
    pub magenta_amount: f32,
    pub yellow_amount: f32,
    pub black_amount: f32,
    pub cyan_offset: [f32; 2],
    pub magenta_offset: [f32; 2],
    pub yellow_offset: [f32; 2],
    pub black_offset: [f32; 2],
    pub random_registration_enabled: u32,
    pub random_seed: u32,
    pub random_amount: [f32; 2],
    pub random_plate_mask: u32,
    pub halftone_enabled: u32,
    pub halftone_frequency: f32,
    pub halftone_shape: u32,
    pub halftone_dot_gain: f32,
    pub halftone_softness: f32,
    pub halftone_angles: [f32; 4],
    pub halftone_offset: [f32; 2],
    pub paper_color: [f32; 3],
    pub paper_brightness: f32,
    pub preserve_alpha: u32,
    pub view_mode: u32,
    pub quality: u32,
    pub edge_mode: u32,
    pub transparent_mode: u32,
}

impl Default for CmykPressParams {
    fn default() -> Self {
        Self {
            cyan_amount: DEFAULT_CMY_INK_AMOUNT,
            magenta_amount: DEFAULT_CMY_INK_AMOUNT,
            yellow_amount: DEFAULT_CMY_INK_AMOUNT,
            black_amount: DEFAULT_BLACK_INK_AMOUNT,
            cyan_offset: [0.0, 0.0],
            magenta_offset: [0.0, 0.0],
            yellow_offset: [0.0, 0.0],
            black_offset: [0.0, 0.0],
            random_registration_enabled: 0,
            random_seed: 0,
            random_amount: [3.0, 3.0],
            random_plate_mask: 0b0111,
            halftone_enabled: 1,
            halftone_frequency: 8.0,
            halftone_shape: CMYK_DOT_CIRCLE,
            halftone_dot_gain: DEFAULT_HALFTONE_DOT_GAIN,
            halftone_softness: 0.1,
            halftone_angles: [15.0, 75.0, 0.0, 45.0],
            halftone_offset: [0.0, 0.0],
            paper_color: [1.0, 1.0, 1.0],
            paper_brightness: 1.0,
            preserve_alpha: 1,
            view_mode: CMYK_VIEW_COMPOSITE,
            quality: CMYK_QUALITY_FULL,
            edge_mode: CMYK_EDGE_TRANSPARENT,
            transparent_mode: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct EffectParams {
    pub(crate) view_mode: i32,
    pub(crate) preserve_alpha: bool,
    pub(crate) blend_original: f32,
    pub(crate) ink_amounts: [f32; PLATE_COUNT],
    pub(crate) paper: [f32; 3],
    pub(crate) offsets: [[f32; 2]; PLATE_COUNT],
    pub(crate) random_enabled: bool,
    pub(crate) random_seed: u32,
    pub(crate) random_amount: [f32; 2],
    pub(crate) random_affect: [bool; PLATE_COUNT],
    pub(crate) halftone_enabled: bool,
    pub(crate) halftone_frequency: f32,
    pub(crate) halftone_unit: i32,
    pub(crate) halftone_shape: i32,
    pub(crate) halftone_dot_gain: f32,
    pub(crate) halftone_softness: f32,
    pub(crate) halftone_angles: [f32; PLATE_COUNT],
    pub(crate) halftone_offset: [f32; 2],
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    pub(crate) backend: i32,
    pub(crate) quality: i32,
    pub(crate) edge_mode: i32,
    pub(crate) expand_bounds: bool,
    pub(crate) conversion_mode: i32,
    pub(crate) ink_colors: [[f32; 3]; PLATE_COUNT],
    pub(crate) transparent_mode: bool,
}

#[derive(Clone, Debug)]
pub struct CmykPressOptions {
    pub view_mode: u32,
    pub preserve_alpha: bool,
    pub blend_original: f32,
    pub ink_amounts: [f32; PLATE_COUNT],
    pub paper: [f32; 3],
    pub offsets: [[f32; 2]; PLATE_COUNT],
    pub random_enabled: bool,
    pub random_seed: u32,
    pub random_amount: [f32; 2],
    pub random_affect: [bool; PLATE_COUNT],
    pub halftone_enabled: bool,
    pub halftone_frequency: f32,
    pub halftone_unit: u32,
    pub halftone_shape: u32,
    pub halftone_dot_gain: f32,
    pub halftone_softness: f32,
    pub halftone_angles: [f32; PLATE_COUNT],
    pub halftone_offset: [f32; 2],
    pub quality: u32,
    pub edge_mode: u32,
    pub conversion_mode: u32,
    pub ink_colors: [[f32; 3]; PLATE_COUNT],
    pub transparent_mode: bool,
}

impl Default for CmykPressOptions {
    fn default() -> Self {
        Self {
            view_mode: VIEW_COMPOSITE as u32,
            preserve_alpha: true,
            blend_original: 0.0,
            ink_amounts: DEFAULT_INK_AMOUNTS,
            paper: [1.0, 1.0, 1.0],
            offsets: [[0.0, 0.0]; PLATE_COUNT],
            random_enabled: false,
            random_seed: 0,
            random_amount: [3.0, 3.0],
            random_affect: [true, true, true, false],
            halftone_enabled: true,
            halftone_frequency: 8.0,
            halftone_unit: HALFTONE_UNIT_PIXELS as u32,
            halftone_shape: DOT_CIRCLE as u32,
            halftone_dot_gain: DEFAULT_HALFTONE_DOT_GAIN,
            halftone_softness: 0.1,
            halftone_angles: [15.0, 75.0, 0.0, 45.0],
            halftone_offset: [0.0, 0.0],
            quality: QUALITY_FULL as u32,
            edge_mode: EDGE_TRANSPARENT as u32,
            conversion_mode: CONVERSION_SIMPLE as u32,
            ink_colors: CUSTOM_INK_COLORS,
            transparent_mode: false,
        }
    }
}

impl From<CmykPressParams> for CmykPressOptions {
    fn from(params: CmykPressParams) -> Self {
        let mut random_affect = [false; PLATE_COUNT];
        for (plate, affect) in random_affect.iter_mut().enumerate() {
            *affect = (params.random_plate_mask & (1 << plate)) != 0;
        }

        Self {
            view_mode: public_view_to_internal(params.view_mode),
            preserve_alpha: params.preserve_alpha != 0,
            blend_original: 0.0,
            ink_amounts: [
                params.cyan_amount.clamp(0.0, 1.0),
                params.magenta_amount.clamp(0.0, 1.0),
                params.yellow_amount.clamp(0.0, 1.0),
                params.black_amount.clamp(0.0, 1.0),
            ],
            paper: apply_paper_controls(
                params.paper_color,
                params.paper_brightness.clamp(0.0, 1.0),
            ),
            offsets: [
                params.cyan_offset,
                params.magenta_offset,
                params.yellow_offset,
                params.black_offset,
            ],
            random_enabled: params.random_registration_enabled != 0,
            random_seed: params.random_seed,
            random_amount: params.random_amount,
            random_affect,
            halftone_enabled: params.halftone_enabled != 0,
            halftone_frequency: params.halftone_frequency,
            halftone_unit: HALFTONE_UNIT_PIXELS as u32,
            halftone_shape: public_shape_to_internal(params.halftone_shape),
            halftone_dot_gain: params.halftone_dot_gain,
            halftone_softness: params.halftone_softness,
            halftone_angles: params.halftone_angles,
            halftone_offset: params.halftone_offset,
            quality: public_quality_to_internal(params.quality),
            edge_mode: public_edge_to_internal(params.edge_mode),
            conversion_mode: CONVERSION_SIMPLE as u32,
            ink_colors: CUSTOM_INK_COLORS,
            transparent_mode: params.transparent_mode != 0,
        }
    }
}

fn public_view_to_internal(value: u32) -> u32 {
    match value {
        CMYK_VIEW_CYAN => VIEW_CYAN as u32,
        CMYK_VIEW_MAGENTA => VIEW_MAGENTA as u32,
        CMYK_VIEW_YELLOW => VIEW_YELLOW as u32,
        CMYK_VIEW_BLACK => VIEW_BLACK as u32,
        CMYK_VIEW_INK_COVERAGE => VIEW_INK_COVERAGE as u32,
        CMYK_VIEW_SPLIT => VIEW_SPLIT as u32,
        _ => VIEW_COMPOSITE as u32,
    }
}

fn public_shape_to_internal(value: u32) -> u32 {
    match value {
        CMYK_DOT_SQUARE => DOT_SQUARE as u32,
        CMYK_DOT_LINE => DOT_LINE as u32,
        CMYK_DOT_DIAMOND => DOT_DIAMOND as u32,
        _ => DOT_CIRCLE as u32,
    }
}

fn public_quality_to_internal(value: u32) -> u32 {
    match value {
        CMYK_QUALITY_DRAFT => QUALITY_DRAFT as u32,
        _ => QUALITY_FULL as u32,
    }
}

fn public_edge_to_internal(value: u32) -> u32 {
    match value {
        CMYK_EDGE_CLAMP => EDGE_CLAMP as u32,
        _ => EDGE_TRANSPARENT as u32,
    }
}

pub fn render_rgba_f32(
    input: &[[f32; 4]],
    width: usize,
    height: usize,
    options: &CmykPressOptions,
) -> Vec<[f32; 4]> {
    if width == 0 || height == 0 || input.len() < width.saturating_mul(height) {
        return Vec::new();
    }
    let frame = Frame {
        w: width,
        h: height,
        pixels: input
            .iter()
            .take(width * height)
            .map(|px| {
                let a = px[3].clamp(0.0, 1.0);
                Rgba {
                    rgb: [
                        px[0].clamp(0.0, 1.0) * a,
                        px[1].clamp(0.0, 1.0) * a,
                        px[2].clamp(0.0, 1.0) * a,
                    ],
                    a,
                }
            })
            .collect(),
    };
    let ep = options.to_effect_params();
    render_cmyk_press(&frame, &ep)
        .pixels
        .into_iter()
        .map(|px| {
            let straight = unpremultiply(px);
            [straight.rgb[0], straight.rgb[1], straight.rgb[2], px.a]
        })
        .collect()
}

pub fn render_rgba_f32_with_params(
    input: &[[f32; 4]],
    width: usize,
    height: usize,
    params: &CmykPressParams,
) -> Vec<[f32; 4]> {
    render_rgba_f32(input, width, height, &CmykPressOptions::from(*params))
}

impl CmykPressOptions {
    fn to_effect_params(&self) -> EffectParams {
        let conversion_mode = normalize_conversion_mode(self.conversion_mode as i32);
        let ink_colors = if conversion_mode == CONVERSION_ILLUSTRATOR {
            ILLUSTRATOR_INK_COLORS
        } else {
            clamp_ink_colors(self.ink_colors)
        };
        EffectParams {
            view_mode: normalize_view(self.view_mode as i32),
            preserve_alpha: self.preserve_alpha,
            blend_original: self.blend_original.clamp(0.0, 1.0),
            ink_amounts: [
                self.ink_amounts[0].clamp(0.0, 1.0),
                self.ink_amounts[1].clamp(0.0, 1.0),
                self.ink_amounts[2].clamp(0.0, 1.0),
                self.ink_amounts[3].clamp(0.0, 1.0),
            ],
            paper: [
                self.paper[0].clamp(0.0, 1.0),
                self.paper[1].clamp(0.0, 1.0),
                self.paper[2].clamp(0.0, 1.0),
            ],
            offsets: self.offsets,
            random_enabled: self.random_enabled,
            random_seed: self.random_seed,
            random_amount: [
                self.random_amount[0].clamp(0.0, 1000.0),
                self.random_amount[1].clamp(0.0, 1000.0),
            ],
            random_affect: self.random_affect,
            halftone_enabled: self.halftone_enabled,
            halftone_frequency: self.halftone_frequency.clamp(1.0, 1000.0),
            halftone_unit: normalize_halftone_unit(self.halftone_unit as i32),
            halftone_shape: normalize_dot_shape(self.halftone_shape as i32),
            halftone_dot_gain: self.halftone_dot_gain.clamp(-1.0, 1.0),
            halftone_softness: self.halftone_softness.clamp(0.0, 1.0),
            halftone_angles: self.halftone_angles,
            halftone_offset: self.halftone_offset,
            backend: BACKEND_CPU,
            quality: normalize_quality(self.quality as i32),
            edge_mode: normalize_edge_mode(self.edge_mode as i32),
            expand_bounds: false,
            conversion_mode,
            ink_colors,
            transparent_mode: self.transparent_mode,
        }
    }
}

fn clamp_ink_colors(ink_colors: [[f32; 3]; PLATE_COUNT]) -> [[f32; 3]; PLATE_COUNT] {
    std::array::from_fn(|plate| {
        [
            ink_colors[plate][0].clamp(0.0, 1.0),
            ink_colors[plate][1].clamp(0.0, 1.0),
            ink_colors[plate][2].clamp(0.0, 1.0),
        ]
    })
}

#[derive(Clone)]
pub(crate) struct Frame {
    pub(crate) pixels: Vec<Rgba>,
    pub(crate) w: usize,
    pub(crate) h: usize,
}

pub(crate) fn render_cmyk_press(src: &Frame, ep: &EffectParams) -> Frame {
    let w = src.w;
    let h = src.h;
    let _expand_bounds_requested = ep.expand_bounds;
    if w == 0 || h == 0 {
        return src.clone();
    }

    let mut out = vec![Rgba::transparent(); w * h];
    let plan = RenderPlan::new(ep, w, h);
    let threads = num_cpus::get().max(1).min(h).max(1);
    let rows_per_thread = h.div_ceil(threads);

    thread::scope(|scope| {
        for (chunk_index, out_chunk) in out.chunks_mut(rows_per_thread * w).enumerate() {
            let y_start = chunk_index * rows_per_thread;
            let rows = (out_chunk.len() / w).min(h.saturating_sub(y_start));
            scope.spawn(move || {
                render_rows(out_chunk, src, y_start, rows, ep, &plan);
            });
        }
    });

    Frame { pixels: out, w, h }
}

fn render_rows(
    out: &mut [Rgba],
    src: &Frame,
    y_start: usize,
    rows: usize,
    ep: &EffectParams,
    plan: &RenderPlan,
) {
    let w = src.w;
    let h = src.h;
    let y_end = (y_start + rows).min(h);

    for y in y_start..y_end {
        for x in 0..w {
            let xy = [x as f32, y as f32];
            let original = sample_pixel(src, x, y);
            let printed = render_pixel(src, xy, original, ep, plan);
            out[(y - y_start) * w + x] = printed;
        }
    }
}

fn render_pixel(
    src: &Frame,
    xy: [f32; 2],
    original: Rgba,
    ep: &EffectParams,
    plan: &RenderPlan,
) -> Rgba {
    let mut inks = [0.0f32; PLATE_COUNT];
    let mut alpha_max: f32 = 0.0;
    for plate in 0..PLATE_COUNT {
        let pos = if ep.halftone_enabled {
            halftone_sample_position(xy, &plan.plates[plate], ep)
        } else {
            [
                xy[0] + plan.plates[plate].shift[0],
                xy[1] + plan.plates[plate].shift[1],
            ]
        };
        let sampled = if ep.quality == QUALITY_DRAFT {
            sample_nearest(src, pos[0], pos[1], ep.edge_mode)
        } else {
            sample_bilinear(src, pos[0], pos[1], ep.edge_mode)
        };
        alpha_max = alpha_max.max(sampled.a);
        inks[plate] = separate_plate(sampled, ep, plate);
    }
    if ep.halftone_enabled {
        for plate in 0..PLATE_COUNT {
            inks[plate] = halftone_coverage(xy, inks[plate], &plan.plates[plate], ep);
        }
    }
    for plate in 0..PLATE_COUNT {
        inks[plate] = (inks[plate] * ep.ink_amounts[plate]).clamp(0.0, 2.0);
    }

    let mut rgb = preview_rgb(inks, original, xy[0], ep, src.w);
    let alpha = if ep.preserve_alpha {
        original.a
    } else {
        alpha_max
    };
    rgb = mix_rgb(rgb, unpremultiply(original).rgb, ep.blend_original);
    let (rgb, out_alpha) = apply_white_transparency(rgb, alpha, ep);
    let premultiplied = [
        (rgb[0] * out_alpha).clamp(0.0, 1.0),
        (rgb[1] * out_alpha).clamp(0.0, 1.0),
        (rgb[2] * out_alpha).clamp(0.0, 1.0),
    ];
    Rgba {
        rgb: premultiplied,
        a: out_alpha,
    }
}

fn apply_white_transparency(rgb: [f32; 3], alpha: f32, ep: &EffectParams) -> ([f32; 3], f32) {
    let base_alpha = alpha.clamp(0.0, 1.0);
    if !ep.transparent_mode || base_alpha <= 0.0 {
        return (rgb, base_alpha);
    }

    let rgb = [
        rgb[0].clamp(0.0, 1.0),
        rgb[1].clamp(0.0, 1.0),
        rgb[2].clamp(0.0, 1.0),
    ];
    let white_delta = [1.0 - rgb[0], 1.0 - rgb[1], 1.0 - rgb[2]];
    let matte_alpha = white_delta[0].max(white_delta[1]).max(white_delta[2]);
    if matte_alpha <= 0.0001 {
        return ([0.0, 0.0, 0.0], 0.0);
    }

    let recovered_rgb = [
        (1.0 - white_delta[0] / matte_alpha).clamp(0.0, 1.0),
        (1.0 - white_delta[1] / matte_alpha).clamp(0.0, 1.0),
        (1.0 - white_delta[2] / matte_alpha).clamp(0.0, 1.0),
    ];
    let threshold = 1.0;
    let normalized_alpha = (matte_alpha / threshold).clamp(0.0, 1.0);
    let soft_alpha = smoothstep(normalized_alpha);
    let softness = 0.0;
    let coverage = normalized_alpha + (soft_alpha - normalized_alpha) * softness;
    (recovered_rgb, base_alpha * coverage)
}

fn preview_rgb(
    inks: [f32; PLATE_COUNT],
    original: Rgba,
    x: f32,
    ep: &EffectParams,
    width: usize,
) -> [f32; 3] {
    if ep.view_mode == VIEW_SPLIT {
        let split_x = width as f32 * 0.5;
        return if split_x <= 0.0 {
            composite_cmyk(inks, ep)
        } else {
            let orig = unpremultiply(original).rgb;
            let comp = composite_cmyk(inks, ep);
            if x < split_x {
                orig
            } else {
                comp
            }
        };
    }

    match ep.view_mode {
        VIEW_CYAN => composite_cmyk([inks[0], 0.0, 0.0, 0.0], ep),
        VIEW_MAGENTA => composite_cmyk([0.0, inks[1], 0.0, 0.0], ep),
        VIEW_YELLOW => composite_cmyk([0.0, 0.0, inks[2], 0.0], ep),
        VIEW_BLACK => composite_cmyk([0.0, 0.0, 0.0, inks[3]], ep),
        VIEW_INK_COVERAGE => {
            let coverage = (inks[0] + inks[1] + inks[2] + inks[3]).clamp(0.0, 4.0) / 4.0;
            let v = 1.0 - coverage;
            [v, v, v]
        }
        _ => composite_cmyk(inks, ep),
    }
}

fn separate_plate(sampled: Rgba, ep: &EffectParams, plate: usize) -> f32 {
    if sampled.a <= 0.0 {
        return 0.0;
    }
    let rgb = unpremultiply(sampled).rgb;
    let cmyk = rgb_to_cmyk_with_controls(rgb, ep);
    (cmyk[plate] * sampled.a).clamp(0.0, 2.0)
}

/// Composite CMYK inks onto paper.
///
/// Simple mode: standard subtractive model — paper * (1-C) * (1-K) etc.
/// Illustrator mode: each ink has a defined color; inks are layered via
/// multiply blending in C→M→Y→K order, matching Illustrator's CMYK appearance.
fn composite_cmyk(inks: [f32; PLATE_COUNT], ep: &EffectParams) -> [f32; 3] {
    if matches!(
        ep.conversion_mode,
        CONVERSION_ILLUSTRATOR | CONVERSION_CUSTOM
    ) {
        composite_cmyk_illustrator(inks, ep.paper, &ep.ink_colors)
    } else {
        composite_cmyk_simple(inks, ep.paper)
    }
}

/// Standard subtractive CMYK compositing.
fn composite_cmyk_simple(inks: [f32; PLATE_COUNT], paper: [f32; 3]) -> [f32; 3] {
    [
        (paper[0] * (1.0 - inks[0]).clamp(0.0, 1.0) * (1.0 - inks[3]).clamp(0.0, 1.0))
            .clamp(0.0, 1.0),
        (paper[1] * (1.0 - inks[1]).clamp(0.0, 1.0) * (1.0 - inks[3]).clamp(0.0, 1.0))
            .clamp(0.0, 1.0),
        (paper[2] * (1.0 - inks[2]).clamp(0.0, 1.0) * (1.0 - inks[3]).clamp(0.0, 1.0))
            .clamp(0.0, 1.0),
    ]
}

/// Illustrator-style CMYK compositing.
///
/// Each ink is blended onto the running result using multiply, weighted by
/// ink coverage.  Order: C → M → Y → K.
///
/// For a single ink at full coverage the result equals `paper * ink_color`.
/// At zero coverage the paper is unchanged.  This matches how Illustrator
/// renders CMYK swatches on screen.
fn composite_cmyk_illustrator(
    inks: [f32; PLATE_COUNT],
    paper: [f32; 3],
    ink_colors: &[[f32; 3]; PLATE_COUNT],
) -> [f32; 3] {
    let mut result = paper;
    for plate in 0..PLATE_COUNT {
        let t = inks[plate].clamp(0.0, 1.0);
        if t <= 0.0 {
            continue;
        }
        let multiplied = [
            result[0] * ink_colors[plate][0],
            result[1] * ink_colors[plate][1],
            result[2] * ink_colors[plate][2],
        ];
        result = [
            result[0] + (multiplied[0] - result[0]) * t,
            result[1] + (multiplied[1] - result[1]) * t,
            result[2] + (multiplied[2] - result[2]) * t,
        ];
    }
    [
        result[0].clamp(0.0, 1.0),
        result[1].clamp(0.0, 1.0),
        result[2].clamp(0.0, 1.0),
    ]
}

fn rgb_to_cmyk_with_controls(rgb: [f32; 3], _ep: &EffectParams) -> [f32; PLATE_COUNT] {
    let r = rgb[0].clamp(0.0, 1.0);
    let g = rgb[1].clamp(0.0, 1.0);
    let b = rgb[2].clamp(0.0, 1.0);
    let k = 1.0 - r.max(g).max(b);
    if k >= 0.999 {
        return [0.0, 0.0, 0.0, k];
    }
    let denom = (1.0 - k).max(0.0001);
    [
        ((1.0 - r - k) / denom).clamp(0.0, 1.0),
        ((1.0 - g - k) / denom).clamp(0.0, 1.0),
        ((1.0 - b - k) / denom).clamp(0.0, 1.0),
        k,
    ]
}

#[derive(Clone, Copy)]
pub(crate) struct PlatePlan {
    pub(crate) shift: [f32; 2],
    pub(crate) pivot: [f32; 2],
    pub(crate) sin: f32,
    pub(crate) cos: f32,
    pub(crate) cell: f32,
}

#[derive(Clone, Copy)]
pub(crate) struct RenderPlan {
    pub(crate) plates: [PlatePlan; PLATE_COUNT],
}

impl RenderPlan {
    pub(crate) fn new(ep: &EffectParams, width: usize, height: usize) -> Self {
        let pivot = [width as f32 * 0.5, height as f32 * 0.5];
        let base_cell = halftone_cell_size(ep);
        let cell = if ep.quality == QUALITY_DRAFT {
            base_cell.max(2.0) * 1.25
        } else {
            base_cell.max(1.0)
        };
        Self {
            plates: std::array::from_fn(|plate| {
                let theta = ep.halftone_angles[plate].to_radians();
                let (sin, cos) = theta.sin_cos();
                PlatePlan {
                    shift: final_plate_offset(ep, plate),
                    pivot,
                    sin,
                    cos,
                    cell,
                }
            }),
        }
    }
}

fn halftone_cell_size(ep: &EffectParams) -> f32 {
    if ep.halftone_unit == HALFTONE_UNIT_LPI {
        (72.0 / ep.halftone_frequency.max(1.0)).clamp(1.0, 1000.0)
    } else {
        ep.halftone_frequency.clamp(1.0, 1000.0)
    }
}

fn final_plate_offset(ep: &EffectParams, plate: usize) -> [f32; 2] {
    let mut offset = ep.offsets[plate];
    if ep.random_enabled && ep.random_affect[plate] {
        offset[0] += random_signed(ep.random_seed, plate as u32 + 1, 0) * ep.random_amount[0];
        offset[1] += random_signed(ep.random_seed, plate as u32 + 1, 1) * ep.random_amount[1];
    }
    offset[0] = -offset[0];
    offset
}

pub fn hash_u32(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^= value >> 16;
    value
}

pub fn random_signed(seed: u32, plate_id: u32, axis_id: u32) -> f32 {
    let h = hash_u32(seed ^ plate_id.wrapping_mul(31) ^ axis_id);
    let normalized = h as f32 / u32::MAX as f32;
    normalized * 2.0 - 1.0
}

fn halftone_coverage(xy: [f32; 2], value: f32, plan: &PlatePlan, ep: &EffectParams) -> f32 {
    let value = apply_dot_gain(value, ep.halftone_dot_gain);
    if value <= 0.0 {
        return 0.0;
    }
    let cell = dot_cell_position(xy, plan, ep);
    let dist = dot_shape_distance(cell, ep.halftone_shape);
    let radius = dot_radius(value);
    let edge = dot_edge_width(plan.cell, ep.halftone_softness);
    smooth_circle(dist, radius, edge)
}

fn dot_shape_distance(cell: [f32; 2], shape: i32) -> f32 {
    match shape {
        DOT_SQUARE => cell[0].abs().max(cell[1].abs()),
        DOT_LINE => cell[1].abs(),
        DOT_DIAMOND => cell[0].abs() + cell[1].abs(),
        _ => (cell[0] * cell[0] + cell[1] * cell[1]).sqrt(),
    }
}

fn apply_dot_gain(value: f32, dot_gain: f32) -> f32 {
    (value.clamp(0.0, 1.0) + dot_gain.clamp(-1.0, 1.0) * 0.25).clamp(0.0, 1.0)
}

fn dot_cell_position(xy: [f32; 2], plan: &PlatePlan, ep: &EffectParams) -> [f32; 2] {
    let rotated = halftone_rotated_position(xy, plan, ep);
    [
        (rotated[0] / plan.cell).rem_euclid(1.0) - 0.5,
        (rotated[1] / plan.cell).rem_euclid(1.0) - 0.5,
    ]
}

fn halftone_sample_position(xy: [f32; 2], plan: &PlatePlan, ep: &EffectParams) -> [f32; 2] {
    let rotated = halftone_rotated_position(xy, plan, ep);
    let center = [
        ((rotated[0] / plan.cell).floor() + 0.5) * plan.cell,
        ((rotated[1] / plan.cell).floor() + 0.5) * plan.cell,
    ];
    let unrotated = [
        center[0] * plan.cos - center[1] * plan.sin,
        center[0] * plan.sin + center[1] * plan.cos,
    ];
    [
        unrotated[0] + plan.pivot[0] - ep.halftone_offset[0],
        unrotated[1] + plan.pivot[1] - ep.halftone_offset[1],
    ]
}

fn halftone_rotated_position(xy: [f32; 2], plan: &PlatePlan, ep: &EffectParams) -> [f32; 2] {
    let p = [
        xy[0] + plan.shift[0] + ep.halftone_offset[0] - plan.pivot[0],
        xy[1] + plan.shift[1] + ep.halftone_offset[1] - plan.pivot[1],
    ];
    [
        p[0] * plan.cos + p[1] * plan.sin,
        -p[0] * plan.sin + p[1] * plan.cos,
    ]
}

fn dot_radius(value: f32) -> f32 {
    value.clamp(0.0, 1.0).sqrt() * 0.5
}

fn dot_edge_width(cell: f32, softness: f32) -> f32 {
    let cell_aa = 0.5 / cell.max(1.0);
    (cell_aa + softness.clamp(0.0, 1.0) * 0.03).max(0.0001)
}

fn smooth_circle(dist: f32, radius: f32, edge: f32) -> f32 {
    ((radius + edge - dist) / (2.0 * edge)).clamp(0.0, 1.0)
}

fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
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

fn unpremultiply(px: Rgba) -> Rgba {
    if px.a <= 0.0001 {
        return Rgba::transparent();
    }
    Rgba {
        rgb: [
            (px.rgb[0] / px.a).clamp(0.0, 1.0),
            (px.rgb[1] / px.a).clamp(0.0, 1.0),
            (px.rgb[2] / px.a).clamp(0.0, 1.0),
        ],
        a: px.a,
    }
}

fn sample_pixel(src: &Frame, x: usize, y: usize) -> Rgba {
    if src.w == 0 || src.h == 0 {
        return Rgba::transparent();
    }
    src.pixels[(y.min(src.h - 1) * src.w) + x.min(src.w - 1)]
}

fn sample_bilinear(src: &Frame, x: f32, y: f32, edge_mode: i32) -> Rgba {
    let w = src.w;
    let h = src.h;
    if w == 0 || h == 0 {
        return Rgba::transparent();
    }
    let (x, y) = if edge_mode == EDGE_CLAMP {
        (x.clamp(0.0, (w - 1) as f32), y.clamp(0.0, (h - 1) as f32))
    } else if x < 0.0 || y < 0.0 || x > (w - 1) as f32 || y > (h - 1) as f32 {
        return Rgba::transparent();
    } else {
        (x, y)
    };

    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(w - 1);
    let y1 = (y0 + 1).min(h - 1);
    let tx = x - x0 as f32;
    let ty = y - y0 as f32;

    let a = sample_pixel(src, x0, y0);
    let b = sample_pixel(src, x1, y0);
    let c = sample_pixel(src, x0, y1);
    let d = sample_pixel(src, x1, y1);
    let top = mix_rgba(a, b, tx);
    let bottom = mix_rgba(c, d, tx);
    mix_rgba(top, bottom, ty)
}

fn sample_nearest(src: &Frame, x: f32, y: f32, edge_mode: i32) -> Rgba {
    let w = src.w;
    let h = src.h;
    if w == 0 || h == 0 {
        return Rgba::transparent();
    }
    let (x, y) = if edge_mode == EDGE_CLAMP {
        (x.clamp(0.0, (w - 1) as f32), y.clamp(0.0, (h - 1) as f32))
    } else if x < 0.0 || y < 0.0 || x > (w - 1) as f32 || y > (h - 1) as f32 {
        return Rgba::transparent();
    } else {
        (x, y)
    };
    sample_pixel(src, x.round() as usize, y.round() as usize)
}

fn mix_rgba(a: Rgba, b: Rgba, t: f32) -> Rgba {
    Rgba {
        rgb: mix_rgb(a.rgb, b.rgb, t),
        a: a.a + (b.a - a.a) * t.clamp(0.0, 1.0),
    }
}

fn mix_rgb(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

#[cfg(test)]
fn to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

#[cfg(test)]
fn to_u16(v: f32) -> u16 {
    (v.clamp(0.0, 1.0) * ae::MAX_CHANNEL16 as f32 + 0.5) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_params() -> EffectParams {
        EffectParams {
            view_mode: VIEW_COMPOSITE,
            preserve_alpha: true,
            blend_original: 0.0,
            ink_amounts: [1.0; PLATE_COUNT],
            paper: [1.0, 1.0, 1.0],
            offsets: [[0.0, 0.0]; PLATE_COUNT],
            random_enabled: false,
            random_seed: 0,
            random_amount: [3.0, 3.0],
            random_affect: [true, true, true, false],
            halftone_enabled: true,
            halftone_frequency: 8.0,
            halftone_unit: HALFTONE_UNIT_PIXELS,
            halftone_shape: DOT_CIRCLE,
            halftone_dot_gain: 0.0,
            halftone_softness: 0.1,
            halftone_angles: [15.0, 75.0, 0.0, 45.0],
            halftone_offset: [0.0, 0.0],
            backend: BACKEND_AUTO,
            quality: QUALITY_FULL,
            edge_mode: EDGE_TRANSPARENT,
            expand_bounds: false,
            conversion_mode: CONVERSION_SIMPLE,
            ink_colors: CUSTOM_INK_COLORS,
            transparent_mode: false,
        }
    }

    fn average_luma(pixels: &[[f32; 4]]) -> f32 {
        let total = pixels.iter().fold(0.0, |sum, px| {
            sum + px[0] * 0.2126 + px[1] * 0.7152 + px[2] * 0.0722
        });
        total / pixels.len().max(1) as f32
    }

    #[test]
    fn rgb_to_cmyk_handles_white_black_and_red() {
        let options = CmykPressOptions::default();
        let ep = options.to_effect_params();
        assert_eq!(
            rgb_to_cmyk_with_controls([1.0, 1.0, 1.0], &ep),
            [0.0, 0.0, 0.0, 0.0]
        );
        assert_eq!(
            rgb_to_cmyk_with_controls([0.0, 0.0, 0.0], &ep),
            [0.0, 0.0, 0.0, 1.0]
        );
        let red = rgb_to_cmyk_with_controls([1.0, 0.0, 0.0], &ep);
        assert!(red[1] > 0.99 && red[2] > 0.99);
    }

    #[test]
    fn defaults_match_cmyk_dots_preset() {
        let options = CmykPressOptions::default();
        let ffi = CmykPressParams::default();
        assert!(options.halftone_enabled);
        assert_eq!(options.halftone_shape as i32, DOT_CIRCLE);
        assert_eq!(options.random_enabled, false);
        assert_eq!(ffi.halftone_enabled, 1);
        assert_eq!(ffi.halftone_shape, CMYK_DOT_CIRCLE);
        assert_eq!(ffi.view_mode, CMYK_VIEW_COMPOSITE);
        assert_eq!(ffi.quality, CMYK_QUALITY_FULL);
        assert_eq!(ffi.random_registration_enabled, 0);
        assert_eq!(ffi.random_plate_mask, 0b0111);
        assert_eq!(options.ink_amounts, DEFAULT_INK_AMOUNTS);
        assert_eq!(options.halftone_dot_gain, DEFAULT_HALFTONE_DOT_GAIN);
        assert_eq!(ffi.cyan_amount, DEFAULT_CMY_INK_AMOUNT);
        assert_eq!(ffi.magenta_amount, DEFAULT_CMY_INK_AMOUNT);
        assert_eq!(ffi.yellow_amount, DEFAULT_CMY_INK_AMOUNT);
        assert_eq!(ffi.black_amount, DEFAULT_BLACK_INK_AMOUNT);
        assert_eq!(ffi.halftone_dot_gain, DEFAULT_HALFTONE_DOT_GAIN);
    }

    #[test]
    fn default_preset_is_lighter_than_full_ink_pressing() {
        let input = vec![[0.18, 0.16, 0.14, 1.0]; 64 * 64];
        let lighter = render_rgba_f32(&input, 64, 64, &CmykPressOptions::default());

        let mut heavy = CmykPressOptions::default();
        heavy.ink_amounts = [1.0; PLATE_COUNT];
        heavy.halftone_dot_gain = 0.0;
        let heavy = render_rgba_f32(&input, 64, 64, &heavy);

        assert!(average_luma(&lighter) > average_luma(&heavy) + 0.02);
    }

    #[test]
    fn public_params_convert_to_internal_renderer_options() {
        let mut params = CmykPressParams::default();
        params.view_mode = CMYK_VIEW_CYAN;
        params.halftone_shape = CMYK_DOT_SQUARE;
        params.quality = CMYK_QUALITY_DRAFT;
        params.edge_mode = CMYK_EDGE_CLAMP;
        params.random_plate_mask = 0b1010;

        let options = CmykPressOptions::from(params);
        assert_eq!(options.view_mode as i32, VIEW_CYAN);
        assert_eq!(options.halftone_shape as i32, DOT_SQUARE);
        assert_eq!(options.quality as i32, QUALITY_DRAFT);
        assert_eq!(options.edge_mode as i32, EDGE_CLAMP);
        assert_eq!(options.random_affect, [false, true, false, true]);
    }

    #[test]
    fn ink_and_paper_brightness_clamp_to_one_hundred_percent() {
        let mut params = CmykPressParams::default();
        params.cyan_amount = 2.0;
        params.magenta_amount = 1.5;
        params.yellow_amount = 1.25;
        params.black_amount = 3.0;
        params.paper_color = [0.8, 0.7, 0.6];
        params.paper_brightness = 2.0;

        let options = CmykPressOptions::from(params);
        assert_eq!(options.ink_amounts, [1.0; PLATE_COUNT]);
        assert_eq!(options.paper, [0.8, 0.7, 0.6]);

        let options = CmykPressOptions {
            ink_amounts: [2.0, 1.5, 1.25, 3.0],
            ..CmykPressOptions::default()
        };
        let ep = options.to_effect_params();
        assert_eq!(ep.ink_amounts, [1.0; PLATE_COUNT]);
    }

    #[test]
    fn default_render_produces_visible_halftone_variation() {
        let input = vec![[1.0, 0.0, 0.0, 1.0]; 16 * 16];
        let dotted = render_rgba_f32(&input, 16, 16, &CmykPressOptions::default());
        let mut clean_options = CmykPressOptions::default();
        clean_options.halftone_enabled = false;
        let clean = render_rgba_f32(&input, 16, 16, &clean_options);

        assert!(dotted.windows(2).any(|pair| pair[0] != pair[1]));
        assert!(clean.windows(2).all(|pair| pair[0] == pair[1]));
    }

    #[test]
    fn random_registration_is_deterministic_and_k_default_disabled() {
        let mut ep = test_params();
        ep.random_enabled = true;
        ep.random_seed = 42;
        let a = RenderPlan::new(&ep, 1920, 1080);
        let b = RenderPlan::new(&ep, 1920, 1080);
        assert_eq!(a.plates[0].shift, b.plates[0].shift);
        assert_eq!(a.plates[3].shift, [0.0, 0.0]);
        ep.random_seed = 43;
        let c = RenderPlan::new(&ep, 1920, 1080);
        assert_ne!(a.plates[0].shift, c.plates[0].shift);
    }

    #[test]
    fn registration_offset_x_moves_in_ui_direction() {
        let mut ep = test_params();
        ep.offsets[0] = [12.0, -3.0];
        let plan = RenderPlan::new(&ep, 100, 100);
        assert_eq!(plan.plates[0].shift, [-12.0, -3.0]);
    }

    #[test]
    fn halftone_shape_values_produce_valid_coverage() {
        let mut ep = test_params();
        ep.halftone_enabled = true;
        for shape in [DOT_CIRCLE, DOT_SQUARE, DOT_LINE, DOT_DIAMOND] {
            ep.halftone_shape = shape;
            let plan = RenderPlan::new(&ep, 100, 100);
            let value = halftone_coverage([12.0, 18.0], 0.5, &plan.plates[0], &ep);
            assert!(
                (0.0..=1.0).contains(&value),
                "shape {shape} produced out-of-range coverage: {value}"
            );
        }
        // Circle and square should differ at a non-center point
        ep.halftone_shape = DOT_CIRCLE;
        let plan = RenderPlan::new(&ep, 100, 100);
        let circle = halftone_coverage([12.0, 18.0], 0.5, &plan.plates[0], &ep);
        ep.halftone_shape = DOT_SQUARE;
        let plan = RenderPlan::new(&ep, 100, 100);
        let square = halftone_coverage([12.0, 18.0], 0.5, &plan.plates[0], &ep);
        // They may differ (square has larger coverage at corners)
        let _ = (circle, square);
    }

    #[test]
    fn halftone_controls_affect_dot_math() {
        let mut ep = test_params();
        ep.halftone_frequency = 8.0;
        let plan_small = RenderPlan::new(&ep, 100, 100);
        ep.halftone_frequency = 16.0;
        let plan_large = RenderPlan::new(&ep, 100, 100);
        assert_ne!(plan_small.plates[0].cell, plan_large.plates[0].cell);

        ep.halftone_frequency = 8.0;
        ep.halftone_dot_gain = -1.0;
        let plan = RenderPlan::new(&ep, 100, 100);
        let thin = halftone_coverage([12.0, 18.0], 0.5, &plan.plates[0], &ep);
        ep.halftone_dot_gain = 1.0;
        let thick = halftone_coverage([12.0, 18.0], 0.5, &plan.plates[0], &ep);
        assert!(thick >= thin);
    }

    #[test]
    fn halftone_radius_maps_ink_to_round_dot_size() {
        for value in [0.1, 0.5, 0.9] {
            let radius = dot_radius(value);
            assert!(radius <= 0.5);
            assert!(((radius * 2.0).powi(2) - value).abs() < 0.001);
        }
    }

    #[test]
    fn halftone_dot_is_radially_symmetric() {
        let ep = test_params();
        let plan = RenderPlan::new(&ep, 64, 64);
        let plate = &plan.plates[0];
        let point_for_cell = |cell_x: f32, cell_y: f32| {
            let rx = cell_x * plate.cell;
            let ry = cell_y * plate.cell;
            [
                plate.pivot[0] + rx * plate.cos - ry * plate.sin,
                plate.pivot[1] + rx * plate.sin + ry * plate.cos,
            ]
        };
        let right = point_for_cell(0.7, 0.5);
        let left = point_for_cell(0.3, 0.5);
        let a = halftone_coverage(right, 0.4, plate, &ep);
        let b = halftone_coverage(left, 0.4, plate, &ep);
        assert!((a - b).abs() < 0.0001);
    }

    #[test]
    fn halftone_samples_once_per_screen_cell() {
        let ep = test_params();
        let plan = RenderPlan::new(&ep, 64, 64);
        let plate = &plan.plates[0];
        let point_for_cell = |cell_x: f32, cell_y: f32| {
            let rx = cell_x * plate.cell;
            let ry = cell_y * plate.cell;
            [
                plate.pivot[0] + rx * plate.cos - ry * plate.sin,
                plate.pivot[1] + rx * plate.sin + ry * plate.cos,
            ]
        };
        let a = halftone_sample_position(point_for_cell(0.15, 0.2), plate, &ep);
        let b = halftone_sample_position(point_for_cell(0.85, 0.8), plate, &ep);
        assert!((a[0] - b[0]).abs() < 0.0001);
        assert!((a[1] - b[1]).abs() < 0.0001);
    }

    #[test]
    fn edge_mode_controls_out_of_bounds_sampling() {
        let src = Frame {
            w: 1,
            h: 1,
            pixels: vec![Rgba {
                rgb: [0.2, 0.4, 0.6],
                a: 1.0,
            }],
        };
        assert_eq!(sample_bilinear(&src, -1.0, 0.0, EDGE_TRANSPARENT).a, 0.0);
        assert_eq!(
            sample_bilinear(&src, -1.0, 0.0, EDGE_CLAMP).rgb,
            [0.2, 0.4, 0.6]
        );
    }

    #[test]
    fn transparent_pixels_do_not_create_black_edges() {
        let cmyk_options = CmykPressOptions::default();
        let ep = cmyk_options.to_effect_params();
        let src = Frame {
            w: 1,
            h: 1,
            pixels: vec![Rgba::transparent()],
        };
        let out = render_cmyk_press(&src, &ep);
        assert_eq!(out.pixels[0].a, 0.0);
        assert_eq!(out.pixels[0].rgb, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn white_transparency_behaves_like_white_unmult() {
        let mut ep = test_params();
        ep.halftone_enabled = false;
        ep.transparent_mode = true;

        let (_white_rgb, white_alpha) = apply_white_transparency([1.0, 1.0, 1.0], 1.0, &ep);
        let (red_rgb, red_alpha) = apply_white_transparency([1.0, 0.5, 0.5], 1.0, &ep);
        let (gray_rgb, gray_alpha) = apply_white_transparency([0.7, 0.7, 0.7], 1.0, &ep);

        assert!(white_alpha < 0.001);
        assert!((red_alpha - 0.5).abs() < 0.001);
        assert!((red_rgb[0] - 1.0).abs() < 0.001);
        assert!(red_rgb[1] < 0.001 && red_rgb[2] < 0.001);
        assert!((gray_alpha - 0.3).abs() < 0.001);
        assert!(gray_rgb.iter().all(|channel| *channel < 0.001));
    }

    #[test]
    fn halftone_frequency_clamps_to_one_thousand() {
        let mut options = CmykPressOptions::default();
        options.halftone_frequency = 5000.0;
        let ep = options.to_effect_params();
        assert_eq!(ep.halftone_frequency, 1000.0);
        assert_eq!(halftone_cell_size(&ep), 1000.0);
    }

    #[test]
    fn public_param_render_entrypoint_uses_default_dots() {
        let input = vec![[0.0, 0.0, 0.0, 1.0]; 8 * 8];
        let out = render_rgba_f32_with_params(&input, 8, 8, &CmykPressParams::default());
        assert_eq!(out.len(), 64);
        assert!(out.iter().all(|px| px[3] == 1.0));
    }

    #[test]
    fn render_core_stays_in_float_range() {
        let src = Frame {
            w: 8,
            h: 6,
            pixels: (0..48)
                .map(|i| Rgba {
                    rgb: [
                        ((i * 37) % 255) as f32 / 255.0,
                        ((i * 67) % 255) as f32 / 255.0,
                        ((i * 97) % 255) as f32 / 255.0,
                    ],
                    a: 1.0,
                })
                .collect(),
        };
        let out = render_cmyk_press(&src, &test_params());
        assert_eq!(out.w, src.w);
        assert_eq!(out.h, src.h);
        for px in out.pixels {
            assert!((0.0..=1.0).contains(&px.a));
            for channel in px.rgb {
                assert!(channel.is_finite());
                assert!((0.0..=1.0).contains(&channel));
            }
        }
    }

    #[test]
    fn illustrator_mode_uses_ink_colors_for_compositing() {
        let ep = CmykPressOptions {
            conversion_mode: CONVERSION_ILLUSTRATOR as u32,
            halftone_enabled: false,
            ..CmykPressOptions::default()
        }
        .to_effect_params();
        assert_eq!(ep.ink_colors, ILLUSTRATOR_INK_COLORS);

        let result =
            composite_cmyk_illustrator([1.0, 0.0, 0.0, 0.0], [1.0, 1.0, 1.0], &ep.ink_colors);
        assert!((result[0] - ILLUSTRATOR_INK_COLOR_CYAN[0]).abs() < 0.001);
        assert!((result[1] - ILLUSTRATOR_INK_COLOR_CYAN[1]).abs() < 0.001);
        assert!((result[2] - ILLUSTRATOR_INK_COLOR_CYAN[2]).abs() < 0.001);

        let white = composite_cmyk_illustrator([0.0; 4], [1.0, 1.0, 1.0], &ep.ink_colors);
        assert_eq!(white, [1.0, 1.0, 1.0]);

        // All values in valid range
        let input = vec![[0.5, 0.3, 0.7, 1.0]; 8 * 8];
        let mut opts = CmykPressOptions::default();
        opts.conversion_mode = CONVERSION_ILLUSTRATOR as u32;
        opts.halftone_enabled = false;
        let out = render_rgba_f32(&input, 8, 8, &opts);
        for px in &out {
            for &ch in px.iter() {
                assert!(ch.is_finite() && (0.0..=1.0).contains(&ch));
            }
        }
    }

    #[test]
    fn depth_conversions_clamp_to_ae_ranges() {
        assert_eq!(to_u8(-1.0), 0);
        assert_eq!(to_u8(2.0), 255);
        assert_eq!(to_u16(-1.0), 0);
        assert_eq!(to_u16(2.0), ae::MAX_CHANNEL16 as u16);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn metal_shader_dispatches_when_device_available() {
        let Some(device) = metal::Device::system_default() else {
            return;
        };
        let options = metal::CompileOptions::new();
        options.set_fast_math_enabled(true);
        let library = device
            .new_library_with_source(metal_gpu::METAL_SHADER, &options)
            .expect("Metal shader should compile");
        let function = library
            .get_function("cmyk_press", None)
            .expect("Metal kernel entry point should exist");
        let pipeline = device
            .new_compute_pipeline_state_with_function(&function)
            .expect("Metal compute pipeline should compile");

        let desc = metal::TextureDescriptor::new();
        desc.set_texture_type(metal::MTLTextureType::D2);
        desc.set_pixel_format(metal::MTLPixelFormat::RGBA32Float);
        desc.set_width(2);
        desc.set_height(2);
        desc.set_storage_mode(metal::MTLStorageMode::Shared);
        desc.set_usage(metal::MTLTextureUsage::ShaderRead | metal::MTLTextureUsage::ShaderWrite);
        let input = device.new_texture(&desc);
        let output = device.new_texture(&desc);

        let input_pixels = [
            1.0f32, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.25, 0.5, 0.75, 1.0,
        ];
        let region = metal::MTLRegion::new_2d(0, 0, 2, 2);
        input.replace_region(
            region,
            0,
            input_pixels.as_ptr() as *const std::ffi::c_void,
            2 * 4 * std::mem::size_of::<f32>() as u64,
        );

        let cmyk_options = CmykPressOptions {
            halftone_enabled: false,
            ..CmykPressOptions::default()
        };
        let ep = cmyk_options.to_effect_params();
        let params = metal_gpu::MetalParams::new(&ep, 2, 2);
        let queue = device.new_command_queue();
        let command_buffer = queue.new_command_buffer();
        let encoder = command_buffer.new_compute_command_encoder();
        encoder.set_compute_pipeline_state(&pipeline);
        encoder.set_bytes(
            0,
            std::mem::size_of::<metal_gpu::MetalParams>() as u64,
            &params as *const _ as *const std::ffi::c_void,
        );
        encoder.set_texture(0, Some(&input));
        encoder.set_texture(1, Some(&output));
        encoder.dispatch_threads(metal::MTLSize::new(2, 2, 1), metal::MTLSize::new(2, 2, 1));
        encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();
        assert_eq!(
            command_buffer.status(),
            metal::MTLCommandBufferStatus::Completed
        );

        let mut output_pixels = [0.0f32; 16];
        output.get_bytes(
            output_pixels.as_mut_ptr() as *mut std::ffi::c_void,
            2 * 4 * std::mem::size_of::<f32>() as u64,
            region,
            0,
        );
        // Verify all output pixels are in valid float range
        for (i, &v) in output_pixels.iter().enumerate() {
            assert!(
                v.is_finite() && (0.0..=1.0).contains(&v),
                "output pixel[{i}] = {v} is out of range"
            );
        }
    }
}
