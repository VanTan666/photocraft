//! Temporary evidence harness: invokes the production Brushes panel and real drag events.
use egui::{accesskit::Role, vec2};
use egui_kittest::{Harness, kittest::Queryable};
use photocraft_ui_egui::{PhotocraftApp, Services, brush_panel::BrushesView, theme::ThemeKind};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::args().nth(1).ok_or("output path required")?;
    let theme = std::env::args().nth(2).and_then(|s| ThemeKind::from_name(&s)).unwrap_or_default();
    let mut app = PhotocraftApp::new(photocraft_engine::Session::new(), Services::default());
    app.run("prefs.set", serde_json::json!({"path":"interface.theme","value":theme.id()}))?;
    app.ui.brushes_panel.view = if std::env::args().nth(3).as_deref() == Some("grid") { BrushesView::Grid } else { BrushesView::List };
    app.ui.panels.brush_settings = true;
    app.ui.brush_tab = 1;
    let name = app.session.tools.presets.first().ok_or("no brush presets")?.name.clone();
    let mut h = Harness::builder().with_size(vec2(1200.0, 900.0)).with_pixels_per_point(1.5).wgpu().build_eframe(move |cc| {
        PhotocraftApp::setup_context(&cc.egui_ctx, theme);
        app
    });
    h.run_steps(4);
    let start = h.query_by_role_and_label(Role::Button, &name).ok_or("brush tile missing")?.rect().center();
    h.hover_at(start);
    h.run_steps(1);
    h.event(egui::Event::PointerButton { pos: start, button: egui::PointerButton::Primary, pressed: true, modifiers: egui::Modifiers::NONE });
    h.run_steps(1);
    h.event(egui::Event::PointerMoved(start + vec2(50.0, 10.0)));
    h.run_steps(4);
    h.render().map_err(std::io::Error::other)?.save(out)?;
    Ok(())
}
