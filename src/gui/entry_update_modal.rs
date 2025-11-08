use std::str::FromStr;

use eframe::egui::{self, Id, TextBuffer};
use egui_typed_input::ValText;

use crate::sfo::{SfoEntry, keys::Keys, mapping::DataField};

pub struct EntryUpdateModal {
  key: String,
  data_field_string_value: String,
  data_field_num_value: u32,
  data_field_variant: DataFieldVariant,
  pub variant: ModalVariant,
}

pub enum ModalVariant {
  Add,
  Edit,
}

pub enum EntryUpdateModalAction {
  Close,
  Noop,
  Save(DraftEntry),
}

enum ModalAction {
  Ok,
  Cancel,
  Noop,
}

#[derive(Copy, Clone, Default, PartialEq)]
pub enum DataFieldVariant {
  #[default]
  Text,
  Number,
}

impl EntryUpdateModal {
  pub fn new_add_entry_modal() -> Self {
    EntryUpdateModal {
      variant: ModalVariant::Add,
      data_field_variant: Default::default(),
      data_field_num_value: Default::default(),
      data_field_string_value: Default::default(),
      key: Default::default(),
    }
  }

  pub fn new_update_entry_modal(key: &Keys, entry: &SfoEntry) -> Self {
    let data_field_variant = match entry.index_table_entry.data_format {
      crate::sfo::format::Format::Utf8 | crate::sfo::format::Format::Utf8Special => {
        DataFieldVariant::Text
      }
      crate::sfo::format::Format::U32 => DataFieldVariant::Number,
    };
    let mut modal = EntryUpdateModal {
      variant: ModalVariant::Edit,
      key: key.to_string(),
      data_field_variant,
      data_field_num_value: Default::default(),
      data_field_string_value: Default::default(),
    };
    match entry.data {
      DataField::Utf8String(text) => {
        modal.data_field_string_value = String::from(text);
      }
      DataField::U32(val) => {
        modal.data_field_num_value = *val;
      }
    };
    modal
  }

  pub fn show(&mut self, ctx: &eframe::egui::Context) -> Result<EntryUpdateModalAction, String> {
    let modal = egui::Modal::new(Id::new("draft_entry_modal")).show(ctx, |ui| {
      ui.set_width(250.0);
      match self.variant {
        ModalVariant::Add => ui.heading("Add entry"),
        ModalVariant::Edit => ui.heading("Edit entry"),
      };

      ui.horizontal(|ui| {
        ui.radio_value(&mut self.data_field_variant, DataFieldVariant::Text, "Text");
        ui.radio_value(
          &mut self.data_field_variant,
          DataFieldVariant::Number,
          "Number",
        );
      });
      egui::Grid::new("draft_entry_grid")
        .num_columns(2)
        .min_col_width(10.0)
        .max_col_width(ui.available_size().x)
        .spacing([20.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
          ui.label("Key");
          match self.variant {
            ModalVariant::Add => {
              ui.text_edit_singleline(&mut self.key);
            },
            ModalVariant::Edit => {
              ui.label(&self.key).on_hover_text("Key edit is disabled when editing row. Please either add or remove an entry with a new key");
            },
          };
          ui.end_row();

          ui.label("Data");
          match self.data_field_variant {
            DataFieldVariant::Text => {
              ui.text_edit_singleline(&mut self.data_field_string_value);
            }
            DataFieldVariant::Number => {
              let mut num_input = ValText::<u32, <u32 as FromStr>::Err>::number_uint();
              num_input.set_val(self.data_field_num_value);
              ui.text_edit_singleline(&mut num_input);
              if let Some(Ok(new_val)) = num_input.get_val() {
                self.data_field_num_value = *new_val;
              };
            }
          }
          ui.end_row();
        });

      ui.separator();

      ui.horizontal(|ui| {
        let value_filled = match self.data_field_variant {
          DataFieldVariant::Text => !self.data_field_string_value.is_empty(),
          DataFieldVariant::Number => true,
        };
        let ok_btn = ui
          .add_enabled(
            !self.key.is_empty() && value_filled,
            egui::Button::new("Ok"),
          )
          .on_disabled_hover_text("Cannot add an entry with empty key or field");
        if ok_btn.clicked() {
          return ModalAction::Ok;
        }

        let cancel_btn = ui.button("Cancel");
        if cancel_btn.clicked() {
          return ModalAction::Cancel;
        }

        ModalAction::Noop
      })
      .inner
    });

    match modal.inner {
      ModalAction::Ok => {
        let draft_entry_key =
          Keys::from_str(&self.key.take()).expect("could not serialize string for draft entry key");
        let draft_entry_field = match self.data_field_variant {
          DataFieldVariant::Text => DataField::Utf8String(self.data_field_string_value.take()),
          DataFieldVariant::Number => DataField::U32(self.data_field_num_value),
        };

        return Ok(EntryUpdateModalAction::Save(DraftEntry {
          key: draft_entry_key,
          field: draft_entry_field,
        }));
      }
      ModalAction::Cancel => {
        return Ok(EntryUpdateModalAction::Close);
      }
      ModalAction::Noop => {}
    }

    if modal.should_close() {
      return Ok(EntryUpdateModalAction::Close);
    }

    Ok(EntryUpdateModalAction::Noop)
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
