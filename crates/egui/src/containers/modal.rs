use std::sync::Arc;

use emath::{Align2, GuiRounding as _, NumExt as _, Pos2, Vec2};
use epaint::{MarginF32, RectShape};

use crate::{
    Area, Color32, Context, Frame, Galley, Id, InnerResponse, Order, Rect, Response, Sense, Shape,
    TextStyle, Ui, UiBuilder, UiKind, WidgetInfo, WidgetText, WidgetType,
};

const TITLE_BAR_PADDING: MarginF32 = MarginF32::same(6.0);

/// A modal dialog.
///
/// Similar to a [`crate::Window`] but centered and with a backdrop that
/// blocks input to the rest of the UI.
///
/// You can show multiple modals on top of each other. The topmost modal will always be
/// the most recently shown one.
/// If multiple modals are newly shown in the same frame, the order of the modals is undefined
/// (either first or second could be top).
pub struct Modal {
    pub area: Area,
    pub backdrop_color: Color32,
    pub frame: Option<Frame>,
    pub title: Option<WidgetText>,
    pub title_bar_fill: Option<Color32>,
}

impl Modal {
    /// Create a new Modal.
    ///
    /// The id is passed to the area.
    pub fn new(id: Id) -> Self {
        Self {
            area: Self::default_area(id),
            backdrop_color: Color32::from_black_alpha(100),
            frame: None,
            title: None,
            title_bar_fill: None,
        }
    }

    /// Returns an area customized for a modal.
    ///
    /// Makes these changes to the default area:
    /// - sense: hover
    /// - anchor: center
    /// - order: foreground
    pub fn default_area(id: Id) -> Area {
        Area::new(id)
            .kind(UiKind::Modal)
            .sense(Sense::hover())
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .order(Order::Foreground)
            .interactable(true)
    }

    /// Set the frame of the modal.
    ///
    /// Default is [`Frame::popup`].
    #[inline]
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = Some(frame);
        self
    }

    /// Set the title of the modal.
    ///
    /// When set, the modal shows a title bar above the contents.
    #[inline]
    pub fn title(mut self, title: impl Into<WidgetText>) -> Self {
        self.title = Some(title.into().fallback_text_style(TextStyle::Heading));
        self
    }

    /// Override the background color of the title bar.
    ///
    /// Only has an effect when the modal has a title.
    #[inline]
    pub fn title_bar_fill(mut self, color: Color32) -> Self {
        self.title_bar_fill = Some(color);
        self
    }

    /// Set the backdrop color of the modal.
    ///
    /// Default is `Color32::from_black_alpha(100)`.
    #[inline]
    pub fn backdrop_color(mut self, color: Color32) -> Self {
        self.backdrop_color = color;
        self
    }

    /// Set the area of the modal.
    ///
    /// Default is [`Modal::default_area`].
    #[inline]
    pub fn area(mut self, area: Area) -> Self {
        self.area = area;
        self
    }

    /// Show the modal.
    pub fn show<T>(self, ctx: &Context, content: impl FnOnce(&mut Ui) -> T) -> ModalResponse<T> {
        let Self {
            area,
            backdrop_color,
            frame,
            title,
            title_bar_fill,
        } = self;

        let is_top_modal = ctx.memory_mut(|mem| {
            mem.set_modal_layer(area.layer());
            mem.top_modal_layer() == Some(area.layer())
        });
        let any_popup_open = crate::Popup::is_any_open(ctx);
        let InnerResponse {
            inner: (inner, backdrop_response),
            response,
        } = area.show(ctx, |ui| {
            let bg_rect = ui.ctx().content_rect();
            let bg_sense = Sense::CLICK | Sense::DRAG;
            let mut backdrop = ui.new_child(UiBuilder::new().sense(bg_sense).max_rect(bg_rect));
            backdrop.set_min_size(bg_rect.size());
            ui.painter().rect_filled(bg_rect, 0.0, backdrop_color);
            let backdrop_response = backdrop.response();

            let frame = frame.unwrap_or_else(|| Frame::popup(ui.style()));

            // We need the extra scope with the sense since frame can't have a sense and since we
            // need to prevent the clicks from passing through to the backdrop.
            let inner = ui
                .scope_builder(UiBuilder::new().sense(Sense::CLICK | Sense::DRAG), |ui| {
                    let pixels_per_point = ui.pixels_per_point();
                    let mut prepared_frame = frame.begin(ui);
                    let where_to_put_header_background = ui.painter().add(Shape::Noop);

                    let title_bar = title.map(|title| {
                        ModalTitleBar::new(
                            &prepared_frame.content_ui,
                            title,
                            prepared_frame.frame,
                            title_bar_fill,
                        )
                    });

                    if let Some(title_bar) = &title_bar {
                        let content_offset = (title_bar.height_with_padding
                            + prepared_frame.frame.stroke.width)
                            .round_to_pixels(pixels_per_point);
                        prepared_frame.content_ui.add_space(content_offset);
                    }

                    let inner = content(&mut prepared_frame.content_ui);
                    let outer_rect = prepared_frame.end(ui).rect;

                    if let Some(title_bar) = title_bar {
                        title_bar.ui(ui, outer_rect, where_to_put_header_background);
                    }

                    inner
                })
                .inner;

            (inner, backdrop_response)
        });

        ModalResponse {
            response,
            backdrop_response,
            inner,
            is_top_modal,
            any_popup_open,
        }
    }
}

struct ModalTitleBar {
    frame: Frame,
    title_galley: Arc<Galley>,
    height_with_padding: f32,
    title_bar_fill: Color32,
    foreground_color: Color32,
}

impl ModalTitleBar {
    fn new(ui: &Ui, title: WidgetText, frame: Frame, title_bar_fill: Option<Color32>) -> Self {
        let title_galley = title.into_galley(
            ui,
            Some(crate::TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Heading,
        );
        let title_height = title_galley.size().y.at_least(ui.spacing().interact_size.y);
        let height_with_padding =
            (title_height + TITLE_BAR_PADDING.sum().y).round_to_pixels(ui.pixels_per_point());
        let title_bar_fill =
            title_bar_fill.unwrap_or_else(|| ui.visuals().widgets.open.weak_bg_fill);

        Self {
            frame,
            title_galley,
            height_with_padding,
            title_bar_fill,
            foreground_color: contrast_color(title_bar_fill),
        }
    }

    fn ui(self, ui: &mut Ui, outer_rect: Rect, background: crate::layers::ShapeIdx) {
        let mut title_bar_rect = outer_rect
            .shrink(self.frame.stroke.width)
            .round_to_pixels(ui.pixels_per_point());
        title_bar_rect.max.y = (title_bar_rect.min.y + self.height_with_padding)
            .round_to_pixels(ui.pixels_per_point());

        let mut corner_radius = self.frame.corner_radius - self.frame.stroke.width.round() as u8;
        let half_height = (self.height_with_padding / 2.0).round() as u8;
        corner_radius.nw = corner_radius.nw.min(half_height);
        corner_radius.ne = corner_radius.ne.min(half_height);
        corner_radius.sw = 0;
        corner_radius.se = 0;

        ui.painter().set(
            background,
            RectShape::filled(title_bar_rect, corner_radius, self.title_bar_fill)
                .with_round_to_pixels(true),
        );

        let close_button_rect = Self::close_button_rect(ui, title_bar_rect);
        let close_button_response = close_button(ui, close_button_rect, self.foreground_color);
        if close_button_response.clicked() {
            ui.close();
        }

        let side_reserved_width =
            title_bar_rect.max.x - close_button_rect.min.x + TITLE_BAR_PADDING.left;
        let text_rect = Rect::from_min_max(
            Pos2::new(
                title_bar_rect.min.x + side_reserved_width,
                title_bar_rect.min.y,
            ),
            Pos2::new(
                title_bar_rect.max.x - side_reserved_width,
                title_bar_rect.max.y,
            ),
        );
        let text_pos = Align2::CENTER_CENTER
            .align_size_within_rect(self.title_galley.size(), text_rect)
            .left_top()
            - self.title_galley.rect.min.to_vec2();
        ui.painter()
            .galley(text_pos, self.title_galley, self.foreground_color);

        let mut separator_y = title_bar_rect.bottom() + self.frame.stroke.width / 2.0;
        self.frame
            .stroke
            .round_center_to_pixel(ui.pixels_per_point(), &mut separator_y);
        ui.painter()
            .hline(title_bar_rect.x_range(), separator_y, self.frame.stroke);
    }

    fn close_button_rect(ui: &Ui, title_bar_rect: Rect) -> Rect {
        let button_center = Align2::RIGHT_CENTER
            .align_size_within_rect(Vec2::splat(title_bar_rect.height()), title_bar_rect)
            .center();
        let button_size = Vec2::splat(ui.spacing().icon_width);
        Rect::from_center_size(button_center, button_size).round_to_pixels(ui.pixels_per_point())
    }
}

fn close_button(ui: &mut Ui, rect: Rect, foreground_color: Color32) -> Response {
    let close_id = ui.auto_id_with("modal_close_button");
    let response = ui.interact(rect, close_id, Sense::click());
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), "Close modal"));

    ui.expand_to_include_rect(response.rect);

    let visuals = ui.style().interact(&response);
    let rect = rect.shrink(2.0).expand(visuals.expansion);
    let stroke = crate::Stroke::new(visuals.fg_stroke.width, foreground_color);
    ui.painter()
        .line_segment([rect.left_top(), rect.right_bottom()], stroke);
    ui.painter()
        .line_segment([rect.right_top(), rect.left_bottom()], stroke);
    response
}

fn contrast_color(color: impl Into<crate::Rgba>) -> Color32 {
    if color.into().intensity() < 0.5 {
        Color32::WHITE
    } else {
        Color32::BLACK
    }
}

/// The response of a modal dialog.
pub struct ModalResponse<T> {
    /// The response of the modal contents
    pub response: Response,

    /// The response of the modal backdrop.
    ///
    /// A click on this means the user clicked outside the modal,
    /// in which case you might want to close the modal.
    pub backdrop_response: Response,

    /// The inner response from the content closure
    pub inner: T,

    /// Is this the topmost modal?
    pub is_top_modal: bool,

    /// Is there any popup open?
    /// We need to check this before the modal contents are shown, so we can know if any popup
    /// was open when checking if the escape key was clicked.
    pub any_popup_open: bool,
}

impl<T> ModalResponse<T> {
    /// Should the modal be closed?
    /// Returns true if:
    ///  - the backdrop was clicked
    ///  - this is the topmost modal, no popup is open and the escape key was pressed
    pub fn should_close(&self) -> bool {
        let ctx = &self.response.ctx;

        // this is a closure so that `Esc` is consumed only if the modal is topmost
        let escape_clicked =
            || ctx.input_mut(|i| i.consume_key(crate::Modifiers::NONE, crate::Key::Escape));

        let ui_close_called = self.response.should_close();

        self.backdrop_response.clicked()
            || ui_close_called
            || (self.is_top_modal && !self.any_popup_open && escape_clicked())
    }
}
