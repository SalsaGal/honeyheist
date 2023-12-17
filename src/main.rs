#![windows_subsystem = "windows"]

use app::HoneyApp;
use eframe::NativeOptions;

mod app;
mod bear;
mod config;
mod creator;
mod play;

fn main() {
    let options = NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Honey Heist Helper",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);
            Box::<HoneyApp>::default()
        }),
    )
    .unwrap();
}
