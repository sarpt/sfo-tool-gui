use eframe::egui::{self};

use crate::sfo::keys::Keys;

pub struct DeleteEntryDialog {
  pub key: Keys,
}

impl DeleteEntryDialog {
  pub fn new(key: Keys) -> Self {
    DeleteEntryDialog { key }
  }

  pub fn show(&self, ctx: &eframe::egui::Context) -> Option<bool> {
    egui::Modal::new(egui::Id::new("delete_entry_dialog"))
      .show(ctx, |ui| {
        ui.set_width(250.0);
        ui.heading(format!("Delete {}?", self.key));
        ui.label(format!(
          "Are you sure you want to delete entry under key {}?",
          self.key
        ));
        ui.separator();

        ui.horizontal(|ui| {
          let ok_btn = ui.button("Ok");
          if ok_btn.clicked() {
            return Some(true);
          }

          let cancel_btn = ui.button("Cancel");
          if cancel_btn.clicked() {
            return Some(false);
          }

          None
        })
        .inner
      })
      .inner
  }
}
