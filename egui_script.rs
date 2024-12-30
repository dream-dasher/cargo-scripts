#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
profile.dev.opt-level = 2
profile.dev.package."*".opt-level = 2
[dependencies]
egui = "0.30.0"
eframe = {version="0.30.0", default-features=false, features=["glow", "wayland"]}
---
//! # egui_script

use eframe::run_native;
use egui::CentralPanel;
fn main() -> std::result::Result <(), eframe::Error>{
        println!("Look, I'm running");
        let native_options = eframe::NativeOptions::default();
        run_native("egui_script", native_options, 
        Box::new( |_cc| Ok(Box::new(MyApp{counter:0}))))?;
        Ok(())
}

struct MyApp {
        counter: u64
 }
impl eframe::App for MyApp {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
                CentralPanel::default().show(ctx, |ui| {
                        ui.heading("imma heading");
                        ui.label("imma label");
                        if ui.button("+7").clicked() {
                                self.counter += 7;
                                if self.counter.rem_euclid(12) == 0 {
                                        self.counter = 0;
                                }
                        }
                ui.label(format!("counter is: {}", self.counter));
                });
        }
}
