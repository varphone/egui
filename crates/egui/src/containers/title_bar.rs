//! Shared title-bar helpers for container types such as Window and Modal.
//!
//! This module stays inside `containers` instead of `widgets` because it depends on
//! container-level geometry: frame rects, stroke widths, separator alignment, and
//! edge-aware button backgrounds.
//!
//! Geometry is intentionally split into two layers:
//! - `paint_rect`: the area used to paint the title-bar background.
//! - `rect`: the inner layout area used for text, button slots, and separator placement.
//!
//! Keeping those separate helps avoid visual artifacts such as frame/title-bar drift,
//! white seams around rounded corners, and background spill into the content area.

use emath::{Align2, GuiRounding as _, Pos2, Vec2};
use epaint::{Galley, MarginF32, RectShape};

use crate::{
    Color32, CornerRadius, Frame, Rect, Response, Sense, Stroke, StrokeKind, Ui, UiBuilder,
    WidgetInfo, WidgetType, layers::ShapeIdx,
};

use super::collapsing_header::CollapsingState;

/// Default padding used when reserving horizontal title space around side buttons.
pub(super) const TITLE_BAR_PADDING: MarginF32 = MarginF32::same(6.0);

/// Shared title-bar geometry and background painting data.
///
/// `TitleBar` centralizes the rects and radii that multiple containers need to agree on,
/// while leaving container-specific behavior such as closing or collapsing to callers.
pub(super) struct TitleBar {
    /// Frame outer rect with outer margin removed.
    pub outer_rect: Rect,

    /// Actual title-bar background paint bounds.
    pub paint_rect: Rect,

    /// Inner layout bounds for text, button slots, and separator placement.
    pub rect: Rect,

    /// Stroke reused for separator alignment.
    pub stroke: Stroke,

    /// Corner radius matching the visible outer title-bar silhouette.
    pub outer_corner_radius: CornerRadius,

    /// Corner radius matching the inner fill area of the title bar.
    pub corner_radius: CornerRadius,
    fill: Color32,
}

impl TitleBar {
    /// Build title-bar geometry from a container frame and its resolved outer rect.
    pub fn new(
        ui: &Ui,
        outer_rect: Rect,
        frame: Frame,
        height_with_padding: f32,
        fill: Color32,
        include_bottom_corners: bool,
    ) -> Self {
        let outer_rect = outer_rect - MarginF32::from(frame.outer_margin);
        let fill_rect = outer_rect.shrink(frame.stroke.width);

        let mut paint_rect = fill_rect;
        let mut separator_y = paint_rect.min.y + height_with_padding + frame.stroke.width / 2.0;
        frame
            .stroke
            .round_center_to_pixel(ui.pixels_per_point(), &mut separator_y);
        if !include_bottom_corners {
            paint_rect.max.y = separator_y;
        }
        let mut rect = fill_rect;
        rect.max.y = rect.min.y + height_with_padding;

        let half_height = (paint_rect.height() / 2.0).round() as u8;

        let mut outer_corner_radius = frame.corner_radius;
        outer_corner_radius.nw = outer_corner_radius.nw.min(half_height);
        outer_corner_radius.ne = outer_corner_radius.ne.min(half_height);
        outer_corner_radius.sw = if include_bottom_corners {
            outer_corner_radius.sw.min(half_height)
        } else {
            0
        };
        outer_corner_radius.se = if include_bottom_corners {
            outer_corner_radius.se.min(half_height)
        } else {
            0
        };

        let mut corner_radius = frame.corner_radius - frame.stroke.width.round() as u8;
        corner_radius.nw = corner_radius.nw.min(half_height);
        corner_radius.ne = corner_radius.ne.min(half_height);
        corner_radius.sw = if include_bottom_corners {
            corner_radius.sw.min(half_height)
        } else {
            0
        };
        corner_radius.se = if include_bottom_corners {
            corner_radius.se.min(half_height)
        } else {
            0
        };

        Self {
            outer_rect,
            paint_rect,
            rect,
            stroke: frame.stroke,
            outer_corner_radius,
            corner_radius,
            fill,
        }
    }

    /// Paint the title-bar background into a preallocated background shape slot.
    pub fn paint(&self, ui: &Ui, background: ShapeIdx) {
        ui.painter().set(
            background,
            RectShape::filled(self.paint_rect, self.corner_radius, self.fill),
        );
    }

    /// Return the separator y-position using the same stroke-centering logic as the frame.
    pub fn separator_y(&self, ui: &Ui) -> f32 {
        let mut separator_y = self.rect.bottom() + self.stroke.width / 2.0;
        self.stroke
            .round_center_to_pixel(ui.pixels_per_point(), &mut separator_y);
        separator_y
    }

    /// Standard right-side slot used by close buttons.
    pub fn close_button_rect(&self, ui: &Ui) -> Rect {
        let button_center = Align2::RIGHT_CENTER
            .align_size_within_rect(Vec2::splat(self.rect.height()), self.rect)
            .center();
        let button_size = Vec2::splat(self.rect.height());
        Rect::from_center_size(button_center, button_size).round_to_pixels(ui.pixels_per_point())
    }

    /// Standard left-side slot used by collapse buttons.
    pub fn left_button_rect(&self, ui: &Ui) -> Rect {
        let button_center = Align2::LEFT_CENTER
            .align_size_within_rect(Vec2::splat(self.rect.height()), self.rect)
            .center();
        let button_size = Vec2::splat(self.rect.height());
        Rect::from_center_size(button_center, button_size).round_to_pixels(ui.pixels_per_point())
    }

    /// Paint bounds for a right-side close-button background.
    pub fn close_button_paint_rect(
        &self,
        ui: &Ui,
        close_button_rect: Rect,
        left_extension: f32,
        bottom_y: f32,
    ) -> Rect {
        Rect::from_min_max(
            Pos2::new(
                close_button_rect.min.x - left_extension,
                self.outer_rect.min.y,
            ),
            Pos2::new(self.outer_rect.max.x, bottom_y),
        )
        .round_to_pixels(ui.pixels_per_point())
    }

    /// Corner radius for a right-edge title-bar button that should visually merge with the frame.
    pub fn right_button_corner_radius(&self, include_bottom_corner: bool) -> CornerRadius {
        let mut corner_radius = self.outer_corner_radius;
        corner_radius.nw = 0;
        corner_radius.sw = 0;
        if !include_bottom_corner {
            corner_radius.se = 0;
        }
        corner_radius
    }

    /// Reserve a symmetric title text area by removing the same width from both sides.
    pub fn title_text_rect(&self, reserved_side_width: f32) -> Rect {
        Rect::from_min_max(
            Pos2::new(self.rect.min.x + reserved_side_width, self.rect.min.y),
            Pos2::new(self.rect.max.x - reserved_side_width, self.rect.max.y),
        )
    }

    /// Compute a centered title text area while accounting for optional left and right buttons.
    pub fn title_text_rect_with_buttons(
        &self,
        left_button_rect: Option<Rect>,
        right_button_rect: Option<Rect>,
        side_padding: f32,
    ) -> Rect {
        let left_reserved_width =
            left_button_rect.map_or(0.0, |rect| rect.max.x - self.rect.min.x + side_padding);
        let right_reserved_width =
            right_button_rect.map_or(0.0, |rect| self.rect.max.x - rect.min.x + side_padding);
        self.title_text_rect(left_reserved_width.max(right_reserved_width))
    }

    /// Position a prepared title galley centered within a previously reserved title rect.
    pub fn centered_galley_pos(galley: &Galley, text_rect: Rect) -> Pos2 {
        Align2::CENTER_CENTER
            .align_size_within_rect(galley.size(), text_rect)
            .left_top()
            - galley.rect.min.to_vec2()
    }
}

/// Windows-style close button used by shared title bars.
pub(super) struct CloseButton {
    id_salt: &'static str,
    accessible_name: &'static str,
    interact_rect: Rect,
    paint_rect: Rect,
    foreground_color: Color32,
    corner_radius: CornerRadius,
}

impl CloseButton {
    /// Create a close button with separate interaction and paint bounds.
    pub fn new(
        id_salt: &'static str,
        accessible_name: &'static str,
        interact_rect: Rect,
        paint_rect: Rect,
        foreground_color: Color32,
        corner_radius: CornerRadius,
    ) -> Self {
        Self {
            id_salt,
            accessible_name,
            interact_rect,
            paint_rect,
            foreground_color,
            corner_radius,
        }
    }

    /// Show the close button and paint its icon and hover/pressed background.
    pub fn show(self, ui: &mut Ui) -> Response {
        let close_id = ui.auto_id_with(self.id_salt);
        let response = ui.interact(self.interact_rect, close_id, Sense::click());
        response.widget_info(|| {
            WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), self.accessible_name)
        });

        ui.expand_to_include_rect(response.rect);

        let visuals = ui.style().interact(&response);
        let (background_fill, background_stroke, icon_color) =
            if response.is_pointer_button_down_on() {
                (
                    Color32::from_rgb(180, 24, 24),
                    Stroke::new(1.0, Color32::from_rgb(140, 12, 12)),
                    Color32::WHITE,
                )
            } else if response.hovered() {
                (
                    Color32::from_rgb(212, 43, 43),
                    Stroke::new(1.0, Color32::from_rgb(180, 24, 24)),
                    Color32::WHITE,
                )
            } else {
                (Color32::TRANSPARENT, Stroke::NONE, self.foreground_color)
            };

        if background_fill != Color32::TRANSPARENT || background_stroke != Stroke::NONE {
            ui.painter().rect(
                self.paint_rect.round_to_pixels(ui.pixels_per_point()),
                self.corner_radius,
                background_fill,
                background_stroke,
                StrokeKind::Inside,
            );
        }

        let icon_rect = Rect::from_center_size(
            self.interact_rect.center(),
            Vec2::splat(ui.spacing().icon_width),
        )
        .shrink(1.0)
        .round_to_pixels(ui.pixels_per_point());
        let stroke = Stroke::new(visuals.fg_stroke.width, icon_color);
        ui.painter()
            .line_segment([icon_rect.left_top(), icon_rect.right_bottom()], stroke);
        ui.painter()
            .line_segment([icon_rect.right_top(), icon_rect.left_bottom()], stroke);
        response
    }
}

/// Wrapper for the standard egui collapse button placed in a title-bar slot.
pub(super) struct CollapseButton {
    slot_rect: Rect,
}

impl CollapseButton {
    /// Create a collapse button centered inside a precomputed title-bar slot.
    pub fn new(slot_rect: Rect) -> Self {
        Self { slot_rect }
    }

    /// Paint and interact with the standard egui collapse button inside the slot.
    pub fn show(self, ui: &mut Ui, collapsing: &mut CollapsingState) -> Response {
        let button_size = Vec2::splat(ui.spacing().icon_width);
        let button_rect = Rect::from_center_size(self.slot_rect.center(), button_size).round_ui();

        ui.scope_builder(UiBuilder::new().max_rect(button_rect), |ui| {
            collapsing.show_default_button_with_size(ui, button_size)
        })
        .response
    }
}
