use eframe::egui;

pub fn view(ctx: &egui::Context) {
    egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Create new").clicked() {
                    println!("Create new folder in gmh_roots!");
                }
                if ui.button("Test").clicked() {
                }
            });
        })
    });
}