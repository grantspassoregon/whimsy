use egui::{Align, Layout, Sense, Slider, Ui};
use egui_extras::{Column, TableBuilder};
use spreadsheet::prelude::{BeaDatum, BeaData};
use std::collections::HashSet;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableView<T: Tabular<U>, U: Columnar> {
    data: T,
    search: String,
    selection: HashSet<usize>,
    target: usize,
    phantom: PhantomData<U>,
}

impl<T: Tabular<U>, U: Columnar> TableView<T, U> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            search: Default::default(),
            selection: Default::default(),
            target: Default::default(),
            phantom: Default::default(),
        }
    }

    fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selection.contains(&row_index) {
                self.selection.remove(&row_index);
            } else {
                self.selection.insert(row_index);
            }
        }
    }

    pub fn table(&mut self, ui: &mut Ui) {
        let mut track_item = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search"));
            if ui.button("X").clicked() {
                self.search = Default::default();
            }
        });
        let num_rows = self.data.len();
        if num_rows == 0 {
            ui.label("Tracker disabled.");
        } else {
            ui.horizontal(|ui| {
                track_item |= ui.add(Slider::new(&mut self.target, 0..=(num_rows - 1)))
                    .dragged();
                scroll_top |= ui.button("|<").clicked();
                scroll_bottom |= ui.button(">|").clicked();
            });
        }
        if scroll_top {
            ui.scroll_to_cursor(Some(Align::TOP));
        }
        if scroll_bottom {
            ui.scroll_to_cursor(Some(Align::BOTTOM));
        }
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(Sense::click())
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto());
        if track_item {
            table = table.scroll_to_row(self.target, Some(Align::Center));
        }


        table
            .header(20.0, |mut header| {
                let names = BeaData::names();
                names.iter().map(|v| header.col(|ui| {
                    ui.strong(v);
                })).for_each(drop);
            })
            .body(|mut body| {
                for (i, record) in self.data.rows().iter().enumerate() {
                    let columns = record.values();
                    body.row(18.0, |mut row| {
                        columns.iter().map(|v| {
                            row.set_selected(self.selection.contains(&i));
                            row.col(|ui| {
                                ui.label(v);
                            });
                            self.toggle_row_selection(i, &row.response());
                        }).for_each(drop);
                    });
                }
            });
    }

    pub fn show(&self, ui: &mut Ui) {
        self.data.table(ui);
    }

}

pub trait Tabular<T: Columnar> {
    fn table(&self, ui: &mut Ui);
    fn headers() -> Vec<String>;
    fn rows(&self) -> Vec<T>;
    fn len(&self) -> usize {
        self.rows().len()
    }
}

impl Tabular<BeaDatum> for BeaData {
    fn headers() -> Vec<String> {
        BeaDatum::names()
    }

    fn rows(&self) -> Vec<BeaDatum> {
        self.records()
    }

    fn table(&self, ui: &mut Ui) {
        let mut track_item = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;
        let mut search = String::new();
        let mut selection: Option<HashSet<usize>> = None;
        let mut target = 0;
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut search).hint_text("Search"));
            if ui.button("X").clicked() {
                search = Default::default();
            }
        });
        let num_rows = self.records_ref().len();
        if num_rows == 0 {
            ui.label("Tracker disabled.");
        } else {
            ui.horizontal(|ui| {
                track_item |= ui.add(Slider::new(&mut target, 0..=(num_rows - 1)))
                    .dragged();
                scroll_top |= ui.button("|<").clicked();
                scroll_bottom |= ui.button(">|").clicked();
            });
        }
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(Sense::click())
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .header(20.0, |mut header| {
                let names = BeaData::names();
                names.iter().map(|v| header.col(|ui| {
                    ui.strong(v);
                })).for_each(drop);
            })
            .body(|mut body| {
                for record in self.records_ref().iter().take(100) {
                    let columns = record.columns();
                    body.row(18.0, |mut row| {
                        columns.iter().map(|v| {
                            row.col(|ui| {
                                ui.label(v);
                            });
                        }).for_each(drop);
                    });
                }
            });
    }

}

pub trait Columnar {
    fn headers() -> Vec<String>;
    fn values(&self) -> Vec<String>;
}

impl Columnar for BeaDatum {
    fn headers() -> Vec<String> {
        Self::names()
    }

    fn values(&self) -> Vec<String> {
        Self::columns(self)
    }
}
