use egui::Ui;
use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    fmt::Debug,
};

#[cfg(feature = "derive")]
pub use egui_inspect_derive as derive;

pub trait Inspect: Debug {
    fn inspect(&self, ui: &mut Ui, id_source: u64);
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        self.inspect(ui, id_source);
    }
}

impl Inspect for String {
    fn inspect_mut(&mut self, ui: &mut Ui, _id_source: u64) {
        ui.text_edit_singleline(self);
    }

    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        ui.label(self);
    }
}

impl<T: Inspect> Inspect for Vec<T> {
    fn inspect_mut(&mut self, ui: &mut Ui, mut id_source: u64) {
        ui.inspect_iter_with_mut(
            &format!("Vec [{}]", self.len()),
            self,
            &mut id_source,
            |ui, i, item, _id_source| {
                ui.horizontal(|ui| {
                    if ui
                        .add(egui::Label::new(i.to_string()).sense(egui::Sense::click()))
                        .clicked()
                    {
                        ui.output().copied_text = format!("{:?}", item);
                    }
                    item.inspect_mut(ui, i as u64);
                });
            },
        );
    }

    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        egui::CollapsingHeader::new(format!("Vec [{}]", self.len()))
            .id_source(id_source)
            .show(ui, |ui| {
                for (i, item) in self.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(i.to_string()).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output().copied_text = format!("{:?}", item);
                        }
                        item.inspect(ui, i as u64);
                    });
                }
            });
    }
}

impl<T: Inspect> Inspect for Option<T> {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        match self {
            None => {
                ui.label("None");
            }
            Some(t) => {
                t.inspect_mut(ui, id_source);
            }
        }
    }

    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        match self {
            None => {
                ui.label("None");
            }
            Some(t) => {
                t.inspect(ui, id_source);
            }
        }
    }
}

impl Inspect for OsString {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        self.inspect(ui, id_source);
    }

    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        ui.label(format!("(OsString) {}", self.to_string_lossy()));
    }
}

impl<T: Inspect> Inspect for HashSet<T> {
    fn inspect(&self, ui: &mut Ui, mut id_source: u64) {
        egui::CollapsingHeader::new("HashSet")
            .id_source(id_source)
            .show(ui, |ui| {
                for item in self.iter() {
                    ui.inspect(item, &mut id_source);
                }
            });
    }
}

impl<T: Inspect> Inspect for &mut T {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        (*self).inspect_mut(ui, id_source)
    }

    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        (**self).inspect(ui, id_source)
    }
}

impl<T: Inspect, const N: usize> Inspect for [T; N] {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        egui::CollapsingHeader::new(format!("array[{}]", self.len()))
            .id_source(id_source)
            .show(ui, |ui| {
                for (i, item) in self.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(i.to_string()).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output().copied_text = format!("{:?}", item);
                        }
                        item.inspect_mut(ui, i as u64);
                    });
                }
            });
    }

    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        egui::CollapsingHeader::new(format!("array[{}]", self.len()))
            .id_source(id_source)
            .show(ui, |ui| {
                for (i, item) in self.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(i.to_string()).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output().copied_text = format!("{:?}", item);
                        }
                        item.inspect(ui, i as u64);
                    });
                }
            });
    }
}

impl<K: Debug, V: Inspect> Inspect for HashMap<K, V> {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        egui::CollapsingHeader::new(format!("HashMap [{}]", self.len()))
            .id_source(id_source)
            .show(ui, |ui| {
                for (i, (k, v)) in self.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(format!("{:?}", k)).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output().copied_text = format!("{:?}", v);
                        }
                        v.inspect_mut(ui, i as u64);
                    });
                }
            });
    }

    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        egui::CollapsingHeader::new(format!("HashMap [{}]", self.len()))
            .id_source(id_source)
            .show(ui, |ui| {
                for (i, (k, v)) in self.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::Label::new(format!("{:?}", k)).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.output().copied_text = format!("{:?}", v);
                        }
                        v.inspect(ui, i as u64);
                    });
                }
            });
    }
}

impl<'a> Inspect for &'a str {
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        if ui
            .add(egui::Label::new(*self).sense(egui::Sense::click()))
            .clicked()
        {
            ui.output().copied_text = self.to_string();
        }
    }
}

impl Inspect for bool {
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        let mut value = *self;
        ui.checkbox(&mut value, "");
    }
    fn inspect_mut(&mut self, ui: &mut Ui, _id_source: u64) {
        ui.checkbox(self, "");
    }
}

macro_rules! impl_num_inspect {
    ($($ty:ty),*) => {
        $(impl Inspect for $ty {
            fn inspect_mut(&mut self, ui: &mut Ui, _id_source: u64) {
                ui.add(egui::DragValue::new(self));
            }
            fn inspect(&self, ui: &mut Ui, _id_source: u64) {
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
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        self.0.inspect_mut(ui, id_source);
        self.1.inspect_mut(ui, id_source);
    }

    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        self.0.inspect(ui, id_source);
        self.1.inspect(ui, id_source);
    }
}

pub trait UiExt {
    fn inspect<T: Inspect>(&mut self, what: &T, id_source: &mut u64);
    fn inspect_iter_with<'a, I, T, F>(
        &mut self,
        title: &str,
        into_iter: I,
        id_source: &mut u64,
        fun: F,
    ) where
        I: IntoIterator<Item = &'a T>,
        T: 'a,
        F: FnMut(&mut Ui, usize, &T, &mut u64);
    fn inspect_iter_with_mut<'a, I, T, F>(
        &mut self,
        title: &str,
        into_iter: I,
        id_source: &mut u64,
        fun: F,
    ) where
        I: IntoIterator<Item = &'a mut T>,
        T: 'a,
        F: FnMut(&mut Ui, usize, &mut T, &mut u64);
    fn inspect_mut<T: Inspect>(&mut self, what: &mut T, id_source: &mut u64);
    fn property<T: Inspect>(&mut self, name: &str, what: &mut T, id_source: &mut u64);
}

macro_rules! inspect_iter_with_body {
    ($self:expr, $title:expr, $into_iter:expr, $id_source:expr, $fun:expr) => {
        egui::CollapsingHeader::new($title)
            .id_source(*$id_source)
            .show($self, |ui| {
                for (i, item) in $into_iter.into_iter().enumerate() {
                    $fun(ui, i, item, $id_source);
                }
            });
    };
}

impl UiExt for Ui {
    fn inspect<T: Inspect>(&mut self, what: &T, id_source: &mut u64) {
        what.inspect(self, *id_source);
        *id_source += 1;
    }
    fn inspect_iter_with<'a, I, T, F>(
        &mut self,
        title: &str,
        into_iter: I,
        id_source: &mut u64,
        mut fun: F,
    ) where
        I: IntoIterator<Item = &'a T>,
        T: 'a,
        F: FnMut(&mut Ui, usize, &T, &mut u64),
    {
        inspect_iter_with_body!(self, title, into_iter, id_source, fun);
    }
    fn inspect_iter_with_mut<'a, I, T, F>(
        &mut self,
        title: &str,
        into_iter: I,
        id_source: &mut u64,
        mut fun: F,
    ) where
        I: IntoIterator<Item = &'a mut T>,
        T: 'a,
        F: FnMut(&mut Ui, usize, &mut T, &mut u64),
    {
        inspect_iter_with_body!(self, title, into_iter, id_source, fun);
    }
    fn inspect_mut<T: Inspect>(&mut self, what: &mut T, id_source: &mut u64) {
        what.inspect_mut(self, *id_source);
        *id_source += 1;
    }
    fn property<T: Inspect>(&mut self, name: &str, what: &mut T, id_source: &mut u64) {
        self.horizontal(|ui| {
            if ui
                .add(egui::Label::new(name).sense(egui::Sense::click()))
                .clicked()
            {
                ui.output().copied_text = format!("{:?}", what);
            }
            ui.inspect_mut(what, id_source);
        });
    }
}

#[macro_export]
macro_rules! inspect {(
    $ui:expr, $($rest:tt)*
) => ({
    let mut id_source = 0;
    $crate::inspect_helper! { $ui id_source $($rest)* }
})}

#[macro_export]
macro_rules! inspect_helper {
    ($ui:tt $id_source:tt) => ();

    (
        $ui:tt $id_source:tt
        $name:literal : $arg:expr $(, $($rest:tt)* )?
    ) => (
        $crate::UiExt::property(
            $ui, $name, &mut $arg, &mut $id_source
        );
        $($crate::inspect_helper! {
            $ui $id_source $($rest)*
        })?
    );

    (
        $ui:tt $id_source:tt
        $arg:expr $(, $($rest:tt)* )?
    ) => (
        $crate::UiExt::property(
            $ui, ::core::stringify!($arg), &mut $arg, &mut $id_source
        );
        $($crate::inspect_helper! {
            $ui $id_source $($rest)*
        })?
    );
}
