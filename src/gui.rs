use crate::sfo::Sfo;
use eframe::egui::{self};

pub struct GuiApp {
  sfo: Sfo,
}

impl GuiApp {
  pub fn new(_cc: &eframe::CreationContext<'_>, sfo: Sfo) -> Self {
    GuiApp { sfo }
  }

  fn mapping_entries_grid(&self, ui: &mut eframe::egui::Ui) {
    egui::Grid::new("mapping_grid")
      .num_columns(2)
      .min_col_width(120.0)
      .max_col_width(ui.available_size().x)
      .spacing([40.0, 4.0])
      .striped(true)
      .show(ui, |ui| {
        ui.label("KEY");
        ui.label("DATA");
        ui.end_row();

        ui.label("");
        ui.add_sized(ui.available_size(), egui::Label::new(""));
        ui.end_row();

        for (key, entry) in self.sfo.iter() {
          ui.label(key.to_string())
            .on_hover_text(entry.index_table_entry.to_string());
          ui.label(entry.data.to_string());
          ui.end_row();
        }
      });
  }
}

impl eframe::App for GuiApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::ScrollArea::both().show(ui, |ui| {
        self.mapping_entries_grid(ui);
      });
    });
  }
}
