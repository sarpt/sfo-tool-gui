use crate::sfo::Sfo;
use eframe::egui;

pub struct GuiApp {
  sfo: Sfo,
}

impl GuiApp {
  pub fn new(_cc: &eframe::CreationContext<'_>, sfo: Sfo) -> Self {
    GuiApp { sfo }
  }

  fn mapping_entries_grid(&self, ui: &mut eframe::egui::Ui) {
    for (key, value) in &self.sfo.entries_mapping.entries {
      ui.label(key.to_string());
      ui.label(value.to_string());
      ui.end_row();
    }
  }
}

impl eframe::App for GuiApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::Grid::new("mapping_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
          self.mapping_entries_grid(ui);
        });
    });
  }
}
