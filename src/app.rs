use eframe::emath::{pos2, Rect, Rot2};
use eframe::epaint::{Color32, Rounding, Shape, Stroke};
use egui::emath::TSTransform;
use egui::epaint::{CubicBezierShape, PathShape, TextShape};
use egui::{emath, epaint, Frame, Grid, Id, ImageOptions, Pos2, Sense, Ui, Vec2, Widget};
use egui_plot::{HLine, LineStyle, PlotBounds, PlotGeometry, PlotPoint, PlotTransform};
use std::ops::RangeInclusive;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MainApp {
    label: String,

    control_points: [Pos2; 4],

    #[serde[skip]]
    aux_stroke: Stroke,
    #[serde[skip]]
    fill: Color32,
    #[serde[skip]]
    stroke: Stroke,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            control_points: [
                pos2(50.0, 50.0),
                pos2(60.0, 250.0),
                pos2(200.0, 200.0),
                pos2(250.0, 50.0),
            ],
            aux_stroke: Stroke::new(1.0, Color32::RED.linear_multiply(0.25)),
            fill: Color32::from_rgb(50, 100, 150).linear_multiply(0.25),
            stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
        }
    }
}

struct PlotShape {
    pub(super) shape: Shape,
    pub(super) stroke: egui::Stroke,
    pub(super) name: String,
    pub(super) highlight: bool,
    pub(super) allow_hover: bool,
    pub(super) style: LineStyle,
    id: Option<Id>,
}

impl PlotShape {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            stroke: Stroke::new(10.0, Color32::RED),
            name: String::default(),
            highlight: false,
            allow_hover: true,
            style: LineStyle::Solid,
            id: None,
        }
    }

    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    #[inline]
    pub fn allow_hover(mut self, hovering: bool) -> Self {
        self.allow_hover = hovering;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    #[inline]
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.stroke.width = width.into();
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.stroke.color = color.into();
        self
    }

    #[inline]
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }
}

impl egui_plot::PlotItem for PlotShape {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let mut shape = self.shape.clone();
        let t = transform.position_from_point(&PlotPoint::new(0.0, 0.0));
        let t2 = transform.position_from_point(&PlotPoint::new(100.0, 1.0));
        let screen_rect = egui::Rect::from([[t.x, t.y].into(), [t2.x, t2.y].into()]);

        // shape.transform(TSTransform::new(transform));
        let z = transform.bounds().width() as f32 / transform.frame().width();
        // shape.scale(1.0 / z);
        // shape.translate(Vec2::new(t.x, t.y));

        egui::paint_texture_at(
            ui.painter(),
            screen_rect,
            &ImageOptions {
                uv: Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                bg_fill: Color32::RED,
                tint: Color32::BLACK,
                rotation: Some((Rot2::from_angle(0.0), Vec2::splat(0.5))),
                rounding: Rounding::ZERO,
            },
            &(self.shape.texture_id(), screen_rect.size()).into(),
        );
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self) -> Color32 {
        self.stroke.color
    }

    fn highlight(&mut self) {
        self.highlight = true
    }

    fn highlighted(&self) -> bool {
        self.highlight
    }

    fn allow_hover(&self) -> bool {
        self.allow_hover
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let rect = self.shape.visual_bounding_rect();
        PlotBounds::from_min_max(
            [rect.min.x as f64, rect.min.y as f64],
            [rect.max.x as f64, rect.max.y as f64],
        )
    }

    fn id(&self) -> Option<Id> {
        self.id
    }
}

fn compute_bezier3_point(
    t: f64,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> (f64, f64) {
    let x = (1.0 - t).powi(3) * p0.0
        + 3.0 * (1.0 - t).powi(2) * t * p1.0
        + 3.0 * (1.0 - t) * t.powi(2) * p2.0
        + t.powi(3) * p3.0;
    let y = (1.0 - t).powi(3) * p0.1
        + 3.0 * (1.0 - t).powi(2) * t * p1.1
        + 3.0 * (1.0 - t) * t.powi(2) * p2.1
        + t.powi(3) * p3.1;
    (x, y)
}

fn compute_bezier3_derivative(
    t: f64,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> (f64, f64) {
    let x = 3.0 * (1.0 - t).powi(2) * (p1.0 - p0.0)
        + 6.0 * (1.0 - t) * t * (p2.0 - p1.0)
        + 3.0 * t.powi(2) * (p3.0 - p2.0);
    let y = 3.0 * (1.0 - t).powi(2) * (p1.1 - p0.1)
        + 6.0 * (1.0 - t) * t * (p2.1 - p1.1)
        + 3.0 * t.powi(2) * (p3.1 - p2.1);
    (x, y)
}

fn calculate_delta_arc_length(d_t: f64, d_x: f64, d_y: f64) -> f64 {
    (d_x.powi(2) + d_y.powi(2)).sqrt() * d_t
}

fn find_t_for_arc_length(
    mut current_len: f64,
    total_len: f64,
    mut current_t: f64,
    delta_t: f64,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> f64 {
    while current_len < total_len {
        current_t += delta_t;
        let b_prime_t = compute_bezier3_derivative(current_t, p0, p1, p2, p3);
        let delta_len = calculate_delta_arc_length(delta_t, b_prime_t.0, b_prime_t.1);
        current_len += delta_len;
    }
    current_t
}

fn compute_features(
    t: f64,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> ((f64, f64), (f64, f64), f64) {
    let b_t = compute_bezier3_point(t, p0, p1, p2, p3);
    let b_prime_t = compute_bezier3_derivative(t, p0, p1, p2, p3);
    let angle = f64::atan2(b_prime_t.1, b_prime_t.0);
    (b_t, b_prime_t, angle)
}

impl MainApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("Left Panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.label);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                self.ui_content(ui);
            });

            egui::warn_if_debug_build(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl MainApp {
    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(ui.available_width(), 300.0), Sense::hover());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let control_point_radius = 8.0;

        let control_point_shapes: Vec<Shape> = self
            .control_points
            .iter_mut()
            .enumerate()
            .map(|(i, point)| {
                let size = Vec2::splat(2.0 * control_point_radius);

                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
                let point_response = ui.interact(point_rect, point_id, Sense::drag());

                *point += point_response.drag_delta();
                *point = to_screen.from().clamp(*point);

                let point_in_screen = to_screen.transform_pos(*point);
                let stroke = ui.style().interact(&point_response).fg_stroke;

                Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
            })
            .collect();

        let points_in_screen: Vec<Pos2> =
            self.control_points.iter().map(|p| to_screen * *p).collect();

        let points = points_in_screen.clone().try_into().unwrap();
        let shape = CubicBezierShape::from_points_stroke(points, true, self.fill, self.stroke);
        painter.add(shape);
        painter.add(PathShape::line(points_in_screen, self.aux_stroke));
        painter.extend(control_point_shapes);

        response
    }
}
