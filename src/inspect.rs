use {
    crate::ui_ext::UiExt as _,
    egui::Ui,
    std::{
        collections::{HashMap, HashSet},
        ffi::OsString,
        fmt::Debug,
        marker::PhantomData,
    },
};

/// Trait for inspecting a value of a type through egui.
pub trait Inspect: Debug {
    /// Inspect this value immutably.
    ///
    /// `id_salt` provides a unique id that might be needed by egui.
    fn inspect(&self, ui: &mut Ui, id_salt: u64);
    /// Inspect this value mutably
    ///
    /// `id_salt` provides a unique id that might be needed by egui.
    fn inspect_mut(&mut self, ui: &mut Ui, id_salt: u64) {
        self.inspect(ui, id_salt);
    }
}

impl Inspect for String {
    fn inspect_mut(&mut self, ui: &mut Ui, _id_salt: u64) {
        ui.text_edit_singleline(self);
    }

    fn inspect(&self, ui: &mut Ui, _id_salt: u64) {
        ui.label(self);
    }
}

trait InspectAddUi: Sized {
    fn inspect_add_ui(ui: &mut Ui, vec: &mut Vec<Self>);
}

impl<T> InspectAddUi for T {
    default fn inspect_add_ui(_ui: &mut Ui, _vec: &mut Vec<T>) {}
}

impl<T: Default> InspectAddUi for T {
    fn inspect_add_ui(ui: &mut Ui, vec: &mut Vec<T>) {
        if ui.button("+").clicked() {
            vec.push(T::default())
        }
    }
}

impl<T: Inspect> Inspect for Vec<T> {
    fn inspect_mut(&mut self, ui: &mut Ui, mut id_salt: u64) {
        T::inspect_add_ui(ui, self);
        if ui.button("🗑").on_hover_text("Clear").clicked() {
            self.clear();
        }
        ui.inspect_iter_with_mut(
            &format!("Vec [{}]", self.len()),
            self,
            &mut id_salt,
            |ui, i, item, _id_salt| {
                ui.horizontal(|ui| {
                    if ui
                        .add(egui::Label::new(i.to_string()).sense(egui::Sense::click()))
                        .clicked()
                    {
                        ui.output_mut(|o| o.copied_text = format!("{:?}", item));
                    }
                    item.inspect_mut(ui, i as u64);
                });
            },
        );
    }

    fn inspect(&self, ui: &mut Ui, id_salt: u64) {
        egui::CollapsingHeader::new(format!("Vec [{}]", self.len()))
            .id_salt(id_salt)
            .show(ui, |ui| {
                for (i, item) in self.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(i.to_string()).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output_mut(|o| o.copied_text = format!("{:?}", item));
                        }
                        item.inspect(ui, i as u64);
                    });
                }
            });
    }
}

impl<T: Inspect> Inspect for Option<T> {
    fn inspect_mut(&mut self, ui: &mut Ui, id_salt: u64) {
        match self {
            None => {
                ui.label("None");
            }
            Some(t) => {
                t.inspect_mut(ui, id_salt);
            }
        }
    }

    fn inspect(&self, ui: &mut Ui, id_salt: u64) {
        match self {
            None => {
                ui.label("None");
            }
            Some(t) => {
                t.inspect(ui, id_salt);
            }
        }
    }
}

impl Inspect for OsString {
    fn inspect_mut(&mut self, ui: &mut Ui, id_salt: u64) {
        self.inspect(ui, id_salt);
    }

    fn inspect(&self, ui: &mut Ui, _id_salt: u64) {
        ui.label(format!("(OsString) {}", self.to_string_lossy()));
    }
}

impl<T: Inspect> Inspect for HashSet<T> {
    fn inspect(&self, ui: &mut Ui, mut id_salt: u64) {
        egui::CollapsingHeader::new("HashSet")
            .id_salt(id_salt)
            .show(ui, |ui| {
                for item in self.iter() {
                    ui.inspect(item, &mut id_salt);
                }
            });
    }
}

impl<T: Inspect> Inspect for &mut T {
    fn inspect_mut(&mut self, ui: &mut Ui, id_salt: u64) {
        (*self).inspect_mut(ui, id_salt)
    }

    fn inspect(&self, ui: &mut Ui, id_salt: u64) {
        (**self).inspect(ui, id_salt)
    }
}

impl<T: Inspect, const N: usize> Inspect for [T; N] {
    fn inspect_mut(&mut self, ui: &mut Ui, id_salt: u64) {
        egui::CollapsingHeader::new(format!("array[{}]", self.len()))
            .id_salt(id_salt)
            .show(ui, |ui| {
                for (i, item) in self.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(i.to_string()).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output_mut(|o| o.copied_text = format!("{:?}", item));
                        }
                        item.inspect_mut(ui, i as u64);
                    });
                }
            });
    }

    fn inspect(&self, ui: &mut Ui, id_salt: u64) {
        egui::CollapsingHeader::new(format!("array[{}]", self.len()))
            .id_salt(id_salt)
            .show(ui, |ui| {
                for (i, item) in self.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(i.to_string()).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output_mut(|o| o.copied_text = format!("{:?}", item));
                        }
                        item.inspect(ui, i as u64);
                    });
                }
            });
    }
}

impl<T: Inspect, const N: usize> Inspect for Box<[T; N]> {
    fn inspect(&self, ui: &mut Ui, mut id_salt: u64) {
        let arr: &[T; N] = self;
        ui.inspect(arr, &mut id_salt);
    }
    fn inspect_mut(&mut self, ui: &mut Ui, mut id_salt: u64) {
        let arr: &mut [T; N] = self;
        ui.inspect(arr, &mut id_salt);
    }
}

impl<K: Debug, V: Inspect, S> Inspect for HashMap<K, V, S> {
    fn inspect_mut(&mut self, ui: &mut Ui, id_salt: u64) {
        egui::CollapsingHeader::new(format!("HashMap [{}]", self.len()))
            .id_salt(id_salt)
            .show(ui, |ui| {
                for (i, (k, v)) in self.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(format!("{:?}", k)).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output_mut(|o| o.copied_text = format!("{:?}", v));
                        }
                        v.inspect_mut(ui, i as u64);
                    });
                }
            });
    }

    fn inspect(&self, ui: &mut Ui, id_salt: u64) {
        egui::CollapsingHeader::new(format!("HashMap [{}]", self.len()))
            .id_salt(id_salt)
            .show(ui, |ui| {
                for (i, (k, v)) in self.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(format!("{:?}", k)).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output_mut(|o| o.copied_text = format!("{:?}", v));
                        }
                        v.inspect(ui, i as u64);
                    });
                }
            });
    }
}

impl Inspect for &'_ str {
    fn inspect(&self, ui: &mut Ui, _id_salt: u64) {
        if ui
            .add(egui::Label::new(*self).sense(egui::Sense::click()))
            .clicked()
        {
            ui.output_mut(|o| o.copied_text = self.to_string())
        }
    }
}

impl Inspect for bool {
    fn inspect(&self, ui: &mut Ui, _id_salt: u64) {
        let mut value = *self;
        ui.checkbox(&mut value, "");
    }
    fn inspect_mut(&mut self, ui: &mut Ui, _id_salt: u64) {
        ui.checkbox(self, "");
    }
}

macro_rules! impl_num_inspect {
    ($($ty:ty),*) => {
        $(impl Inspect for $ty {
            fn inspect_mut(&mut self, ui: &mut Ui, _id_salt: u64) {
                ui.add(egui::DragValue::new(self));
            }
            fn inspect(&self, ui: &mut Ui, _id_salt: u64) {
                ui.label(self.to_string());
            }
        })*
    };
}

impl_num_inspect!(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);

impl<T, U> Inspect for (T, U)
where
    T: Inspect,
    U: Inspect,
{
    fn inspect_mut(&mut self, ui: &mut Ui, id_salt: u64) {
        self.0.inspect_mut(ui, id_salt);
        self.1.inspect_mut(ui, id_salt);
    }

    fn inspect(&self, ui: &mut Ui, id_salt: u64) {
        self.0.inspect(ui, id_salt);
        self.1.inspect(ui, id_salt);
    }
}

impl<T> Inspect for PhantomData<T> {
    fn inspect(&self, ui: &mut Ui, _id_salt: u64) {
        ui.label("PhantomData");
    }
}

impl Inspect for () {
    fn inspect(&self, ui: &mut Ui, _id_salt: u64) {
        ui.label("()");
    }
}
