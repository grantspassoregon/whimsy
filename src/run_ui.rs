use crate::prelude::{Addresses, Parcels};
use egui::{Align, Color32, Context, DragValue, ScrollArea, Slider, TextStyle, Ui};
use itertools::sorted;
use spreadsheet::prelude::BeaData;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct UiState {
    pub addresses: Option<Addresses>,
    pub bea: Option<BeaData>,
    pub bea_panel: Option<BeaPanel>,
    pub counter: i32,
    pub parcels: Option<Arc<Parcels>>,
}

impl UiState {
    pub fn new() -> Self {
        // let vec = include_bytes!("../data/addresses.data");
        // let addresses: Option<AddressPoints> = match bincode::deserialize(&vec[..]) {
        //     Ok(data) => Some(data),
        //     Err(e) => {
        //         tracing::info!("{:#?}", e.to_string());
        //         None
        //     }
        // };
        let addresses = match Addresses::load("data/addresses.data") {
            Ok(data) => Some(data),
            Err(_) => None,
        };

        let mut bea_panel = None;
        let bea = match BeaData::load("data/bea.data") {
            Ok(data) => {
                bea_panel = Some(BeaPanel::new(&data));
                Some(data)
            },
            Err(e) => {
                tracing::info!("Error loading BEA data: {}", e.to_string());
                None
            }
        };

        let parcels = match Parcels::load("data/parcels.data") {
            Ok(data) => Some(Arc::new(data)),
            Err(_) => None,
        };

        Self {
            addresses,
            bea,
            bea_panel,
            counter: Default::default(),
            parcels,
        }
    }
    pub fn run(&mut self, ui: &Context) {
        egui::Window::new("Whimsy UI").show(ui, |ui| {
            ui.heading("Window");
            if ui.button("Increment").clicked() {
                self.counter += 1;
            }
            ui.label(format!("{}", self.counter));

            let mut address_ct = 0;
            if let Some(data) = &self.addresses {
                address_ct = data.records.len();
            }
            ui.label(format!("Addresses: {}", address_ct));

            let mut bea_ct = 0;
            if let Some(data) = &self.bea {
                bea_ct = data.records_ref().len();
            }
            ui.label(format!("Bea: {}", bea_ct));

            let mut parcel_ct = 0;
            if let Some(data) = &self.parcels {
                parcel_ct = data.records.len();
            }
            ui.label(format!("Parcels: {}", parcel_ct));
        });
        
        let text_style = TextStyle::Body;

        egui::Window::new("Parcels").show(ui, |ui| {
            if let Some(data) = &self.parcels {
                let row_height = ui.text_style_height(&text_style);
                let num_rows = data.records.len();
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    num_rows,
                    |ui, row_range| {
                    for row in row_range {
                        let record = &data.records[row].owner;
                        let name = if let Some(val) = &record.name {
                            val.clone()
                        } else {
                            "None".to_string()
                        };
                        ui.label(format!("Owner: {}", name));
                        ui.label(format!("Map #: {}", &record.id));
                    }
                });
            } else {
                ui.label("None loaded.");
            }
        });

        egui::Window::new("Fips").show(ui, |ui| {
            if let Some(panel) = &mut self.bea_panel {
                panel.fips.show(ui);
            }
        });

        egui::Window::new("Codes").show(ui, |ui| {
            if let Some(panel) = &mut self.bea_panel {
                panel.codes.show(ui);
            }
        });

    }

}

pub fn runner(state: &mut UiState, ui: &Context) {
    egui::Window::new("Whimsy UI").show(ui, |ui| {
        ui.heading("Window");
        if ui.button("Increment").clicked() {
            state.counter += 1;
        }
        ui.label(format!("{}", state.counter));
    });
}

#[derive(Clone, Debug)]
pub struct BeaPanel {
    pub times: Vec<i32>,
    pub codes: HashPanel<String, String>,
    pub fips: HashPanel<i32, String>,
}

impl BeaPanel {
    pub fn new(data: &BeaData) -> Self {
        let mut times = data.time_period_keys();
        times.reverse();
        let codes = HashPanel::new(data.linecode_hash());
        let fips = HashPanel::new(data.geofips_hash());
        Self {
            times,
            codes,
            fips
        }
    }
}

#[derive(Clone, Debug)]
pub struct HashPanel<K, V> {
    pub data: HashMap<K, V>,
    pub selected: usize,
    pub search: String,
    pub value: V,
}

impl<K: Eq + std::hash::Hash + Ord + Clone + std::fmt::Display, V: std::fmt::Display + Clone + Default + Eq> HashPanel<K, V> {

    pub fn new(data: HashMap<K, V>) -> Self {
        let selected = 0;
        let search = String::new();
        let value = Default::default();
        Self {
            data,
            selected,
            search,
            value,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let mut panel = self.clone();
        if !self.search.is_empty() {
            panel.contains(&self.search);
        }
        let keys: Vec<&K> = sorted(panel.data.keys().into_iter()).collect();
        let num_rows = keys.len();
        let mut track_item = false;
        if num_rows == 0 {
            ui.label("Tracker disabled.");
        } else {
            ui.horizontal(|ui| {
                track_item |= ui.add(Slider::new(&mut self.selected, 0..=(num_rows - 1)).text("Track Item"))
                    .dragged();
            });
        }

        let mut scroll_top = false;
        let mut scroll_bottom = false;
        ui.horizontal(|ui| {
            scroll_top |= ui.button("|<").clicked();
            scroll_bottom |= ui.button(">|").clicked();
        });

        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search"));
            if ui.button("X").clicked() {
                self.search = Default::default();
            }

        });
        ui.separator();
        ScrollArea::vertical().max_height(400.)
            .show(ui, |ui| {
                if scroll_top {
                    ui.scroll_to_cursor(Some(Align::TOP));
                }
                ui.vertical(|ui| {
                    if num_rows == 0 {
                        ui.label("No data to display.");
                    } else {
                        for item in 0..=(num_rows - 1) {
                            if track_item && item == self.selected {
                                let response =
                                    ui.selectable_value(&mut self.value, self.data[keys[item]].clone(), format!("{}: {}", keys[item], self.data[keys[item]]));
                                response.scroll_to_me(Some(Align::Center));
                                self.value = self.data[keys[item]].clone();
                            } else {
                                ui.selectable_value(&mut self.value, self.data[keys[item]].clone(), format!("{}: {}", keys[item], self.data[keys[item]]));
                                // ui.label(format!("{}: {}", keys[item], self.data[keys[item]]));
                            }
                        }
                    }
                });

                if scroll_bottom {
                    ui.scroll_to_cursor(Some(Align::BOTTOM));
                }
            })
            .inner;

        ui.separator();
        ui.label(format!("Value selected: {}", self.value));
    }

    pub fn entry_contains(fragment: &str, entry: (&K, &mut V)) -> bool {
        let key_str = entry.0.to_string();
        let val_str = entry.1.to_string();
        if key_str.contains(fragment) | val_str.contains(fragment) {
            true
        } else {
            false
        }
    }

    pub fn contains(&mut self, fragment: &str) {
        self.data.retain(|k, v| {
            let key = k.to_string();
            let val = v.to_string();
            if key.contains(fragment) | val.contains(fragment) {
                true
            } else {
                false
            }
        });
    }

}
