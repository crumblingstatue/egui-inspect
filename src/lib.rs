pub trait Inspect {
    fn inspect(&mut self, ui: &mut egui::Ui, id_source: u64);
}

impl Inspect for String {
    fn inspect(&mut self, ui: &mut egui::Ui, _id_source: u64) {
        ui.text_edit_singleline(self);
    }
}

impl<T: Inspect> Inspect for Vec<T> {
    fn inspect(&mut self, ui: &mut egui::Ui, id_source: u64) {
        egui::CollapsingHeader::new("Vec")
            .id_source(id_source)
            .show(ui, |ui| {
                for (i, item) in self.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(i.to_string());
                        item.inspect(ui, i as u64);
                    });
                }
            });
    }
}

impl Inspect for bool {
    fn inspect(&mut self, ui: &mut egui::Ui, _id_source: u64) {
        ui.checkbox(self, "");
    }
}

macro_rules! impl_num_inspect {
    ($($ty:ty),*) => {
        $(impl Inspect for $ty {
            fn inspect(&mut self, ui: &mut egui::Ui, _id_source: u64) {
                ui.add(egui::DragValue::new(self));
            }
        })*
    };
}

impl_num_inspect!(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64);
