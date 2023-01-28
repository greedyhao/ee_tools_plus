#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.label("This is a label");
        //     ui.hyperlink("https://github.com/emilk/egui");
        //     ui.text_edit_singleline(&mut my_string);
        //     if ui.button("Click me").clicked() {}
        //     ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0));
        //     ui.add(egui::DragValue::new(&mut my_f32));

        //     ui.checkbox(&mut my_boolean, "Checkbox");

        //     #[derive(PartialEq)]
        //     enum Enum {
        //         First,
        //         Second,
        //         Third,
        //     }
        //     ui.horizontal(|ui| {
        //         ui.radio_value(&mut my_enum, Enum::First, "First");
        //         ui.radio_value(&mut my_enum, Enum::Second, "Second");
        //         ui.radio_value(&mut my_enum, Enum::Third, "Third");
        //     });

        //     ui.separator();

        //     ui.image(my_image, [640.0, 480.0]);

        //     ui.collapsing("Click to see what is hidden!", |ui| {
        //         ui.label("Not much, as it turns out");
        //     });
        // });
    }
}
