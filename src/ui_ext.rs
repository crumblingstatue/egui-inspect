use {crate::inspect::Inspect, egui::Ui};

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

/// Extension trait for `egui::Ui`. Provides helper methods for inspecting values.
pub trait UiExt {
    /// Inspect a single value immutably.
    /// `id_source` is used to generate unique ids for egui.
    fn inspect<T: Inspect>(&mut self, what: &T, id_source: &mut u64);
    /// Inspect a single value mutably.
    /// `id_source` is used to generate unique ids for egui.
    fn inspect_mut<T: Inspect>(&mut self, what: &mut T, id_source: &mut u64);
    /// Inspect an iterator immutably.
    /// `id_source` is used to generate unique ids for egui.
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
    /// Inspect an iterator mutably.
    /// `id_source` is used to generate unique ids for egui.
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
    /// Inspect a struct field mutably.
    /// `id_source` is used to generate unique ids for egui.
    fn property_mut<T: Inspect>(&mut self, name: &str, what: &mut T, id_source: &mut u64);
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
    fn property_mut<T: Inspect>(&mut self, name: &str, what: &mut T, id_source: &mut u64) {
        self.horizontal(|ui| {
            if ui
                .add(egui::Label::new(name).sense(egui::Sense::click()))
                .clicked()
            {
                ui.output_mut(|o| o.copied_text = format!("{:?}", what));
            }
            ui.inspect_mut(what, id_source);
        });
    }
}
