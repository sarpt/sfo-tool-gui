use std::str::FromStr;

use eframe::egui::{self, Id, TextBuffer};

use crate::sfo::{keys::Keys, mapping::DataField};

#[derive(Default)]
pub struct EntryUpdateModal {
  pub key: String,
  pub data_field_value: String,
}

impl EntryUpdateModal {
  pub fn show(&mut self, ctx: &eframe::egui::Context) -> Result<Option<DraftEntry>, String> {
    let modal = egui::Modal::new(Id::new("draft_entry_modal")).show(ctx, |ui| {
      ui.set_width(250.0);
      egui::Grid::new("draft_entry_grid")
        .num_columns(2)
        .min_col_width(10.0)
        .max_col_width(ui.available_size().x)
        .spacing([20.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
          ui.label("Key");
          ui.text_edit_singleline(&mut self.key);
          ui.end_row();

          ui.label("Data");
          ui.text_edit_singleline(&mut self.data_field_value);
          ui.end_row();
        });
    });

    if !modal.should_close() {
      return Ok(None);
    }

    if self.key.is_empty() || self.data_field_value.is_empty() {
      return Err(String::from("Cannot add an entry with empty key or field"));
    }

    let draft_entry_key =
      Keys::from_str(&self.key.take()).expect("could not serialize string for draft entry key");
    let draft_entry_field = DataField::Utf8String(self.data_field_value.take());

    Ok(Some(DraftEntry {
      key: draft_entry_key,
      field: draft_entry_field,
    }))
  }
}

pub struct DraftEntry {
  pub key: Keys,
  pub field: DataField,
}

impl Default for DraftEntry {
  fn default() -> Self {
    Self {
      key: Keys::Unknown(String::new()),
      field: DataField::Utf8String(String::new()),
    }
  }
}
