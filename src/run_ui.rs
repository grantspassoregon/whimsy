use crate::prelude::{Addresses, Parcels, TableView};
use egui::{Align, Color32, Context, DragValue, ScrollArea, Slider, TextStyle, Ui};
use itertools::{sorted, Itertools};
use spreadsheet::prelude::BeaData;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct UiState {
    pub addresses: Option<Addresses>,
    pub bea: Option<BeaData>,
    pub bea_panel: Option<BeaPanel>,
    pub bea_select: BeaSelect,
    pub bea_table: Option<TableView<BeaData>>,
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
        let mut bea_table = None;
        let bea = match BeaData::from_csv("p:/bea_cainc5n.csv") {
        // let bea = match BeaData::load("data/bea.data") {
            Ok(data) => {
                bea_panel = Some(BeaPanel::new(&data));
                bea_table = Some(TableView::new(data.clone()));
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
            bea_select: Default::default(),
            bea_table,
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

        // egui::Window::new("Parcels").show(ui, |ui| {
        //     if let Some(data) = &self.parcels {
        //         let row_height = ui.text_style_height(&text_style);
        //         let num_rows = data.records.len();
        //         egui::ScrollArea::vertical().show_rows(
        //             ui,
        //             row_height,
        //             num_rows,
        //             |ui, row_range| {
        //             for row in row_range {
        //                 let record = &data.records[row].owner;
        //                 let name = if let Some(val) = &record.name {
        //                     val.clone()
        //                 } else {
        //                     "None".to_string()
        //                 };
        //                 ui.label(format!("Owner: {}", name));
        //                 ui.label(format!("Map #: {}", &record.id));
        //             }
        //         });
        //     } else {
        //         ui.label("None loaded.");
        //     }
        // });

        // egui::Window::new("Fips").show(ui, |ui| {
        //     if let Some(panel) = &mut self.bea_panel {
        //         panel.fips.show(ui);
        //     }
        // });
        //
        // egui::Window::new("Codes").show(ui, |ui| {
        //     if let Some(panel) = &mut self.bea_panel {
        //         panel.codes.show(ui);
        //     }
        // });
        //
        // egui::Window::new("Years").show(ui, |ui| {
        //     if let Some(panel) = &mut self.bea_panel {
        //         panel.times.show(ui);
        //     }
        // });

        egui::Window::new("Bea").show(ui, |ui| {
            if let Some(table) = &self.bea_table {
                table.show(ui);
            }
        });

        egui::Window::new("Bea Select").show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(panel) = &mut self.bea_panel {
                    ui.horizontal(|ui| {
                        panel.times.combo(ui, "Year".to_string());
                        ui.checkbox(&mut self.bea_select.times, "Adv");
                    });
                    if self.bea_select.times {
                        ui.push_id("times", |ui| {
                            panel.times.show(ui);
                        });
                    }
                    ui.horizontal(|ui| {
                        panel.fips.combo(ui, "Fips".to_string());
                        ui.checkbox(&mut self.bea_select.fips, "Adv");
                    });
                    if self.bea_select.fips {
                        ui.push_id("fips", |ui| {
                            panel.fips.show(ui);
                        });
                    }
                    ui.horizontal(|ui| {
                        panel.codes.combo(ui, "Code".to_string());
                        ui.checkbox(&mut self.bea_select.codes, "Adv");
                    });
                    if self.bea_select.codes {
                        ui.push_id("codes", |ui| {
                            panel.codes.show(ui);
                        });
                    }
                    if ui.button("Search").clicked() {
                        if let Some(year) = panel.times.value {
                            if let Some(mut data) = self.bea.clone() {
                                tracing::info!("Starting data: {}", data.records_ref().len());
                                data = data.filter("year", &year.0.to_string());
                                tracing::info!("Data at year: {}", data.records_ref().len());
                                data = data.filter("location", &panel.fips.value);
                                tracing::info!("Data at place: {}", data.records_ref().len());
                                if let Some(key) = &panel.codes.key {
                                    data = data.filter("code", key);
                                    tracing::info!("Data of type: {}", data.records_ref().len());
                                }
                                self.bea_select.results = Some(TableView::new(data));
                            }
                        }
                    };
                    if let Some(data) = &self.bea_select.results {
                        data.show(ui);
                    }
                }
            });
        });

        egui::Window::new("Sanity Check").show(ui, |ui| {
            if let Some(panel) = &self.bea_panel {
                for (key, value) in &panel.fips.data {
                    ui.label(format!("{}", key));
                    ui.label(value);

                }
            }
        });

    }

}

#[derive(Default, Debug, Clone)]
pub struct BeaSelect {
    codes: bool,
    fips: bool,
    times: bool,
    results: Option<TableView<BeaData>>,
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
    pub times: Panel<Year>,
    pub codes: HashPanel<String, String>,
    pub fips: HashPanel<i32, String>,
}

impl BeaPanel {
    pub fn new(data: &BeaData) -> Self {
        let mut times = data.time_period_keys();
        times.reverse();
        let times = Panel::new(Year::years(&times[..]));
        let mut codes = HashPanel::new(data.linecode_btree());
        // codes.data.sort();
        let fips = HashPanel::new(data.geofips_btree());
        Self {
            times,
            codes,
            fips
        }
    }
}

#[derive(Clone, Debug)]
pub struct HashPanel<K, V> {
    pub data: BTreeMap<K, V>,
    pub selected: usize,
    pub search: String,
    pub value: V,
    pub key: Option<K>,
}

impl<K: Eq + std::hash::Hash + Ord + Clone + std::fmt::Display, V: std::fmt::Display + Clone + Default + Eq> HashPanel<K, V> {

    pub fn new(data: BTreeMap<K, V>) -> Self {
        let selected = 0;
        let search = String::new();
        let value = Default::default();
        let key = Default::default();
        Self {
            data,
            selected,
            search,
            value,
            key,
        }
    }

    pub fn combo(&mut self, ui: &mut Ui, label: String) {
        egui::ComboBox::from_label(label)
            .selected_text(format!("{}", self.value))
            .show_ui(ui, |ui| {
                for (_, val) in &self.data {
                    ui.selectable_value(&mut self.value, val.clone(), format!("{}", val));
                }
            });
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
                                self.key = Some(keys[item].clone());
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
            let key = k.to_string().to_lowercase();
            let val = v.to_string().to_lowercase();
            if key.contains(fragment) | val.contains(fragment) {
                true
            } else {
                false
            }
        });
    }

}

#[derive(Clone, Debug)]
pub struct Panel<T> {
    pub data: Vec<T>,
    pub selected: usize,
    pub search: String,
    pub value: Option<T>,
}

impl<T: PartialEq + Clone + std::fmt::Display + Card> Panel<T> {

    pub fn new(data: Vec<T>) -> Self {
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

    pub fn combo(&mut self, ui: &mut Ui, label: String) {
        let mut selected = if let Some(value) = &self.value {
            value.clone()
        } else {
            self.data[0].clone()
        };
        egui::ComboBox::from_label(label)
            .selected_text(format!("{}", selected))
            .show_ui(ui, |ui| {
                for value in &self.data {
                    ui.selectable_value(&mut selected, value.clone(), format!("{}", value));

                }
            });
        self.value = Some(selected);
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let mut panel = self.clone();
        if !self.search.is_empty() {
            panel.contains(&self.search);
        }
        let num_rows = panel.data.len();
        let mut track_item = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search"));
            if ui.button("X").clicked() {
                self.search = Default::default();
            }
        });
        if num_rows == 0 {
            ui.label("Tracker disabled.");
        } else {
            ui.horizontal(|ui| {
                track_item |= ui.add(Slider::new(&mut self.selected, 0..=(num_rows - 1)))
                    .dragged();
                scroll_top |= ui.button("|<").clicked();
                scroll_bottom |= ui.button(">|").clicked();
            });
        }

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
                                    ui.selectable_value(&mut self.value, Some(panel.data[item].clone()), format!("{}", panel.data[item]));
                                response.scroll_to_me(Some(Align::Center));
                                self.value = Some(panel.data[item].clone());
                            } else {
                                ui.selectable_value(&mut self.value, Some(panel.data[item].clone()), format!("{}", panel.data[item]));
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
        ui.label(
            if let Some(value) = &self.value {
                format!("Value selected: {}", value)
            } else {
                format!("No value selected.")
            });
    }

    pub fn contains(&mut self, fragment: &str) {
        self.data = self.data.iter().filter(|v| v.contains(fragment, SearchConfig::default())).cloned().collect();
    }

}

pub trait Card {
    fn contains(&self, fragment: &str, config: SearchConfig) -> bool;
    fn show(&self, ui: &mut Ui);
}


#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
pub struct SearchConfig {
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Year(i32);

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&i32> for Year {
    fn from(value: &i32) -> Self {
        Self(*value)
    }
}

impl Year {
    pub fn years(values: &[i32]) -> Vec<Self> {
        values.iter().map(|v| Self::from(v)).collect::<Vec<Self>>()

    }
}

impl Card for Year {
    fn contains(&self, fragment: &str, config: SearchConfig) -> bool {
        let mut test = fragment.to_string();
        let mut text = format!("{}", self);
        if !config.case_sensitive {
            test = test.to_lowercase();
            text = text.to_lowercase();
        }
        text.contains(&test)
    }

    fn show(&self, ui: &mut Ui) {
        ui.label(format!("{}", self));
    }
}
