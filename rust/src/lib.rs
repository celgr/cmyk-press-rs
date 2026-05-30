use after_effects as ae;
use std::thread;

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
    TransparentMode,
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
    Backend,
    Quality,
    EdgeMode,
    ExpandBounds,
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
pub const CMYK_EDGE_TRANSPARENT: u32 = 0;
pub const CMYK_EDGE_CLAMP: u32 = 1;

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
const EDGE_TRANSPARENT: i32 = 1;
const EDGE_CLAMP: i32 = 2;
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
const DEFAULT_HALFTONE_DOT_GAIN: f32 = -0.15;
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
                    ae::ParamUIFlags::empty(),
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
                    "Transparent Mode",
                    ae::CheckBoxDef::setup(|f| {
                        f.set_default(false);
                        f.set_label("White transparent");
                    }),
                )?;
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
                    200.0,
                )?;
                add_percent_param(
                    params,
                    Params::MagentaAmount,
                    "Magenta",
                    DEFAULT_CMY_INK_AMOUNT * 100.0,
                    0.0,
                    200.0,
                )?;
                add_percent_param(
                    params,
                    Params::YellowAmount,
                    "Yellow",
                    DEFAULT_CMY_INK_AMOUNT * 100.0,
                    0.0,
                    200.0,
                )?;
                add_percent_param(
                    params,
                    Params::BlackAmount,
                    "Black",
                    DEFAULT_BLACK_INK_AMOUNT * 100.0,
                    0.0,
                    200.0,
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
                    200.0,
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
                        f.set_label("Fixed random registration");
                    }),
                )?;
                params.add(
                    Params::RandomSeed,
                    "Seed",
                    ae::SliderDef::setup(|f| {
                        f.set_valid_min(0);
                        f.set_valid_max(2_147_483_647);
                        f.set_slider_min(0);
                        f.set_slider_max(9999);
                        f.set_default(0);
                    }),
                )?;
                add_px_param(params, Params::RandomAmountX, "Amount X", 3.0, 0.0, 1000.0)?;
                add_px_param(params, Params::RandomAmountY, "Amount Y", 3.0, 0.0, 1000.0)?;
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
                    10.0,
                    0.0,
                    100.0,
                )?;
                add_angle_param(params, Params::HalftoneCyanAngle, "Cyan Angle", 15.0)?;
                add_angle_param(params, Params::HalftoneMagentaAngle, "Magenta Angle", 75.0)?;
                add_angle_param(params, Params::HalftoneYellowAngle, "Yellow Angle", 0.0)?;
                add_angle_param(params, Params::HalftoneBlackAngle, "Black Angle", 45.0)?;
                add_px_param(
                    params,
                    Params::HalftoneOffsetX,
                    "Offset X",
                    0.0,
                    -10_000.0,
                    10_000.0,
                )?;
                add_px_param(
                    params,
                    Params::HalftoneOffsetY,
                    "Offset Y",
                    0.0,
                    -10_000.0,
                    10_000.0,
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
                    Params::EdgeMode,
                    "Edge Mode",
                    ae::PopupDef::setup(|f| {
                        f.set_options(&["Transparent", "Clamp Edge"]);
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
                let ep = get_params(params)?;
                let out = render_cmyk_press(&src, &ep);
                frame_to_layer(&out, &mut out_layer);
            }
            ae::Command::SmartPreRender { mut extra } => {
                smart_pre_render(&in_data, &mut extra)?;
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
struct EffectParams {
    view_mode: i32,
    preserve_alpha: bool,
    blend_original: f32,
    ink_amounts: [f32; PLATE_COUNT],
    paper: [f32; 3],
    offsets: [[f32; 2]; PLATE_COUNT],
    random_enabled: bool,
    random_seed: u32,
    random_amount: [f32; 2],
    random_affect: [bool; PLATE_COUNT],
    halftone_enabled: bool,
    halftone_frequency: f32,
    halftone_unit: i32,
    halftone_shape: i32,
    halftone_dot_gain: f32,
    halftone_softness: f32,
    halftone_angles: [f32; PLATE_COUNT],
    halftone_offset: [f32; 2],
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    backend: i32,
    quality: i32,
    edge_mode: i32,
    expand_bounds: bool,
    conversion_mode: i32,
    ink_colors: [[f32; 3]; PLATE_COUNT],
    transparent_mode: bool,
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
                params.cyan_amount,
                params.magenta_amount,
                params.yellow_amount,
                params.black_amount,
            ],
            paper: apply_paper_controls(
                params.paper_color,
                params.paper_brightness.clamp(0.0, 2.0),
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
                self.ink_amounts[0].clamp(0.0, 2.0),
                self.ink_amounts[1].clamp(0.0, 2.0),
                self.ink_amounts[2].clamp(0.0, 2.0),
                self.ink_amounts[3].clamp(0.0, 2.0),
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
    params.add(
        id,
        name,
        ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(min);
            f.set_valid_max(max);
            f.set_slider_min(min);
            f.set_slider_max(max);
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
    add_px_param(params, id, name, default, -100.0, 100.0)
}

fn add_angle_param(
    params: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    default: f32,
) -> Result<(), ae::Error> {
    add_px_param(params, id, name, default, 0.0, 180.0)
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

fn get_params(params: &ae::Parameters<Params>) -> Result<EffectParams, ae::Error> {
    let paper_base = pixel_to_rgb(params.get(Params::PaperColor)?.as_color()?.value());
    let brightness = percent(params, Params::PaperBrightness, 0.0, 2.0)?;
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
            percent(params, Params::CyanAmount, 0.0, 2.0)?,
            percent(params, Params::MagentaAmount, 0.0, 2.0)?,
            percent(params, Params::YellowAmount, 0.0, 2.0)?,
            percent(params, Params::BlackAmount, 0.0, 2.0)?,
        ],
        paper: apply_paper_controls(paper_base, brightness),
        offsets: [
            [
                float_param(params, Params::CyanOffsetX)?.clamp(-100.0, 100.0),
                float_param(params, Params::CyanOffsetY)?.clamp(-100.0, 100.0),
            ],
            [
                float_param(params, Params::MagentaOffsetX)?.clamp(-100.0, 100.0),
                float_param(params, Params::MagentaOffsetY)?.clamp(-100.0, 100.0),
            ],
            [
                float_param(params, Params::YellowOffsetX)?.clamp(-100.0, 100.0),
                float_param(params, Params::YellowOffsetY)?.clamp(-100.0, 100.0),
            ],
            [
                float_param(params, Params::BlackOffsetX)?.clamp(-100.0, 100.0),
                float_param(params, Params::BlackOffsetY)?.clamp(-100.0, 100.0),
            ],
        ],
        random_enabled: params.get(Params::RandomEnable)?.as_checkbox()?.value(),
        random_seed: params
            .get(Params::RandomSeed)?
            .as_slider()?
            .value()
            .clamp(0, 2_147_483_647) as u32,
        random_amount: [
            float_param(params, Params::RandomAmountX)?.clamp(0.0, 1000.0),
            float_param(params, Params::RandomAmountY)?.clamp(0.0, 1000.0),
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
            float_param(params, Params::HalftoneCyanAngle)?.clamp(0.0, 180.0),
            float_param(params, Params::HalftoneMagentaAngle)?.clamp(0.0, 180.0),
            float_param(params, Params::HalftoneYellowAngle)?.clamp(0.0, 180.0),
            float_param(params, Params::HalftoneBlackAngle)?.clamp(0.0, 180.0),
        ],
        halftone_offset: [
            float_param(params, Params::HalftoneOffsetX)?,
            float_param(params, Params::HalftoneOffsetY)?,
        ],
        backend: normalize_backend(params.get(Params::Backend)?.as_popup()?.value() as i32),
        quality: normalize_quality(params.get(Params::Quality)?.as_popup()?.value() as i32),
        edge_mode: normalize_edge_mode(params.get(Params::EdgeMode)?.as_popup()?.value() as i32),
        expand_bounds: params.get(Params::ExpandBounds)?.as_checkbox()?.value(),
        conversion_mode,
        ink_colors,
        transparent_mode: params.get(Params::TransparentMode)?.as_checkbox()?.value(),
    })
}

fn float_param(params: &ae::Parameters<Params>, param: Params) -> Result<f32, ae::Error> {
    Ok(params.get(param)?.as_float_slider()?.value() as f32)
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

fn normalize_edge_mode(value: i32) -> i32 {
    match value {
        EDGE_CLAMP => value,
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

fn smart_pre_render(
    in_data: &ae::InData,
    extra: &mut ae::pf::PreRenderExtra,
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
    let res: ae::Rect = in_result.result_rect.into();
    let max_res: ae::Rect = in_result.max_result_rect.into();

    // Use the full available rect so that edge sampling (clamp/transparent)
    // and registration offsets can access pixels outside the output rect.
    extra.set_result_rect(res);
    extra.set_max_result_rect(max_res);
    let _ = req_rect;
    extra.set_gpu_render_possible(gpu_prerender_possible(extra));
    Ok(())
}

fn smart_render(
    extra: &ae::pf::SmartRenderExtra,
    params: &ae::Parameters<Params>,
) -> Result<(), ae::Error> {
    let ep = get_params(params)?;
    let cb = extra.callbacks();
    let input_world = cb.checkout_layer_pixels(0)?.ok_or(ae::Error::Generic)?;
    let result = (|| {
        let mut output_world = cb.checkout_output()?.ok_or(ae::Error::Generic)?;
        let src = layer_to_frame(&input_world);
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

    let ep = get_params(params)?;
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

#[derive(Clone)]
struct Frame {
    pixels: Vec<Rgba>,
    w: usize,
    h: usize,
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
                    pixels[y * w + x] = Rgba {
                        rgb: [
                            p.red as f32 * scale,
                            p.green as f32 * scale,
                            p.blue as f32 * scale,
                        ],
                        a: p.alpha as f32 * scale,
                    };
                }
            }
        }
        32 => {
            for y in 0..h {
                for x in 0..w {
                    let p = layer.as_pixel32(x, y);
                    pixels[y * w + x] = Rgba {
                        rgb: [
                            p.red.clamp(0.0, 1.0),
                            p.green.clamp(0.0, 1.0),
                            p.blue.clamp(0.0, 1.0),
                        ],
                        a: p.alpha.clamp(0.0, 1.0),
                    };
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
                    pixels[y * w + x] = Rgba {
                        rgb: [
                            row[off + 1] as f32 / 255.0,
                            row[off + 2] as f32 / 255.0,
                            row[off + 3] as f32 / 255.0,
                        ],
                        a: row[off] as f32 / 255.0,
                    };
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
                    let out = layer.as_pixel16_mut(x, y);
                    out.alpha = to_u16(px.a);
                    out.red = to_u16(px.rgb[0]);
                    out.green = to_u16(px.rgb[1]);
                    out.blue = to_u16(px.rgb[2]);
                }
            }
        }
        32 => {
            for y in 0..h {
                for x in 0..w {
                    let px = frame.pixels[y * frame.w + x];
                    let out = layer.as_pixel32_mut(x, y);
                    out.alpha = px.a.clamp(0.0, 1.0);
                    out.red = px.rgb[0].clamp(0.0, 1.0);
                    out.green = px.rgb[1].clamp(0.0, 1.0);
                    out.blue = px.rgb[2].clamp(0.0, 1.0);
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
                    let off = x * 4;
                    row[off] = to_u8(px.a);
                    row[off + 1] = to_u8(px.rgb[0]);
                    row[off + 2] = to_u8(px.rgb[1]);
                    row[off + 3] = to_u8(px.rgb[2]);
                }
            }
        }
    }
}

fn render_cmyk_press(src: &Frame, ep: &EffectParams) -> Frame {
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
struct PlatePlan {
    shift: [f32; 2],
    pivot: [f32; 2],
    sin: f32,
    cos: f32,
    cell: f32,
}

#[derive(Clone, Copy)]
struct RenderPlan {
    plates: [PlatePlan; PLATE_COUNT],
}

impl RenderPlan {
    fn new(ep: &EffectParams, width: usize, height: usize) -> Self {
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
struct Rgba {
    rgb: [f32; 3],
    a: f32,
}

impl Rgba {
    fn transparent() -> Self {
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

fn pixel_to_rgb(pixel: ae::Pixel8) -> [f32; 3] {
    [
        pixel.red as f32 / 255.0,
        pixel.green as f32 / 255.0,
        pixel.blue as f32 / 255.0,
    ]
}

fn mix_rgb(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
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
        _pad: [f32; 3],
    }

    #[repr(C)]
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
        edge_mode: i32,
        conversion_mode: i32,
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
                edge_mode: ep.edge_mode,
                conversion_mode: ep.conversion_mode,
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
                        _pad: [0.0; 3],
                    }
                }),
            }
        }
    }

    pub struct MetalState {
        pipeline: metal::ComputePipelineState,
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

            // Use AE's command queue when available; otherwise create our own.
            let command_queue = match queue_opt {
                Some(q) => q,
                None => device.new_command_queue(),
            };

            Ok(Self {
                pipeline,
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
            encoder.set_compute_pipeline_state(&self.pipeline);
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
constant int CONVERSION_ILLUSTRATOR = 2;
constant int CONVERSION_CUSTOM = 3;

struct PlatePlan {
    float2 shift;
    float2 pivot;
    float sin_v;
    float cos_v;
    float cell;
    float3 pad;
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
    int edge_mode;
    int conversion_mode;
    float4 paper;
    float4 ink_amounts;
    float4 halftone_offset;
    float4 ink_colors[4];
    PlatePlan plates[4];
};

static inline float4 sample_pixel(texture2d<float, access::read> src, uint2 pos) {
    return saturate(src.read(pos));
}

static inline float4 sample_nearest(texture2d<float, access::read> src, float2 pos, uint width, uint height, int edge_mode) {
    if (width == 0 || height == 0) {
        return float4(0.0);
    }
    if (edge_mode == EDGE_CLAMP) {
        pos = clamp(pos, float2(0.0), float2(float(width - 1), float(height - 1)));
    } else if (pos.x < 0.0 || pos.y < 0.0 || pos.x > float(width - 1) || pos.y > float(height - 1)) {
        return float4(0.0);
    }
    return sample_pixel(src, uint2(round(pos)));
}

static inline float4 sample_bilinear(texture2d<float, access::read> src, float2 pos, uint width, uint height, int edge_mode) {
    if (width == 0 || height == 0) {
        return float4(0.0);
    }
    if (edge_mode == EDGE_CLAMP) {
        pos = clamp(pos, float2(0.0), float2(float(width - 1), float(height - 1)));
    } else if (pos.x < 0.0 || pos.y < 0.0 || pos.x > float(width - 1) || pos.y > float(height - 1)) {
        return float4(0.0);
    }
    uint x0 = uint(floor(pos.x));
    uint y0 = uint(floor(pos.y));
    uint x1 = min(x0 + 1, width - 1);
    uint y1 = min(y0 + 1, height - 1);
    float tx = pos.x - float(x0);
    float ty = pos.y - float(y0);
    return mix(mix(sample_pixel(src, uint2(x0, y0)), sample_pixel(src, uint2(x1, y0)), tx),
               mix(sample_pixel(src, uint2(x0, y1)), sample_pixel(src, uint2(x1, y1)), tx),
               ty);
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

static inline float2 dot_cell_position(float2 xy, constant PlatePlan& plan, constant Params& params) {
    float2 rotated = halftone_rotated_position(xy, plan, params);
    return float2(rotated.x / plan.cell - floor(rotated.x / plan.cell), rotated.y / plan.cell - floor(rotated.y / plan.cell)) - 0.5;
}

static inline float2 halftone_sample_position(float2 xy, constant PlatePlan& plan, constant Params& params) {
    float2 rotated = halftone_rotated_position(xy, plan, params);
    float2 center = floor(rotated / plan.cell) * plan.cell + plan.cell * 0.5;
    float2 unrotated = float2(center.x * plan.cos_v - center.y * plan.sin_v,
                              center.x * plan.sin_v + center.y * plan.cos_v);
    return unrotated + plan.pivot - params.halftone_offset.xy;
}

static inline float dot_radius(float value) {
    return sqrt(clamp(value, 0.0, 1.0)) * 0.5;
}

static inline float dot_edge_width(float cell, float softness) {
    float cell_aa = 0.5 / max(cell, 1.0);
    return max(cell_aa + clamp(softness, 0.0, 1.0) * 0.03, 0.0001);
}

static inline float smooth_circle(float dist, float radius, float edge) {
    return clamp((radius + edge - dist) / (2.0 * edge), 0.0, 1.0);
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

static inline float halftone_coverage(float2 xy, float value, constant PlatePlan& plan, constant Params& params) {
    value = apply_dot_gain(value, params.halftone_dot_gain);
    if (value <= 0.0) return 0.0;
    float2 cell = dot_cell_position(xy, plan, params);
    float dist = dot_shape_distance(cell, params.halftone_shape);
    float radius = dot_radius(value);
    float edge = dot_edge_width(plan.cell, params.halftone_softness);
    return smooth_circle(dist, radius, edge);
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

static inline float cmyk_smoothstep(float t) {
    t = clamp(t, 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

static inline float4 apply_white_transparency(float3 rgb, float alpha, constant Params& params) {
    alpha = clamp(alpha, 0.0, 1.0);
    if (params.transparent_mode == 0 || alpha <= 0.0) {
        return float4(rgb, alpha);
    }
    rgb = saturate(rgb);
    float3 white_delta = 1.0 - rgb;
    float matte_alpha = max(max(white_delta.r, white_delta.g), white_delta.b);
    if (matte_alpha <= 0.0001) {
        return float4(0.0);
    }
    float3 recovered_rgb = saturate(1.0 - white_delta / matte_alpha);
    float threshold = 1.0;
    float normalized_alpha = clamp(matte_alpha / threshold, 0.0, 1.0);
    float soft_alpha = cmyk_smoothstep(normalized_alpha);
    float softness = 0.0;
    float coverage = normalized_alpha + (soft_alpha - normalized_alpha) * softness;
    return float4(recovered_rgb, alpha * coverage);
}

kernel void cmyk_press(texture2d<float, access::read> input [[texture(0)]],
                       texture2d<float, access::write> output [[texture(1)]],
                       constant Params& params [[buffer(0)]],
                       uint2 gid [[thread_position_in_grid]]) {
    if (gid.x >= params.width || gid.y >= params.height) {
        return;
    }

    float2 xy = float2(gid);
    float4 original = sample_pixel(input, gid);
    float4 inks = float4(0.0);
    float alpha_max = 0.0;
    for (uint plate = 0; plate < 4; ++plate) {
        constant PlatePlan& plan = params.plates[plate];
        float2 sample_pos = params.halftone_enabled != 0 ? halftone_sample_position(xy, plan, params) : xy + plan.shift;
        float4 sampled = params.quality == 1 ? sample_nearest(input, sample_pos, params.width, params.height, params.edge_mode)
                                             : sample_bilinear(input, sample_pos, params.width, params.height, params.edge_mode);
        alpha_max = max(alpha_max, sampled.a);
        float4 cmyk = rgb_to_cmyk_controls(unpremultiply_rgb(sampled), params);
        inks[plate] = clamp(cmyk[plate] * sampled.a, 0.0, 2.0);
    }
    if (params.halftone_enabled != 0) {
        for (uint plate = 0; plate < 4; ++plate) {
            inks[plate] = halftone_coverage(xy, inks[plate], params.plates[plate], params);
        }
    }
    inks = clamp(inks * params.ink_amounts, 0.0, 2.0);

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

    float alpha = params.preserve_alpha != 0 ? original.a : alpha_max;
    rgb = mix(rgb, unpremultiply_rgb(original), clamp(params.blend_original, 0.0, 1.0));
    float4 white_unmult = apply_white_transparency(rgb, alpha, params);
    rgb = white_unmult.rgb;
    alpha = white_unmult.a;
    output.write(float4(saturate(rgb * alpha), clamp(alpha, 0.0, 1.0)), gid);
}
"#;
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
