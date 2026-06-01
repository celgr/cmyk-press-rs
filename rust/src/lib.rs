use after_effects as ae;
mod render_core;
mod util;

#[cfg(target_os = "macos")]
use render_core::can_use_composite_single_sample_path;
#[cfg(target_os = "macos")]
use render_core::RenderPlan;
use render_core::{render_cmyk_press, EffectParams};
pub use render_core::{
    render_rgba_f32, render_rgba_f32_with_params, CmykPressOptions, CmykPressParams,
};
use util::{rgba_from_straight, straight_rgb_from_rgba, to_u8, Frame, Rgba};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    PresetStart,
    PresetEnd,
    ConversionStart,
    ConversionEnd,
    InkStart,
    InkEnd,
    PaperStart,
    PaperEnd,
    InkColorStart,
    InkColorEnd,
    RegistrationStart,
    RegistrationEnd,
    RandomStart,
    RandomEnd,
    HalftoneStart,
    HalftoneEnd,
    RenderingStart,
    RenderingEnd,
    Preset,
    ConversionMode,
    ViewMode,
    PreserveAlpha,
    BlendOriginal,
    CyanAmount,
    MagentaAmount,
    YellowAmount,
    BlackAmount,
    PaperColor,
    PaperBrightness,
    InkColorCyan,
    InkColorMagenta,
    InkColorYellow,
    InkColorBlack,
    CyanOffsetX,
    CyanOffsetY,
    MagentaOffsetX,
    MagentaOffsetY,
    YellowOffsetX,
    YellowOffsetY,
    BlackOffsetX,
    BlackOffsetY,
    RandomEnable,
    RandomSeed,
    RandomAmountX,
    RandomAmountY,
    RandomAffectCyan,
    RandomAffectMagenta,
    RandomAffectYellow,
    RandomAffectBlack,
    HalftoneEnable,
    HalftoneFrequency,
    HalftoneUnit,
    HalftoneShape,
    HalftoneDotGain,
    HalftoneSoftness,
    HalftoneCyanAngle,
    HalftoneMagentaAngle,
    HalftoneYellowAngle,
    HalftoneBlackAngle,
    HalftoneOffsetX,
    HalftoneOffsetY,
    HalftoneOffsetPoint,
    Backend,
    Quality,
    Sampling,
    EdgeMode,
    ExpandBounds,
    TransparentMode,
}

#[derive(Default)]
struct Plugin;

ae::define_effect!(Plugin, (), Params);

pub const PLATE_COUNT: usize = 4;
pub const CMYK_PRESET_DEFAULT_DOTS: u32 = 0;
pub const CMYK_VIEW_COMPOSITE: u32 = 0;
pub const CMYK_VIEW_CYAN: u32 = 1;
pub const CMYK_VIEW_MAGENTA: u32 = 2;
pub const CMYK_VIEW_YELLOW: u32 = 3;
pub const CMYK_VIEW_BLACK: u32 = 4;
pub const CMYK_VIEW_INK_COVERAGE: u32 = 5;
pub const CMYK_VIEW_SPLIT: u32 = 6;
pub const CMYK_DOT_CIRCLE: u32 = 0;
pub const CMYK_DOT_SQUARE: u32 = 1;
pub const CMYK_DOT_LINE: u32 = 2;
pub const CMYK_DOT_DIAMOND: u32 = 3;
pub const CMYK_QUALITY_DRAFT: u32 = 0;
pub const CMYK_QUALITY_FULL: u32 = 1;
pub const CMYK_SAMPLING_BILINEAR: u32 = 1;
pub const CMYK_SAMPLING_NEAREST: u32 = 2;
pub const CMYK_EDGE_TRANSPARENT: u32 = 0;
pub const CMYK_EDGE_CLAMP: u32 = 1;
pub const CMYK_EDGE_MIRROR: u32 = 2;

const PRESET_DEFAULT_DOTS: i32 = 1;
const VIEW_COMPOSITE: i32 = 1;
const VIEW_CYAN: i32 = 2;
const VIEW_MAGENTA: i32 = 3;
const VIEW_YELLOW: i32 = 4;
const VIEW_BLACK: i32 = 5;
const VIEW_INK_COVERAGE: i32 = 6;
const VIEW_SPLIT: i32 = 7;
const HALFTONE_UNIT_PIXELS: i32 = 1;
const HALFTONE_UNIT_LPI: i32 = 2;
const DOT_CIRCLE: i32 = 1;
const DOT_SQUARE: i32 = 2;
const DOT_LINE: i32 = 3;
const DOT_DIAMOND: i32 = 4;
const BACKEND_AUTO: i32 = 1;
const BACKEND_CPU: i32 = 2;
const BACKEND_GPU: i32 = 3;
const QUALITY_DRAFT: i32 = 1;
const QUALITY_FULL: i32 = 2;
const SAMPLING_BILINEAR: i32 = 1;
const SAMPLING_NEAREST: i32 = 2;
const EDGE_TRANSPARENT: i32 = 1;
const EDGE_CLAMP: i32 = 2;
const EDGE_MIRROR: i32 = 3;
const DEFAULT_CMY_INK_AMOUNT: f32 = 1.0;
const DEFAULT_BLACK_INK_AMOUNT: f32 = 1.0;

// Conversion modes
const CONVERSION_SIMPLE: i32 = 1;
const CONVERSION_ILLUSTRATOR: i32 = 2;
const CONVERSION_CUSTOM: i32 = 3;

const CUSTOM_INK_COLOR_CYAN: [f32; 3] = [0.0, 1.0, 1.0];
const CUSTOM_INK_COLOR_MAGENTA: [f32; 3] = [1.0, 0.0, 1.0];
const CUSTOM_INK_COLOR_YELLOW: [f32; 3] = [1.0, 1.0, 0.0];
const CUSTOM_INK_COLOR_BLACK: [f32; 3] = [0.0, 0.0, 0.0];
const CUSTOM_INK_COLORS: [[f32; 3]; PLATE_COUNT] = [
    CUSTOM_INK_COLOR_CYAN,
    CUSTOM_INK_COLOR_MAGENTA,
    CUSTOM_INK_COLOR_YELLOW,
    CUSTOM_INK_COLOR_BLACK,
];
const ILLUSTRATOR_INK_COLOR_CYAN: [f32; 3] = [0.0, 160.0 / 255.0, 233.0 / 255.0];
const ILLUSTRATOR_INK_COLOR_MAGENTA: [f32; 3] = [228.0 / 255.0, 0.0, 127.0 / 255.0];
const ILLUSTRATOR_INK_COLOR_YELLOW: [f32; 3] = [1.0, 241.0 / 255.0, 0.0];
const ILLUSTRATOR_INK_COLOR_BLACK: [f32; 3] = [35.0 / 255.0, 24.0 / 255.0, 21.0 / 255.0];
const ILLUSTRATOR_INK_COLORS: [[f32; 3]; PLATE_COUNT] = [
    ILLUSTRATOR_INK_COLOR_CYAN,
    ILLUSTRATOR_INK_COLOR_MAGENTA,
    ILLUSTRATOR_INK_COLOR_YELLOW,
    ILLUSTRATOR_INK_COLOR_BLACK,
];
const DEFAULT_HALFTONE_DOT_GAIN: f32 = 0.0;
const DEFAULT_INK_AMOUNTS: [f32; PLATE_COUNT] = [
    DEFAULT_CMY_INK_AMOUNT,
    DEFAULT_CMY_INK_AMOUNT,
    DEFAULT_CMY_INK_AMOUNT,
    DEFAULT_BLACK_INK_AMOUNT,
];

impl AdobePluginGlobal for Plugin {
    fn params_setup(
        &self,
        params: &mut ae::Parameters<Params>,
        _in_data: ae::InData,
        _out_data: ae::OutData,
    ) -> Result<(), ae::Error> {
        params.add_group(
            Params::PresetStart,
            Params::PresetEnd,
            "Preset",
            false,
            |params| {
                params.add_with_flags(
                    Params::Preset,
                    "Preset",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&["Default CMYK Dots", "Custom"]);
                        f.set_default(PRESET_DEFAULT_DOTS);
                    }),
                    ae::ParamFlag::SUPERVISE,
                    ae::ParamUIFlags::DISABLED,
                )?;
                Ok(())
            },
        )?;

        params.add_group(
            Params::ConversionStart,
            Params::ConversionEnd,
            "Conversion",
            false,
            |params| {
                params.add(
                    Params::ConversionMode,
                    "Mode",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&["Simple", "Illustrator CMYK", "Custom"]);
                        f.set_default(CONVERSION_SIMPLE);
                    }),
                )?;
                params.add(
                    Params::ViewMode,
                    "View",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&[
                            "Composite",
                            "Cyan",
                            "Magenta",
                            "Yellow",
                            "Black",
                            "Ink Coverage",
                            "Original / Result Split",
                        ]);
                        f.set_default(VIEW_COMPOSITE);
                    }),
                )?;
                params.add(
                    Params::PreserveAlpha,
                    "Preserve Alpha",
                    ae::CheckBoxDef::setup(|f| {
                        f.set_default(true);
                        f.set_label("Keep input alpha");
                    }),
                )?;
                params.add(
                    Params::TransparentMode,
                    "Paper Background",
                    ae::CheckBoxDef::setup(|f| {
                        f.set_default(true);
                        f.set_label("Show paper background");
                    }),
                )?;
                Ok(())
            },
        )?;

        params.add_group(
            Params::InkColorStart,
            Params::InkColorEnd,
            "Ink Color",
            true,
            |params| {
                add_color_param(
                    params,
                    Params::InkColorCyan,
                    "Cyan Color",
                    rgb8(0, 255, 255),
                )?;
                add_color_param(
                    params,
                    Params::InkColorMagenta,
                    "Magenta Color",
                    rgb8(255, 0, 255),
                )?;
                add_color_param(
                    params,
                    Params::InkColorYellow,
                    "Yellow Color",
                    rgb8(255, 255, 0),
                )?;
                add_color_param(params, Params::InkColorBlack, "Black Color", rgb8(0, 0, 0))?;
                Ok(())
            },
        )?;

        params.add_group(
            Params::InkStart,
            Params::InkEnd,
            "Ink Amount",
            false,
            |params| {
                add_percent_param(
                    params,
                    Params::CyanAmount,
                    "Cyan",
                    DEFAULT_CMY_INK_AMOUNT * 100.0,
                    0.0,
                    100.0,
                )?;
                add_percent_param(
                    params,
                    Params::MagentaAmount,
                    "Magenta",
                    DEFAULT_CMY_INK_AMOUNT * 100.0,
                    0.0,
                    100.0,
                )?;
                add_percent_param(
                    params,
                    Params::YellowAmount,
                    "Yellow",
                    DEFAULT_CMY_INK_AMOUNT * 100.0,
                    0.0,
                    100.0,
                )?;
                add_percent_param(
                    params,
                    Params::BlackAmount,
                    "Black",
                    DEFAULT_BLACK_INK_AMOUNT * 100.0,
                    0.0,
                    100.0,
                )?;
                Ok(())
            },
        )?;

        params.add_group(
            Params::PaperStart,
            Params::PaperEnd,
            "Paper",
            true,
            |params| {
                add_color_param(params, Params::PaperColor, "Color", rgb8(255, 255, 255))?;
                add_percent_param(
                    params,
                    Params::PaperBrightness,
                    "Brightness",
                    100.0,
                    0.0,
                    100.0,
                )?;
                Ok(())
            },
        )?;

        params.add_group(
            Params::RegistrationStart,
            Params::RegistrationEnd,
            "Registration Offset",
            true,
            |params| {
                add_offset_param(params, Params::CyanOffsetX, "Cyan Offset X", 0.0)?;
                add_offset_param(params, Params::CyanOffsetY, "Cyan Offset Y", 0.0)?;
                add_offset_param(params, Params::MagentaOffsetX, "Magenta Offset X", 0.0)?;
                add_offset_param(params, Params::MagentaOffsetY, "Magenta Offset Y", 0.0)?;
                add_offset_param(params, Params::YellowOffsetX, "Yellow Offset X", 0.0)?;
                add_offset_param(params, Params::YellowOffsetY, "Yellow Offset Y", 0.0)?;
                add_offset_param(params, Params::BlackOffsetX, "Black Offset X", 0.0)?;
                add_offset_param(params, Params::BlackOffsetY, "Black Offset Y", 0.0)?;
                Ok(())
            },
        )?;

        params.add_group(
            Params::RandomStart,
            Params::RandomEnd,
            "Random Registration",
            true,
            |params| {
                params.add(
                    Params::RandomEnable,
                    "Enable",
                    ae::CheckBoxDef::setup(|f| {
                        f.set_default(false);
                        f.set_label("Enable");
                    }),
                )?;
                params.add(
                    Params::RandomSeed,
                    "Seed",
                    ae::FloatSliderDef::setup(|f| {
                        f.set_valid_min(0.0);
                        f.set_valid_max(2_147_483_647.0);
                        f.set_slider_min(0.0);
                        f.set_slider_max(100.0);
                        f.set_default(0.0);
                        f.set_precision(0);
                        f.set_curve_tolerance(1.0);
                    }),
                )?;
                add_px_param_with_slider_range(
                    params,
                    Params::RandomAmountX,
                    "Amount X",
                    3.0,
                    0.0,
                    1000.0,
                    0.0,
                    100.0,
                )?;
                add_px_param_with_slider_range(
                    params,
                    Params::RandomAmountY,
                    "Amount Y",
                    3.0,
                    0.0,
                    1000.0,
                    0.0,
                    100.0,
                )?;
                add_bool_param(params, Params::RandomAffectCyan, "Affect Cyan", true)?;
                add_bool_param(params, Params::RandomAffectMagenta, "Affect Magenta", true)?;
                add_bool_param(params, Params::RandomAffectYellow, "Affect Yellow", true)?;
                add_bool_param(params, Params::RandomAffectBlack, "Affect Black", false)?;
                Ok(())
            },
        )?;

        params.add_group(
            Params::HalftoneStart,
            Params::HalftoneEnd,
            "Halftone",
            true,
            |params| {
                add_bool_param(params, Params::HalftoneEnable, "Enable", true)?;
                add_px_param(params, Params::HalftoneFrequency, "Size", 8.0, 1.0, 1000.0)?;
                params.add(
                    Params::HalftoneUnit,
                    "Unit",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&["Pixels", "Lines Per Inch"]);
                        f.set_default(HALFTONE_UNIT_PIXELS);
                    }),
                )?;
                params.add(
                    Params::HalftoneShape,
                    "Dot Shape",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&["Dot", "Square", "Line", "Diamond"]);
                        f.set_default(DOT_CIRCLE);
                    }),
                )?;
                add_signed_percent_param(
                    params,
                    Params::HalftoneDotGain,
                    "Dot Gain",
                    DEFAULT_HALFTONE_DOT_GAIN * 100.0,
                )?;
                add_percent_param(
                    params,
                    Params::HalftoneSoftness,
                    "Softness",
                    0.0,
                    0.0,
                    100.0,
                )?;
                add_angle_param(params, Params::HalftoneCyanAngle, "Cyan Angle", 15.0)?;
                add_angle_param(params, Params::HalftoneMagentaAngle, "Magenta Angle", 75.0)?;
                add_angle_param(params, Params::HalftoneYellowAngle, "Yellow Angle", 0.0)?;
                add_angle_param(params, Params::HalftoneBlackAngle, "Black Angle", 45.0)?;
                add_hidden_px_param(params, Params::HalftoneOffsetX, "Legacy Offset X", 0.0)?;
                add_hidden_px_param(params, Params::HalftoneOffsetY, "Legacy Offset Y", 0.0)?;
                params.add(
                    Params::HalftoneOffsetPoint,
                    "Offset",
                    ae::PointDef::setup(|f| {
                        f.set_default((50.0, 50.0));
                        f.set_value(f.default());
                        f.set_restrict_bounds(false);
                    }),
                )?;
                Ok(())
            },
        )?;

        params.add_group(
            Params::RenderingStart,
            Params::RenderingEnd,
            "Rendering",
            true,
            |params| {
                params.add(
                    Params::Backend,
                    "Backend",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&["Auto", "CPU", "GPU"]);
                        f.set_default(BACKEND_AUTO);
                    }),
                )?;
                params.add(
                    Params::Quality,
                    "Quality",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&["Draft", "Full"]);
                        f.set_default(QUALITY_FULL);
                    }),
                )?;
                params.add(
                    Params::Sampling,
                    "Sampling",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&["Bilinear", "Nearest"]);
                        f.set_default(SAMPLING_BILINEAR);
                    }),
                )?;
                params.add(
                    Params::EdgeMode,
                    "Edge Mode",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&["None", "Stretch", "Mirror"]);
                        f.set_default(EDGE_TRANSPARENT);
                    }),
                )?;
                add_bool_param(params, Params::ExpandBounds, "Expand Bounds", true)?;
                add_percent_param(
                    params,
                    Params::BlendOriginal,
                    "Blend With Original",
                    0.0,
                    0.0,
                    100.0,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }

    fn handle_command(
        &mut self,
        cmd: ae::Command,
        in_data: ae::InData,
        mut out_data: ae::OutData,
        params: &mut ae::Parameters<Params>,
    ) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::GlobalSetup => {
                let _ = in_data
                    .effect()
                    .effect_wants_checked_out_frames_to_match_render_pixel_format();
                out_data.set_out_flag2(ae::OutFlags2::SupportsGpuRenderF32, true);
            }
            ae::Command::About => {
                out_data.set_return_msg("CMYK Press v0.1");
            }
            ae::Command::Render {
                in_layer,
                mut out_layer,
            } => {
                let src = layer_to_frame(&in_layer);
                let mut ep = get_params(params)?;
                center_halftone_offset(&mut ep, src.w, src.h);
                let out = render_cmyk_press(&src, &ep);
                frame_to_layer(&out, &mut out_layer);
            }
            ae::Command::SmartPreRender { mut extra } => {
                smart_pre_render(&in_data, &mut extra, params)?;
            }
            ae::Command::SmartRender { extra } => {
                smart_render(&extra, params)?;
            }
            ae::Command::SmartRenderGpu { extra } => {
                smart_render_gpu(&extra, params, &in_data)?;
            }
            ae::Command::GpuDeviceSetup { mut extra } => {
                gpu_device_setup(&in_data, &mut out_data, &mut extra)?;
            }
            ae::Command::GpuDeviceSetdown { mut extra } => {
                gpu_device_setdown(&mut extra)?;
            }
            ae::Command::UpdateParamsUi => {
                update_preset_ui(params)?;
                update_ink_color_ui(params)?;
                out_data.set_out_flag(ae::OutFlags::RefreshUi, true);
            }
            ae::Command::UserChangedParam { .. } => {
                out_data.set_force_rerender();
            }
            _ => {}
        }

        Ok(())
    }
}

fn update_preset_ui(params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
    let mut params_copy = params.cloned();
    let mut preset = params_copy.get_mut(Params::Preset)?;
    preset.set_ui_flag(ae::ParamUIFlags::DISABLED, true);
    preset.update_param_ui()?;
    Ok(())
}

fn update_ink_color_ui(params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
    let is_custom =
        normalize_conversion_mode(params.get(Params::ConversionMode)?.as_popup()?.value() as i32)
            == CONVERSION_CUSTOM;
    let mut params_copy = params.cloned();
    for param in [
        Params::InkColorCyan,
        Params::InkColorMagenta,
        Params::InkColorYellow,
        Params::InkColorBlack,
    ] {
        let mut color = params_copy.get_mut(param)?;
        color.set_ui_flag(ae::ParamUIFlags::DISABLED, !is_custom);
        color.update_param_ui()?;
    }
    Ok(())
}

fn add_bool_param(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: bool,
) -> Result<(), ae::Error> {
    params.add(
        id,
        name,
        ae::CheckBoxDef::setup(|f| {
            f.set_default(default.into());
            f.set_label(name);
        }),
    )
}

fn add_percent_param(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: f32,
    min: f32,
    max: f32,
) -> Result<(), ae::Error> {
    params.add(
        id,
        name,
        ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(min);
            f.set_valid_max(max);
            f.set_slider_min(min);
            f.set_slider_max(max);
            f.set_default(default.into());
            f.set_precision(1);
            f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
        }),
    )
}

fn add_signed_percent_param(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: f32,
) -> Result<(), ae::Error> {
    add_percent_param(params, id, name, default, -100.0, 100.0)
}

fn add_px_param(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: f32,
    min: f32,
    max: f32,
) -> Result<(), ae::Error> {
    add_px_param_with_precision(params, id, name, default, min, max, 2)
}

fn add_px_param_with_precision(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: f32,
    min: f32,
    max: f32,
    precision: i16,
) -> Result<(), ae::Error> {
    params.add(
        id,
        name,
        ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(min);
            f.set_valid_max(max);
            f.set_slider_min(min);
            f.set_slider_max(max);
            f.set_default(default.into());
            f.set_precision(precision);
        }),
    )
}

fn add_px_param_with_slider_range(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: f32,
    min: f32,
    max: f32,
    slider_min: f32,
    slider_max: f32,
) -> Result<(), ae::Error> {
    params.add(
        id,
        name,
        ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(min);
            f.set_valid_max(max);
            f.set_slider_min(slider_min);
            f.set_slider_max(slider_max);
            f.set_default(default.into());
            f.set_precision(2);
        }),
    )
}

fn add_offset_param(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: f32,
) -> Result<(), ae::Error> {
    add_px_param_with_slider_range(params, id, name, default, -1000.0, 1000.0, -100.0, 100.0)
}

fn add_hidden_px_param(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: f32,
) -> Result<(), ae::Error> {
    params.add_with_flags(
        id,
        name,
        ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(-1_000_000.0);
            f.set_valid_max(1_000_000.0);
            f.set_slider_min(-1000.0);
            f.set_slider_max(1000.0);
            f.set_default(default.into());
            f.set_precision(2);
        }),
        ae::ParamFlag::empty(),
        ae::ParamUIFlags::NO_ECW_UI,
    )
}

fn add_angle_param(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: f32,
) -> Result<(), ae::Error> {
    params.add(
        id,
        name,
        ae::AngleDef::setup(|f| {
            f.set_default(default);
            f.set_value(f.default());
        }),
    )
}

fn add_color_param(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: ae::Pixel8,
) -> Result<(), ae::Error> {
    params.add(
        id,
        name,
        ae::ColorDef::setup(|f| {
            f.set_default(default);
        }),
    )
}

fn rgb8(red: u8, green: u8, blue: u8) -> ae::Pixel8 {
    ae::Pixel8 {
        alpha: 255,
        red,
        green,
        blue,
    }
}

fn pixel_to_rgb(pixel: ae::Pixel8) -> [f32; 3] {
    [
        pixel.red as f32 / 255.0,
        pixel.green as f32 / 255.0,
        pixel.blue as f32 / 255.0,
    ]
}

fn get_params(params: &ae::Parameters<Params>) -> Result<EffectParams, ae::Error> {
    let paper_base = pixel_to_rgb(params.get(Params::PaperColor)?.as_color()?.value());
    let brightness = percent(params, Params::PaperBrightness, 0.0, 1.0)?;
    let conversion_mode =
        normalize_conversion_mode(params.get(Params::ConversionMode)?.as_popup()?.value() as i32);
    let ink_colors = match conversion_mode {
        CONVERSION_CUSTOM => [
            pixel_to_rgb(params.get(Params::InkColorCyan)?.as_color()?.value()),
            pixel_to_rgb(params.get(Params::InkColorMagenta)?.as_color()?.value()),
            pixel_to_rgb(params.get(Params::InkColorYellow)?.as_color()?.value()),
            pixel_to_rgb(params.get(Params::InkColorBlack)?.as_color()?.value()),
        ],
        CONVERSION_ILLUSTRATOR => ILLUSTRATOR_INK_COLORS,
        _ => CUSTOM_INK_COLORS,
    };

    Ok(EffectParams {
        view_mode: normalize_view(params.get(Params::ViewMode)?.as_popup()?.value() as i32),
        preserve_alpha: params.get(Params::PreserveAlpha)?.as_checkbox()?.value(),
        blend_original: percent(params, Params::BlendOriginal, 0.0, 1.0)?,
        ink_amounts: [
            percent(params, Params::CyanAmount, 0.0, 1.0)?,
            percent(params, Params::MagentaAmount, 0.0, 1.0)?,
            percent(params, Params::YellowAmount, 0.0, 1.0)?,
            percent(params, Params::BlackAmount, 0.0, 1.0)?,
        ],
        paper: apply_paper_controls(paper_base, brightness),
        offsets: [
            [
                float_param(params, Params::CyanOffsetX)?
                    .round()
                    .clamp(-1000.0, 1000.0),
                float_param(params, Params::CyanOffsetY)?
                    .round()
                    .clamp(-1000.0, 1000.0),
            ],
            [
                float_param(params, Params::MagentaOffsetX)?
                    .round()
                    .clamp(-1000.0, 1000.0),
                float_param(params, Params::MagentaOffsetY)?
                    .round()
                    .clamp(-1000.0, 1000.0),
            ],
            [
                float_param(params, Params::YellowOffsetX)?
                    .round()
                    .clamp(-1000.0, 1000.0),
                float_param(params, Params::YellowOffsetY)?
                    .round()
                    .clamp(-1000.0, 1000.0),
            ],
            [
                float_param(params, Params::BlackOffsetX)?
                    .round()
                    .clamp(-1000.0, 1000.0),
                float_param(params, Params::BlackOffsetY)?
                    .round()
                    .clamp(-1000.0, 1000.0),
            ],
        ],
        random_enabled: params.get(Params::RandomEnable)?.as_checkbox()?.value(),
        random_seed: float_param(params, Params::RandomSeed)?
            .round()
            .clamp(0.0, 2_147_483_647.0) as u32,
        random_amount: [
            float_param(params, Params::RandomAmountX)?
                .round()
                .clamp(0.0, 1000.0),
            float_param(params, Params::RandomAmountY)?
                .round()
                .clamp(0.0, 1000.0),
        ],
        random_affect: [
            params.get(Params::RandomAffectCyan)?.as_checkbox()?.value(),
            params
                .get(Params::RandomAffectMagenta)?
                .as_checkbox()?
                .value(),
            params
                .get(Params::RandomAffectYellow)?
                .as_checkbox()?
                .value(),
            params
                .get(Params::RandomAffectBlack)?
                .as_checkbox()?
                .value(),
        ],
        halftone_enabled: params.get(Params::HalftoneEnable)?.as_checkbox()?.value(),
        halftone_frequency: float_param(params, Params::HalftoneFrequency)?.clamp(1.0, 1000.0),
        halftone_unit: normalize_halftone_unit(
            params.get(Params::HalftoneUnit)?.as_popup()?.value() as i32,
        ),
        halftone_shape: normalize_dot_shape(
            params.get(Params::HalftoneShape)?.as_popup()?.value() as i32
        ),
        halftone_dot_gain: percent(params, Params::HalftoneDotGain, -1.0, 1.0)?,
        halftone_softness: percent(params, Params::HalftoneSoftness, 0.0, 1.0)?,
        halftone_angles: [
            angle_param(params, Params::HalftoneCyanAngle)?.rem_euclid(180.0),
            angle_param(params, Params::HalftoneMagentaAngle)?.rem_euclid(180.0),
            angle_param(params, Params::HalftoneYellowAngle)?.rem_euclid(180.0),
            angle_param(params, Params::HalftoneBlackAngle)?.rem_euclid(180.0),
        ],
        halftone_offset: {
            let point = point_param(params, Params::HalftoneOffsetPoint)?;
            let legacy = [
                float_param(params, Params::HalftoneOffsetX).unwrap_or(0.0),
                float_param(params, Params::HalftoneOffsetY).unwrap_or(0.0),
            ];
            [point[0] + legacy[0], point[1] + legacy[1]]
        },
        backend: normalize_backend(params.get(Params::Backend)?.as_popup()?.value() as i32),
        quality: normalize_quality(params.get(Params::Quality)?.as_popup()?.value() as i32),
        sampling_mode: normalize_sampling(params.get(Params::Sampling)?.as_popup()?.value() as i32),
        edge_mode: normalize_edge_mode(params.get(Params::EdgeMode)?.as_popup()?.value() as i32),
        expand_bounds: params.get(Params::ExpandBounds)?.as_checkbox()?.value(),
        conversion_mode,
        ink_colors,
        transparent_mode: !params.get(Params::TransparentMode)?.as_checkbox()?.value(),
    })
}

fn float_param(params: &ae::Parameters<Params>, param: Params) -> Result<f32, ae::Error> {
    Ok(params.get(param)?.as_float_slider()?.value() as f32)
}

fn angle_param(params: &ae::Parameters<Params>, param: Params) -> Result<f32, ae::Error> {
    let param_ref = params.get(param)?;
    let angle = param_ref.as_angle()?;
    angle
        .float_value()
        .map(|value| value as f32)
        .or_else(|_| Ok(angle.value()))
}

fn point_param(params: &ae::Parameters<Params>, param: Params) -> Result<[f32; 2], ae::Error> {
    let point = params.get(param)?.as_point()?.float_value()?;
    Ok([point.x as f32, point.y as f32])
}

fn percent(
    params: &ae::Parameters<Params>,
    param: Params,
    min: f32,
    max: f32,
) -> Result<f32, ae::Error> {
    Ok((float_param(params, param)? / 100.0).clamp(min, max))
}

fn normalize_view(value: i32) -> i32 {
    match value {
        VIEW_CYAN | VIEW_MAGENTA | VIEW_YELLOW | VIEW_BLACK | VIEW_INK_COVERAGE | VIEW_SPLIT => {
            value
        }
        _ => VIEW_COMPOSITE,
    }
}

fn normalize_halftone_unit(value: i32) -> i32 {
    match value {
        HALFTONE_UNIT_LPI => value,
        _ => HALFTONE_UNIT_PIXELS,
    }
}

fn normalize_dot_shape(value: i32) -> i32 {
    match value {
        DOT_SQUARE | DOT_LINE | DOT_DIAMOND => value,
        _ => DOT_CIRCLE,
    }
}

fn normalize_backend(value: i32) -> i32 {
    match value {
        BACKEND_CPU | BACKEND_GPU => value,
        _ => BACKEND_AUTO,
    }
}

fn normalize_quality(value: i32) -> i32 {
    match value {
        QUALITY_DRAFT => value,
        _ => QUALITY_FULL,
    }
}

fn normalize_sampling(value: i32) -> i32 {
    match value {
        SAMPLING_BILINEAR | SAMPLING_NEAREST => value,
        _ => SAMPLING_BILINEAR,
    }
}

fn normalize_edge_mode(value: i32) -> i32 {
    match value {
        EDGE_CLAMP | EDGE_MIRROR => value,
        _ => EDGE_TRANSPARENT,
    }
}

fn normalize_conversion_mode(value: i32) -> i32 {
    match value {
        CONVERSION_ILLUSTRATOR | CONVERSION_CUSTOM => value,
        _ => CONVERSION_SIMPLE,
    }
}

fn apply_paper_controls(rgb: [f32; 3], brightness: f32) -> [f32; 3] {
    [
        (rgb[0] * brightness).clamp(0.0, 1.0),
        (rgb[1] * brightness).clamp(0.0, 1.0),
        (rgb[2] * brightness).clamp(0.0, 1.0),
    ]
}

fn center_halftone_offset(ep: &mut EffectParams, width: usize, height: usize) {
    ep.halftone_offset[0] = width as f32 * 0.5 - ep.halftone_offset[0];
    ep.halftone_offset[1] = height as f32 * 0.5 - ep.halftone_offset[1];
}

fn smart_pre_render(
    in_data: &ae::InData,
    extra: &mut ae::pf::PreRenderExtra,
    params: &ae::Parameters<Params>,
) -> Result<(), ae::Error> {
    let req = extra.output_request();
    let cb = extra.callbacks();
    let in_result = cb.checkout_layer(
        0,
        0,
        &req,
        in_data.current_time(),
        in_data.time_step(),
        in_data.time_scale(),
    )?;

    let req_rect: ae::Rect = req.rect.into();
    let max_res: ae::Rect = in_result.max_result_rect.into();

    if get_params(params)
        .map(|ep| ep.expand_bounds)
        .unwrap_or(true)
    {
        extra.set_result_rect(max_res);
        extra.set_max_result_rect(max_res);
    } else {
        extra.set_result_rect(req_rect);
        extra.set_max_result_rect(req_rect);
    }
    extra.set_gpu_render_possible(gpu_prerender_possible(extra));
    Ok(())
}

fn smart_render(
    extra: &ae::pf::SmartRenderExtra,
    params: &ae::Parameters<Params>,
) -> Result<(), ae::Error> {
    let mut ep = get_params(params)?;
    let cb = extra.callbacks();
    let input_world = cb.checkout_layer_pixels(0)?.ok_or(ae::Error::Generic)?;
    let result = (|| {
        let mut output_world = cb.checkout_output()?.ok_or(ae::Error::Generic)?;
        let src = layer_to_frame(&input_world);
        center_halftone_offset(&mut ep, src.w, src.h);
        let out = render_cmyk_press(&src, &ep);
        frame_to_layer(&out, &mut output_world);
        Ok(())
    })();
    cb.checkin_layer_pixels(0)?;
    result
}

fn gpu_prerender_possible(extra: &ae::pf::PreRenderExtra) -> bool {
    #[cfg(target_os = "macos")]
    {
        extra.what_gpu() == ae::GpuFramework::Metal && extra.bit_depth() == 32
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = extra;
        false
    }
}

#[cfg(target_os = "macos")]
fn smart_render_gpu(
    extra: &ae::pf::SmartRenderExtra,
    params: &ae::Parameters<Params>,
    in_data: &ae::InData,
) -> Result<(), ae::Error> {
    if extra.what_gpu() != ae::GpuFramework::Metal || extra.bit_depth() != 32 {
        return Err(ae::Error::Generic);
    }

    let mut ep = get_params(params)?;
    if ep.backend == BACKEND_CPU {
        return Err(ae::Error::Generic);
    }

    let state = extra
        .gpu_data::<metal_gpu::MetalState>()
        .ok_or(ae::Error::Generic)?;
    let cb = extra.callbacks();
    let mut input_world = cb.checkout_layer_pixels(0)?.ok_or(ae::Error::Generic)?;
    let result = (|| {
        let mut output_world = cb.checkout_output()?.ok_or(ae::Error::Generic)?;
        center_halftone_offset(
            &mut ep,
            input_world.width() as usize,
            input_world.height() as usize,
        );
        state.render(in_data, &mut input_world, &mut output_world, &ep)
    })();
    cb.checkin_layer_pixels(0)?;
    result
}

#[cfg(not(target_os = "macos"))]
fn smart_render_gpu(
    _extra: &ae::pf::SmartRenderExtra,
    _params: &ae::Parameters<Params>,
    _in_data: &ae::InData,
) -> Result<(), ae::Error> {
    Err(ae::Error::Generic)
}

#[cfg(target_os = "macos")]
fn gpu_device_setup(
    in_data: &ae::InData,
    out_data: &mut ae::OutData,
    extra: &mut ae::pf::GpuDeviceSetupExtra,
) -> Result<(), ae::Error> {
    if extra.what_gpu() != ae::GpuFramework::Metal {
        return Ok(());
    }

    match metal_gpu::MetalState::new(in_data, extra.device_index()) {
        Ok(state) => {
            extra.set_gpu_data(state);
            out_data.set_out_flag2(ae::OutFlags2::SupportsGpuRenderF32, true);
        }
        Err(e) => {
            // Log but do not propagate — AE will fall back to CPU rendering.
            // Returning Err here causes AE to show "GPU device setup failed".
            eprintln!("CMYK Press: GPU setup failed ({e:?}), falling back to CPU");
        }
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn gpu_device_setup(
    _in_data: &ae::InData,
    _out_data: &mut ae::OutData,
    _extra: &mut ae::pf::GpuDeviceSetupExtra,
) -> Result<(), ae::Error> {
    Ok(())
}

#[cfg(target_os = "macos")]
fn gpu_device_setdown(extra: &mut ae::pf::GpuDeviceSetdownExtra) -> Result<(), ae::Error> {
    if extra.what_gpu() == ae::GpuFramework::Metal {
        extra.destroy_gpu_data::<metal_gpu::MetalState>();
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn gpu_device_setdown(_extra: &mut ae::pf::GpuDeviceSetdownExtra) -> Result<(), ae::Error> {
    Ok(())
}

fn layer_to_frame(layer: &ae::Layer) -> Frame {
    let w = layer.width() as usize;
    let h = layer.height() as usize;
    let mut pixels = vec![Rgba::transparent(); w * h];

    match layer.bit_depth() {
        16 => {
            let scale = 1.0 / ae::MAX_CHANNEL16 as f32;
            for y in 0..h {
                for x in 0..w {
                    let p = layer.as_pixel16(x, y);
                    pixels[y * w + x] = rgba_from_straight(
                        p.red as f32 * scale,
                        p.green as f32 * scale,
                        p.blue as f32 * scale,
                        p.alpha as f32 * scale,
                    );
                }
            }
        }
        32 => {
            for y in 0..h {
                for x in 0..w {
                    let p = layer.as_pixel32(x, y);
                    pixels[y * w + x] = rgba_from_straight(p.red, p.green, p.blue, p.alpha);
                }
            }
        }
        _ => {
            let stride = layer.buffer_stride();
            let buf = layer.buffer();
            for y in 0..h {
                let src_off = y * stride;
                let row_len = w * 4;
                if src_off + row_len > buf.len() {
                    continue;
                }
                let row = &buf[src_off..src_off + row_len];
                for x in 0..w {
                    let off = x * 4;
                    pixels[y * w + x] = rgba_from_straight(
                        row[off + 1] as f32 / 255.0,
                        row[off + 2] as f32 / 255.0,
                        row[off + 3] as f32 / 255.0,
                        row[off] as f32 / 255.0,
                    );
                }
            }
        }
    }

    Frame { pixels, w, h }
}

fn frame_to_layer(frame: &Frame, layer: &mut ae::Layer) {
    let w = frame.w.min(layer.width() as usize);
    let h = frame.h.min(layer.height() as usize);

    match layer.bit_depth() {
        16 => {
            for y in 0..h {
                for x in 0..w {
                    let px = frame.pixels[y * frame.w + x];
                    let rgb = straight_rgb_from_rgba(px);
                    let out = layer.as_pixel16_mut(x, y);
                    out.alpha = to_u16(px.a);
                    out.red = to_u16(rgb[0]);
                    out.green = to_u16(rgb[1]);
                    out.blue = to_u16(rgb[2]);
                }
            }
        }
        32 => {
            for y in 0..h {
                for x in 0..w {
                    let px = frame.pixels[y * frame.w + x];
                    let rgb = straight_rgb_from_rgba(px);
                    let out = layer.as_pixel32_mut(x, y);
                    out.alpha = px.a.clamp(0.0, 1.0);
                    out.red = rgb[0];
                    out.green = rgb[1];
                    out.blue = rgb[2];
                }
            }
        }
        _ => {
            let stride = layer.buffer_stride();
            let buf = layer.buffer_mut();
            for y in 0..h {
                let dst_off = y * stride;
                let row_len = w * 4;
                if dst_off + row_len > buf.len() {
                    continue;
                }
                let row = &mut buf[dst_off..dst_off + row_len];
                for x in 0..w {
                    let px = frame.pixels[y * frame.w + x];
                    let rgb = straight_rgb_from_rgba(px);
                    let off = x * 4;
                    row[off] = to_u8(px.a);
                    row[off + 1] = to_u8(rgb[0]);
                    row[off + 2] = to_u8(rgb[1]);
                    row[off + 3] = to_u8(rgb[2]);
                }
            }
        }
    }
}

fn to_u16(v: f32) -> u16 {
    (v.clamp(0.0, 1.0) * ae::MAX_CHANNEL16 as f32 + 0.5) as u16
}

#[cfg(target_os = "macos")]
mod metal_gpu {
    use super::*;
    use std::ffi::c_void;

    #[repr(C)]
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct MetalPlatePlan {
        shift: [f32; 2],
        pivot: [f32; 2],
        sin_v: f32,
        cos_v: f32,
        cell: f32,
        inv_cell: f32,
        edge_width: f32,
        _pad: f32,
    }

    #[repr(C, align(16))]
    #[derive(Clone, Copy)]
    pub(super) struct MetalParams {
        width: u32,
        height: u32,
        view_mode: i32,
        preserve_alpha: u32,
        transparent_mode: u32,
        blend_original: f32,
        halftone_enabled: u32,
        halftone_shape: i32,
        halftone_dot_gain: f32,
        halftone_softness: f32,
        quality: i32,
        sampling_mode: i32,
        edge_mode: i32,
        conversion_mode: i32,
        _pad0: [i32; 2],
        paper: [f32; 4],
        ink_amounts: [f32; 4],
        halftone_offset: [f32; 4],
        // ink_colors[plate] = [r, g, b, 0.0]
        ink_colors: [[f32; 4]; PLATE_COUNT],
        plates: [MetalPlatePlan; PLATE_COUNT],
    }

    impl MetalParams {
        pub(super) fn new(ep: &EffectParams, width: usize, height: usize) -> Self {
            let plan = RenderPlan::new(ep, width, height);
            Self {
                width: width as u32,
                height: height as u32,
                view_mode: ep.view_mode,
                preserve_alpha: ep.preserve_alpha as u32,
                transparent_mode: ep.transparent_mode as u32,
                blend_original: ep.blend_original,
                halftone_enabled: ep.halftone_enabled as u32,
                halftone_shape: ep.halftone_shape,
                halftone_dot_gain: ep.halftone_dot_gain,
                halftone_softness: ep.halftone_softness,
                quality: ep.quality,
                sampling_mode: ep.sampling_mode,
                edge_mode: ep.edge_mode,
                conversion_mode: ep.conversion_mode,
                _pad0: [0; 2],
                paper: [ep.paper[0], ep.paper[1], ep.paper[2], 1.0],
                ink_amounts: ep.ink_amounts,
                halftone_offset: [ep.halftone_offset[0], ep.halftone_offset[1], 0.0, 0.0],
                ink_colors: std::array::from_fn(|plate| {
                    [
                        ep.ink_colors[plate][0],
                        ep.ink_colors[plate][1],
                        ep.ink_colors[plate][2],
                        0.0,
                    ]
                }),
                plates: std::array::from_fn(|plate| {
                    let p = plan.plates[plate];
                    MetalPlatePlan {
                        shift: p.shift,
                        pivot: p.pivot,
                        sin_v: p.sin,
                        cos_v: p.cos,
                        cell: p.cell,
                        inv_cell: p.inv_cell,
                        edge_width: p.edge_width,
                        _pad: 0.0,
                    }
                }),
            }
        }
    }

    #[cfg(test)]
    pub(super) fn metal_params_layout_offsets() -> (usize, usize, usize, usize) {
        (
            std::mem::offset_of!(MetalParams, paper),
            std::mem::offset_of!(MetalParams, ink_amounts),
            std::mem::offset_of!(MetalParams, halftone_offset),
            std::mem::offset_of!(MetalParams, plates),
        )
    }

    pub struct MetalState {
        pipeline: metal::ComputePipelineState,
        fast_pipeline: metal::ComputePipelineState,
        // Own the command queue so render() doesn't need to call device_info again.
        // This avoids failures when AE's GPUDevice suite is unavailable at render time.
        command_queue: metal::CommandQueue,
    }

    impl MetalState {
        pub fn new(in_data: &ae::InData, device_index: usize) -> Result<Self, ae::Error> {
            // Prefer the device AE registered for this index; fall back to system default.
            let (device, queue_opt) = Self::device_and_queue_from_ae(in_data, device_index);
            let device = device.ok_or(ae::Error::Generic)?;

            let options = metal::CompileOptions::new();
            options.set_fast_math_enabled(false);
            let library = device
                .new_library_with_source(METAL_SHADER, &options)
                .map_err(|e| {
                    eprintln!("CMYK Press: Metal shader compile error: {e}");
                    ae::Error::Generic
                })?;
            let function = library.get_function("cmyk_press", None).map_err(|e| {
                eprintln!("CMYK Press: Metal get_function error: {e}");
                ae::Error::Generic
            })?;
            let pipeline = device
                .new_compute_pipeline_state_with_function(&function)
                .map_err(|e| {
                    eprintln!("CMYK Press: Metal pipeline error: {e}");
                    ae::Error::Generic
                })?;
            let fast_function = library.get_function("cmyk_press_fast", None).map_err(|e| {
                eprintln!("CMYK Press: Metal get fast_function error: {e}");
                ae::Error::Generic
            })?;
            let fast_pipeline = device
                .new_compute_pipeline_state_with_function(&fast_function)
                .map_err(|e| {
                    eprintln!("CMYK Press: Metal fast pipeline error: {e}");
                    ae::Error::Generic
                })?;

            // Use AE's command queue when available; otherwise create our own.
            let command_queue = match queue_opt {
                Some(q) => q,
                None => device.new_command_queue(),
            };

            Ok(Self {
                pipeline,
                fast_pipeline,
                command_queue,
            })
        }

        /// Returns (device, command_queue) from AE's GPU device info.
        /// Falls back to system_default() device with no queue if AE info is unavailable.
        fn device_and_queue_from_ae(
            in_data: &ae::InData,
            device_index: usize,
        ) -> (Option<metal::Device>, Option<metal::CommandQueue>) {
            use metal::foreign_types::ForeignType;

            let info = match device_info(in_data, device_index) {
                Ok(i) => i,
                Err(_) => {
                    // GPUDevice suite unavailable at this call site — use system default.
                    return (metal::Device::system_default(), None);
                }
            };

            if info.compatibleB == 0 || info.devicePV.is_null() {
                // AE says this device is not compatible; try system default anyway.
                return (metal::Device::system_default(), None);
            }

            // Match AE's device pointer against the enumerated Metal devices so we
            // get a properly retained Device handle.
            let ae_ptr = info.devicePV as *mut metal::MTLDevice;
            let device = metal::Device::all()
                .into_iter()
                .find(|d| std::ptr::eq(d.as_ptr(), ae_ptr));

            // Wrap AE's command queue pointer if it is valid.
            let queue = if !info.command_queuePV.is_null() {
                // SAFETY: command_queuePV is a valid MTLCommandQueue* owned by AE.
                // We wrap it without taking ownership (ManuallyDrop equivalent via
                // foreign_types retain semantics — metal::CommandQueue retains on wrap).
                let queue_ref =
                    unsafe { &*(info.command_queuePV as *const metal::CommandQueueRef) };
                // Clone by creating a new queue on the same device to avoid double-free.
                device
                    .as_ref()
                    .map(|d| d.new_command_queue())
                    .or_else(|| Some(queue_ref.device().new_command_queue()))
            } else {
                None
            };

            (device, queue)
        }

        pub fn render(
            &self,
            in_data: &ae::InData,
            input_world: &mut ae::Layer,
            output_world: &mut ae::Layer,
            ep: &EffectParams,
        ) -> Result<(), ae::Error> {
            let gpu = ae::pf::suites::GPUDevice::new()?;
            let effect_ref = in_data.effect_ref();
            let input_texture = gpu.gpu_world_data(effect_ref, &mut *input_world)?;
            let output_texture = gpu.gpu_world_data(effect_ref, &mut *output_world)?;
            if input_texture.is_null() || output_texture.is_null() {
                return Err(ae::Error::Generic);
            }

            let queue = &self.command_queue;
            let input_texture = unsafe { &*(input_texture as *const metal::TextureRef) };
            let output_texture = unsafe { &*(output_texture as *const metal::TextureRef) };
            let width = output_world.width();
            let height = output_world.height();
            let params = MetalParams::new(ep, width, height);

            let command_buffer = queue.new_command_buffer();
            let encoder = command_buffer.new_compute_command_encoder();
            let pipeline = if can_use_composite_single_sample_path(ep) {
                &self.fast_pipeline
            } else {
                &self.pipeline
            };
            encoder.set_compute_pipeline_state(pipeline);
            encoder.set_bytes(
                0,
                std::mem::size_of::<MetalParams>() as u64,
                &params as *const _ as *const c_void,
            );
            encoder.set_texture(0, Some(input_texture));
            encoder.set_texture(1, Some(output_texture));
            encoder.dispatch_threads(
                metal::MTLSize::new(width as u64, height as u64, 1),
                self.threads_per_group(),
            );
            encoder.end_encoding();
            command_buffer.commit();
            command_buffer.wait_until_completed();

            if command_buffer.status() == metal::MTLCommandBufferStatus::Completed {
                Ok(())
            } else {
                Err(ae::Error::Generic)
            }
        }

        fn threads_per_group(&self) -> metal::MTLSize {
            let width = self.pipeline.thread_execution_width().max(1).min(16);
            let max_total = self.pipeline.max_total_threads_per_threadgroup().max(width);
            let height = (max_total / width).max(1).min(16);
            metal::MTLSize::new(width, height, 1)
        }
    }

    fn device_info(
        in_data: &ae::InData,
        device_index: usize,
    ) -> Result<after_effects::sys::PF_GPUDeviceInfo, ae::Error> {
        ae::pf::suites::GPUDevice::new()?.device_info(in_data.effect_ref(), device_index)
    }

    pub(super) const METAL_SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

constant int VIEW_COMPOSITE = 1;
constant int VIEW_CYAN = 2;
constant int VIEW_MAGENTA = 3;
constant int VIEW_YELLOW = 4;
constant int VIEW_BLACK = 5;
constant int VIEW_INK_COVERAGE = 6;
constant int VIEW_SPLIT = 7;
constant int EDGE_CLAMP = 2;
constant int EDGE_MIRROR = 3;
constant int SAMPLING_BILINEAR = 1;
constant int SAMPLING_NEAREST = 2;
constant int CONVERSION_ILLUSTRATOR = 2;
constant int CONVERSION_CUSTOM = 3;

struct PlatePlan {
    float2 shift;
    float2 pivot;
    float sin_v;
    float cos_v;
    float cell;
    float inv_cell;
    float edge_width;
    float pad;
};

struct Params {
    uint width;
    uint height;
    int view_mode;
    uint preserve_alpha;
    uint transparent_mode;
    float blend_original;
    uint halftone_enabled;
    int halftone_shape;
    float halftone_dot_gain;
    float halftone_softness;
    int quality;
    int sampling_mode;
    int edge_mode;
    int conversion_mode;
    int2 pad0;
    float4 paper;
    float4 ink_amounts;
    float4 halftone_offset;
    float4 ink_colors[4];
    PlatePlan plates[4];
};

constexpr sampler nearest_sampler(coord::pixel, address::clamp_to_edge, filter::nearest);
constexpr sampler linear_sampler(coord::pixel, address::clamp_to_edge, filter::linear);

static inline float4 premultiply_sample(float4 px) {
    px = saturate(px);
    return float4(px.rgb * px.a, px.a);
}

static inline float4 sample_pixel(texture2d<float, access::sample> src, uint2 pos) {
    return premultiply_sample(src.read(pos));
}

static inline float mirror_coord(float value, uint len) {
    if (len <= 1) {
        return 0.0;
    }
    float max_coord = float(len - 1);
    float period = max_coord * 2.0;
    float wrapped = fmod(fmod(value, period) + period, period);
    return wrapped > max_coord ? period - wrapped : wrapped;
}

static inline float4 sample_nearest(texture2d<float, access::sample> src, float2 pos, uint width, uint height, int edge_mode) {
    if (width == 0 || height == 0) {
        return float4(0.0);
    }
    if (edge_mode == EDGE_CLAMP) {
        pos = clamp(pos, float2(0.0), float2(float(width - 1), float(height - 1)));
    } else if (edge_mode == EDGE_MIRROR) {
        pos = float2(mirror_coord(pos.x, width), mirror_coord(pos.y, height));
    } else if (pos.x < 0.0 || pos.y < 0.0 || pos.x > float(width - 1) || pos.y > float(height - 1)) {
        return float4(0.0);
    }
    return saturate(src.sample(nearest_sampler, pos));
}

static inline float4 sample_bilinear(texture2d<float, access::sample> src, float2 pos, uint width, uint height, int edge_mode) {
    if (width == 0 || height == 0) {
        return float4(0.0);
    }
    if (edge_mode == EDGE_CLAMP) {
        pos = clamp(pos, float2(0.0), float2(float(width - 1), float(height - 1)));
    } else if (edge_mode == EDGE_MIRROR) {
        pos = float2(mirror_coord(pos.x, width), mirror_coord(pos.y, height));
    } else if (pos.x < 0.0 || pos.y < 0.0 || pos.x > float(width - 1) || pos.y > float(height - 1)) {
        return float4(0.0);
    }
    uint x0 = uint(floor(pos.x));
    uint y0 = uint(floor(pos.y));
    uint x1 = min(x0 + 1, width - 1);
    uint y1 = min(y0 + 1, height - 1);
    float tx = pos.x - float(x0);
    float ty = pos.y - float(y0);

    float4 a = sample_pixel(src, uint2(x0, y0));
    float4 b = sample_pixel(src, uint2(x1, y0));
    float4 c = sample_pixel(src, uint2(x0, y1));
    float4 d = sample_pixel(src, uint2(x1, y1));
    return mix(mix(a, b, tx), mix(c, d, tx), ty);
}

static inline bool use_nearest_sampling(constant Params& params) {
    if (params.sampling_mode == SAMPLING_NEAREST) {
        return true;
    }
    return false;
}

static inline float3 unpremultiply_rgb(float4 px) {
    if (px.a <= 0.0001) {
        return float3(0.0);
    }
    return saturate(px.rgb / px.a);
}

static inline float apply_dot_gain(float value, float dot_gain) {
    return clamp(clamp(value, 0.0, 1.0) + clamp(dot_gain, -1.0, 1.0) * 0.25, 0.0, 1.0);
}

static inline float2 halftone_rotated_position(float2 xy, constant PlatePlan& plan, constant Params& params) {
    float2 p = xy + plan.shift + params.halftone_offset.xy - plan.pivot;
    float rx = p.x * plan.cos_v + p.y * plan.sin_v;
    float ry = -p.x * plan.sin_v + p.y * plan.cos_v;
    return float2(rx, ry);
}

static inline float2 dot_cell_position_from_rotated(float2 rotated, constant PlatePlan& plan) {
    float2 scaled = rotated * plan.inv_cell;
    return scaled - floor(scaled) - 0.5;
}

static inline float2 halftone_sample_position_from_rotated(float2 rotated, constant PlatePlan& plan, constant Params& params) {
    float2 center = floor(rotated * plan.inv_cell) * plan.cell + plan.cell * 0.5;
    float2 unrotated = float2(center.x * plan.cos_v - center.y * plan.sin_v,
                              center.x * plan.sin_v + center.y * plan.cos_v);
    return unrotated + plan.pivot - params.halftone_offset.xy;
}

struct HalftonePoint {
    float2 sample_pos;
    float2 cell;
};

static inline HalftonePoint halftone_point(float2 xy, constant PlatePlan& plan, constant Params& params) {
    float2 rotated = halftone_rotated_position(xy, plan, params);
    HalftonePoint point;
    point.sample_pos = halftone_sample_position_from_rotated(rotated, plan, params);
    point.cell = dot_cell_position_from_rotated(rotated, plan);
    return point;
}

static inline float dot_radius(float value) {
    return sqrt(clamp(value, 0.0, 1.0)) * 0.5;
}

static inline float dot_edge_width(float cell, float softness) {
    float cell_aa = 0.5 / max(cell, 1.0);
    return max(cell_aa + clamp(softness, 0.0, 1.0) * 0.12, 0.0001);
}

static inline float smooth_circle(float dist, float radius, float edge) {
    float t = clamp((radius + edge - dist) / (2.0 * edge), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

static inline float4 rgb_to_cmyk_controls(float3 rgb, constant Params& params) {
    rgb = saturate(rgb);
    float k = 1.0 - max(max(rgb.r, rgb.g), rgb.b);
    if (k >= 0.999) {
        return float4(0.0, 0.0, 0.0, k);
    }
    float denom = max(1.0 - k, 0.0001);
    return float4(clamp((1.0 - rgb.r - k) / denom, 0.0, 1.0),
                  clamp((1.0 - rgb.g - k) / denom, 0.0, 1.0),
                  clamp((1.0 - rgb.b - k) / denom, 0.0, 1.0),
                  k);
}

static inline float rgb_to_cmyk_plate(float3 rgb, uint plate) {
    rgb = saturate(rgb);
    float k = 1.0 - max(max(rgb.r, rgb.g), rgb.b);
    if (plate == 3) {
        return k;
    }
    if (k >= 0.999) {
        return 0.0;
    }
    float denom = max(1.0 - k, 0.0001);
    float channel = plate == 0 ? rgb.r : (plate == 1 ? rgb.g : rgb.b);
    return clamp((1.0 - channel - k) / denom, 0.0, 1.0);
}

static inline float dot_shape_distance(float2 cell, int shape) {
    if (shape == 2) { // DOT_SQUARE
        return max(abs(cell.x), abs(cell.y));
    } else if (shape == 3) { // DOT_LINE
        return abs(cell.y);
    } else if (shape == 4) { // DOT_DIAMOND
        return abs(cell.x) + abs(cell.y);
    } else { // DOT_CIRCLE (default)
        return length(cell);
    }
}

static inline float halftone_coverage_from_cell(float2 cell, float value, constant PlatePlan& plan, constant Params& params) {
    value = apply_dot_gain(value, params.halftone_dot_gain);
    if (value <= 0.0) return 0.0;
    float dist = dot_shape_distance(cell, params.halftone_shape);
    float radius = dot_radius(value);
    return smooth_circle(dist, radius, plan.edge_width);
}

static inline float3 composite(float4 inks, constant Params& params) {
    if (params.conversion_mode == CONVERSION_ILLUSTRATOR || params.conversion_mode == CONVERSION_CUSTOM) {
        // Illustrator CMYK: multiply each ink onto the running result
        float3 result = params.paper.rgb;
        for (uint plate = 0; plate < 4; ++plate) {
            float t = clamp(inks[plate], 0.0, 1.0);
            if (t <= 0.0) continue;
            float3 multiplied = result * params.ink_colors[plate].rgb;
            result = result + (multiplied - result) * t;
        }
        return saturate(result);
    }
    // Simple subtractive
    return saturate(float3(params.paper.r * clamp(1.0 - inks.x, 0.0, 1.0) * clamp(1.0 - inks.w, 0.0, 1.0),
                           params.paper.g * clamp(1.0 - inks.y, 0.0, 1.0) * clamp(1.0 - inks.w, 0.0, 1.0),
                           params.paper.b * clamp(1.0 - inks.z, 0.0, 1.0) * clamp(1.0 - inks.w, 0.0, 1.0)));
}

static inline float smooth_step(float t) {
    t = clamp(t, 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

static inline float4 composite_preview_to_output(float3 rgb, float alpha, constant Params& params) {
    alpha = clamp(alpha, 0.0, 1.0);
    if (params.transparent_mode != 0 && alpha > 0.0) {
        rgb = saturate(rgb);
        float3 paper = saturate(params.paper.rgb);
        float3 delta = rgb - paper;
        float3 denom_dark = max(paper, float3(0.0001));
        float3 denom_light = max(float3(1.0) - paper, float3(0.0001));
        float3 channel_alpha = select(abs(delta) / denom_light, abs(delta) / denom_dark, delta < 0.0);
        float matte_alpha = clamp(max(max(channel_alpha.r, channel_alpha.g), channel_alpha.b), 0.0, 1.0);
        if (matte_alpha <= 0.0001) {
            return float4(0.0);
        }
        rgb = saturate((rgb - paper * (1.0 - matte_alpha)) / matte_alpha);
        alpha *= smooth_step(matte_alpha);
    }
    return float4(saturate(rgb), alpha);
}

kernel void cmyk_press(texture2d<float, access::sample> input [[texture(0)]],
                       texture2d<float, access::write> output [[texture(1)]],
                       constant Params& params [[buffer(0)]],
                       uint2 gid [[thread_position_in_grid]]) {
    if (gid.x >= params.width || gid.y >= params.height) {
        return;
    }

    float2 xy = float2(gid);
    float4 original = sample_pixel(input, gid);
    float4 inks = float4(0.0);
    float ink_alpha = 0.0;
    for (uint plate = 0; plate < 4; ++plate) {
        constant PlatePlan& plan = params.plates[plate];
        HalftonePoint halftone;
        if (params.halftone_enabled != 0) {
            halftone = halftone_point(xy, plan, params);
        }
        float2 sample_pos = params.halftone_enabled != 0 ? halftone.sample_pos : xy + plan.shift;
        float4 sampled = use_nearest_sampling(params) ? sample_nearest(input, sample_pos, params.width, params.height, params.edge_mode)
                                                      : sample_bilinear(input, sample_pos, params.width, params.height, params.edge_mode);
        float plate_ink = sampled.a <= 0.0 ? 0.0 : clamp(rgb_to_cmyk_plate(unpremultiply_rgb(sampled), plate), 0.0, 2.0);
        if (params.halftone_enabled != 0) {
            plate_ink = halftone_coverage_from_cell(halftone.cell, plate_ink, plan, params);
        }
        inks[plate] = clamp(plate_ink * params.ink_amounts[plate], 0.0, 2.0);
        ink_alpha = max(ink_alpha, clamp(inks[plate], 0.0, 1.0) * sampled.a);
    }

    float3 rgb;
    if (params.view_mode == VIEW_CYAN) {
        rgb = composite(float4(inks.x, 0.0, 0.0, 0.0), params);
    } else if (params.view_mode == VIEW_MAGENTA) {
        rgb = composite(float4(0.0, inks.y, 0.0, 0.0), params);
    } else if (params.view_mode == VIEW_YELLOW) {
        rgb = composite(float4(0.0, 0.0, inks.z, 0.0), params);
    } else if (params.view_mode == VIEW_BLACK) {
        rgb = composite(float4(0.0, 0.0, 0.0, inks.w), params);
    } else if (params.view_mode == VIEW_INK_COVERAGE) {
        float v = 1.0 - clamp((inks.x + inks.y + inks.z + inks.w) / 4.0, 0.0, 1.0);
        rgb = float3(v);
    } else if (params.view_mode == VIEW_SPLIT && gid.x < params.width / 2) {
        rgb = unpremultiply_rgb(original);
    } else {
        rgb = composite(inks, params);
    }

    float alpha = params.preserve_alpha != 0 ? original.a : max(original.a, ink_alpha);
    rgb = mix(rgb, unpremultiply_rgb(original), clamp(params.blend_original, 0.0, 1.0));
    output.write(composite_preview_to_output(rgb, alpha, params), gid);
}

kernel void cmyk_press_fast(texture2d<float, access::sample> input [[texture(0)]],
                            texture2d<float, access::write> output [[texture(1)]],
                            constant Params& params [[buffer(0)]],
                            uint2 gid [[thread_position_in_grid]]) {
    if (gid.x >= params.width || gid.y >= params.height) {
        return;
    }

    float4 original = sample_pixel(input, gid);
    float4 cmyk = rgb_to_cmyk_controls(unpremultiply_rgb(original), params);
    float4 inks = original.a <= 0.0 ? float4(0.0) : clamp(cmyk * params.ink_amounts, 0.0, 2.0);
    float3 rgb = composite(inks, params);
    float alpha = original.a;
    output.write(float4(saturate(rgb), clamp(alpha, 0.0, 1.0)), gid);
}
"#;
}
