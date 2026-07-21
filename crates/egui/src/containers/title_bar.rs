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
use epaint::{Galley, Margin, MarginF32, RectShape};

use crate::collapsing_header::CollapsingState;
use crate::{
    Atom, AtomKind, AtomLayout, Atoms, Color32, CornerRadius, Frame, Id, Rect, Response, Sense,
    Shadow, Stroke, TextStyle, Ui, UiBuilder, WidgetInfo, WidgetType, layers::ShapeIdx,
};

/// Default padding used when reserving horizontal title space around side buttons.
pub(super) const TITLE_BAR_PADDING: MarginF32 = MarginF32::same(6.0);

/// Shared title-bar geometry and background painting data.
///
/// `TitleBar` centralizes the rects and radii that multiple containers need to agree on,
/// while leaving container-specific behavior such as closing or collapsing to callers.
pub(super) struct TitleBar {
    /// Actual title-bar background paint bounds.
    pub paint_rect: Rect,

    /// Inner layout bounds for text, button slots, and separator placement.
    pub rect: Rect,

    /// Stroke reused for separator alignment.
    pub stroke: Stroke,

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
            paint_rect,
            rect,
            stroke: frame.stroke,
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
        let button_size = Vec2::splat(ui.spacing().icon_width);
        let button_center = Align2::RIGHT_CENTER
            .align_size_within_rect(
                button_size,
                self.rect.shrink2(Vec2::splat(TITLE_BAR_PADDING.right)),
            )
            .center();
        Rect::from_center_size(button_center, button_size).round_to_pixels(ui.pixels_per_point())
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

pub(super) struct WindowTitleBar<'title, 'state> {
    pub title: Atoms<'title>,
    pub frame: Frame,
    pub collapsing: &'state mut CollapsingState,
    pub collapsible: bool,
    pub active: bool,
    pub open: Option<&'state mut bool>,
    pub auto_sized: bool,
    pub drag_to_move: bool,
    pub area_id: Id,
}

impl WindowTitleBar<'_, '_> {
    pub fn show(self, ui: &mut Ui) -> Response {
        let Self {
            mut title,
            frame,
            collapsing,
            collapsible,
            active,
            open,
            auto_sized,
            drag_to_move,
            area_id,
        } = self;
        let shape_idx = ui.painter().add(crate::Shape::Noop);

        let mut atoms = Atoms::default();
        let button_size = Vec2::splat(ui.spacing().icon_width);
        let heading_font_height =
            ui.fonts_mut(|f| f.row_height(&TextStyle::Heading.resolve(ui.style())));
        let button_allocation_size = Vec2::splat(heading_font_height);
        let button_shrink = (button_allocation_size - button_size) / 2.0;

        let collapse_atom_id = Id::new("__window_collapse_button");
        let close_atom_id = Id::new("__window_close_button");
        let expanded = collapsing.openness(ui.ctx()) > 0.0;

        if collapsible {
            atoms.push_right(Atom::custom(collapse_atom_id, button_allocation_size));
        }

        atoms.push_right(Atom::grow());

        if !auto_sized
            && !title.any_shrink()
            && let Some(first_text) = title
                .iter_mut()
                .find(|a| matches!(a.kind, AtomKind::Text(..)))
        {
            first_text.shrink = true;
        }
        atoms.extend_right(title);
        atoms.push_right(Atom::grow());

        if open.is_some() {
            atoms.push_right(Atom::custom(close_atom_id, button_allocation_size));
        }

        let mut child_ui = ui.new_child(UiBuilder::new());
        let mut layout = AtomLayout::new(atoms)
            .gap(ui.spacing().item_spacing.x)
            .fallback_font(TextStyle::Heading)
            .wrap_mode(crate::TextWrapMode::Truncate)
            .frame(Frame::NONE.inner_margin(Margin::same(6)));

        let frame = frame.inner_margin(0);

        if expanded {
            let min_width = if auto_sized {
                ui.response().rect.width()
            } else {
                child_ui.available_width()
            };
            layout = layout.min_size(Vec2::new(min_width, 0.0));
        }

        let layout_response = layout.show(&mut child_ui);
        let mut title_click_rect = layout_response.response.rect + frame.total_margin();

        if collapsible && let Some(rect) = layout_response.rect(collapse_atom_id) {
            let rect = rect.shrink2(button_shrink);
            title_click_rect = title_click_rect.with_min_x(rect.max.x);
            let icon_response = child_ui.interact(
                rect,
                child_ui.auto_id_with("collapse_button"),
                Sense::click(),
            );
            icon_response.widget_info(|| {
                WidgetInfo::labeled(
                    WidgetType::Button,
                    child_ui.is_enabled(),
                    if collapsing.is_open() { "Hide" } else { "Show" },
                )
            });
            if icon_response.clicked() {
                collapsing.toggle(&child_ui);
            }
            let openness = collapsing.openness(child_ui.ctx());
            crate::collapsing_header::paint_default_icon(&mut child_ui, openness, &icon_response);
        }

        if let Some(open) = open
            && let Some(rect) = layout_response.rect(close_atom_id)
        {
            let rect = rect.shrink2(button_shrink);
            title_click_rect = title_click_rect.with_max_x(rect.min.x);
            if close_button(&mut child_ui, rect, None).clicked() {
                *open = false;
            }
        }

        if collapsible || drag_to_move {
            let sense = if drag_to_move {
                Sense::click_and_drag()
            } else {
                Sense::click()
            };
            let response =
                child_ui.interact(title_click_rect, area_id.with("__title_click"), sense);
            if collapsible && response.double_clicked() {
                collapsing.toggle(&child_ui);
            }
        }

        let header_color = if active {
            ui.visuals().widgets.open.weak_bg_fill
        } else {
            ui.visuals().widgets.noninteractive.weak_bg_fill
        };
        let mut header_frame = frame.shadow(Shadow::NONE).fill(header_color);
        if expanded {
            header_frame.corner_radius.sw = 0;
            header_frame.corner_radius.se = 0;
        }
        ui.painter()
            .set(shape_idx, header_frame.paint(layout_response.rect));

        let mut advance_rect = child_ui.min_rect();
        if auto_sized {
            advance_rect = advance_rect.with_max_x(advance_rect.min.x);
        }
        if expanded {
            advance_rect.max.y += frame.total_margin().bottom + frame.inner_margin.top as f32
                - child_ui.spacing().item_spacing.y;
        }
        ui.advance_cursor_after_rect(advance_rect);

        layout_response.response
    }
}

pub(super) fn close_button(ui: &mut Ui, rect: Rect, foreground_color: Option<Color32>) -> Response {
    let close_id = ui.auto_id_with("window_close_button");
    let response = ui.interact(rect, close_id, Sense::click());
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), "Close window"));

    ui.expand_to_include_rect(response.rect);

    let visuals = ui.style().interact(&response);
    let rect = rect.shrink(2.0).expand(visuals.expansion);
    let stroke = Stroke::new(
        visuals.fg_stroke.width,
        foreground_color.unwrap_or(visuals.fg_stroke.color),
    );
    ui.painter()
        .line_segment([rect.left_top(), rect.right_bottom()], stroke);
    ui.painter()
        .line_segment([rect.right_top(), rect.left_bottom()], stroke);
    response
}
