use eframe::egui::{self, Rounding, Stroke, Visuals};
use eframe::egui::style::{HandleShape, NumericColorSpace, Selection, WidgetVisuals, Widgets};
use eframe::egui::Color32;
use eframe::epaint::Shadow;

//Widget visuals
const NONINTERACTIVE: WidgetVisuals = WidgetVisuals {
    bg_fill: Color32::from_rgb(227, 227, 227),
    weak_bg_fill: Color32::GRAY,
    bg_stroke: Stroke {
        width: 1.0,
        color: Color32::BLACK
    },
    rounding: Rounding::ZERO,
    fg_stroke: Stroke {
        width: 2.0,
        color: Color32::BLACK
    },
    expansion: 5.0
};

const INACTIVE: WidgetVisuals = WidgetVisuals {
    bg_fill: Color32::from_rgb(227, 227, 227),
    weak_bg_fill: Color32::GRAY,
    bg_stroke: Stroke {
        width: 1.0,
        color: Color32::BLACK
    },
    rounding: Rounding::ZERO,
    fg_stroke: Stroke {
        width: 2.0,
        color: Color32::BLACK
    },
    expansion: 5.0
};

const HOVERED: WidgetVisuals = WidgetVisuals {
    bg_fill: Color32::LIGHT_BLUE,
    weak_bg_fill: Color32::LIGHT_BLUE,
    bg_stroke: Stroke {
        width: 1.0,
        color: Color32::DARK_BLUE
    },
    rounding: Rounding::ZERO,
    fg_stroke: Stroke {
        width: 2.0,
        color: Color32::DARK_BLUE
    },
    expansion: 5.0
};

const ACTIVE: WidgetVisuals = WidgetVisuals {
    bg_fill: Color32::LIGHT_GREEN,
    weak_bg_fill: Color32::GRAY,
    bg_stroke: Stroke {
        width: 1.0,
        color: Color32::BLACK
    },
    rounding: Rounding::ZERO,
    fg_stroke: Stroke {
        width: 2.0,
        color: Color32::BLACK
    },
    expansion: 5.0
};

const OPEN: WidgetVisuals = WidgetVisuals {
    bg_fill: Color32::LIGHT_BLUE,
    weak_bg_fill: Color32::LIGHT_BLUE,
    bg_stroke: Stroke {
        width: 1.0,
        color: Color32::BLACK
    },
    rounding: Rounding::ZERO,
    fg_stroke: Stroke {
        width: 2.0,
        color: Color32::GOLD
    },
    expansion: 5.0
};

pub const APP_VISUALS: Visuals = Visuals {
    dark_mode: false,
    override_text_color: None,
    widgets: Widgets {
        noninteractive: NONINTERACTIVE,
        inactive: INACTIVE,
        hovered: HOVERED,
        active: ACTIVE,
        open: OPEN
    },
    selection: Selection {
        bg_fill: Color32::LIGHT_BLUE,
        stroke: Stroke {
            width: 15.0,
            color: Color32::BLACK
        }
    },
    hyperlink_color: Color32::from_rgb(176, 172, 106),
    faint_bg_color: Color32::GOLD,
    extreme_bg_color: Color32::BROWN,
    code_bg_color: Color32::GREEN,
    warn_fg_color: Color32::RED,
    error_fg_color: Color32::BLACK,
    window_rounding: Rounding::ZERO,
    window_shadow: Shadow::NONE,
    window_fill: Color32::from_rgb(227, 227, 227),
    window_stroke: Stroke {
        width: 2.0,
        color: Color32::BLACK
    },
    window_highlight_topmost: true,
    menu_rounding: Rounding::ZERO,
    panel_fill: Color32::GRAY,
    popup_shadow: Shadow::NONE,
    resize_corner_size: 10.0,
    text_cursor: Stroke {
        width: 10.0,
        color: Color32::GOLD
    },
    text_cursor_preview: true,
    clip_rect_margin: 1.0,
    button_frame: true,
    collapsing_header_frame: false,
    indent_has_left_vline: true,
    striped: false,
    slider_trailing_fill: false,
    handle_shape: HandleShape::Circle,
    interact_cursor: Some(egui::CursorIcon::Default),
    image_loading_spinners: true,
    numeric_color_space: NumericColorSpace::GammaByte
};
