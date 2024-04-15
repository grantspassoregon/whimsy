use egui::{Align, Layout, Sense, Ui};
use egui_extras::{Column, TableBuilder};
use spreadsheet::prelude::{BeaDatum, BeaData};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableView<T: Tabular> {
    data: T,
    search: String,
    selection: HashSet<usize>,
    target: usize,
}

impl<T: Tabular> TableView<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            search: Default::default(),
            selection: Default::default(),
            target: Default::default(),
        }
    }

    pub fn show(&self, ui: &mut Ui) {
        self.data.table(ui);
    }

}

pub trait Tabular {
    fn table(&self, ui: &mut Ui);
}

impl Tabular for BeaData {
    fn table(&self, ui: &mut Ui) {
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
