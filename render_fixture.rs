use photocraft_ui_egui::{PhotocraftApp, i18n};

fn main() {
    let out = std::env::args().nth(1).expect("output PNG");
    let mut h = egui_kittest::Harness::builder()
        .with_size(egui::vec2(1000.0, 740.0))
        .with_pixels_per_point(1.5)
        .with_max_steps(64)
        .wgpu()
        .build_ui_state(
            |ui, _: &mut ()| {
                let ru = i18n::Lang::from_code("ru").expect("Russian catalog");
                ui.heading("PhotoCraft — Russian catalog rendering review");
                ui.add_space(14.0);
                for n in [1, 2, 5, 11, 21, 22] {
                    ui.label(i18n::trn(ru, n, "Imported {n} brush from {group}", "Imported {n} brushes from {group}").replace("{group}", "My Brushes"));
                }
                ui.add_space(14.0);
                ui.label(i18n::trn(ru, 2, "{n} sample point removed", "{n} sample points removed"));
                ui.label(i18n::trn(ru, 22, "Loaded {n} action from {file}", "Loaded {n} actions from {file}").replace("{file}", "actions.atn"));
                ui.add_space(14.0);
                egui::Grid::new("weights").striped(true).show(ui, |ui| {
                    for key in ["ExtraLight", "Light", "ExtraBold", "Black"] {
                        ui.label(key);
                        ui.label(i18n::tr_ctx(ru, "fontWeight", key));
                        ui.end_row();
                    }
                });
                ui.add_space(14.0);
                for key in ["Delete and Fill Selection", "Discard the embedded profile (don't color manage)", "No object found there — drag a rectangle around it"] {
                    ui.label(i18n::tr(ru, key));
                }
            },
            (),
        );
    PhotocraftApp::setup_context(&h.ctx, Default::default());
    h.run_steps(4);
    h.render().expect("render review").save(out).expect("save rendered review");
}
