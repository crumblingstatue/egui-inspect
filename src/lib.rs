//! Provides an easy way to inspect and manipulate various structs, as well as your
//! own types through egui.
//!
//! Optionally provides a derive macro for your own types as well.

#![feature(specialization)]
#![warn(missing_docs)]

mod inspect;
mod ui_ext;

/// Re-export of egui. Derive codegen refers to this.
pub use egui;
#[cfg(feature = "derive")]
pub use egui_inspect_derive as derive;
pub use {inspect::Inspect, ui_ext::UiExt};

/// Helper macro to help you quickly inspect variables
///
/// Usage:
///
/// ```no_run
/// # let my_local = 42;
/// # let ui: &mut egui::Ui = todo!();
/// # struct MyStruct { field: i32 }
/// # let mystruct = MyStruct { field: 10 };
/// egui_inspect::inspect! {
///     ui, // <- an `egui::Ui` is used for showing the ui
///     my_local, // Supports arbitrary expressions, like locals,
///     2 + 4, // computations,
///     mystruct.field // and field access.
/// }
///
/// ````
#[macro_export]
macro_rules! inspect {(
    $ui:expr, $($rest:tt)*
) => ({
    let mut id_source = 0;
    $crate::_egui_inspect_helper! { $ui id_source $($rest)* }
})}

#[macro_export]
#[doc(hidden)]
macro_rules! _egui_inspect_helper {
    ($ui:tt $id_source:tt) => ();

    (
        $ui:tt $id_source:tt
        $name:literal : $arg:expr $(, $($rest:tt)* )?
    ) => (
        $crate::UiExt::property_mut(
            $ui, $name, &mut $arg, &mut $id_source
        );
        $($crate::_egui_inspect_helper! {
            $ui $id_source $($rest)*
        })?
    );

    (
        $ui:tt $id_source:tt
        $arg:expr $(, $($rest:tt)* )?
    ) => (
        $crate::UiExt::property_mut(
            $ui, ::core::stringify!($arg), &mut $arg, &mut $id_source
        );
        $($crate::_egui_inspect_helper! {
            $ui $id_source $($rest)*
        })?
    );
}
