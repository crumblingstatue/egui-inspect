#![feature(specialization)]

mod inspect;
mod ui_ext;

/// Re-export of egui. Derive codegen refers to this.
pub use egui;
#[cfg(feature = "derive")]
pub use egui_inspect_derive as derive;
pub use {inspect::Inspect, ui_ext::UiExt};

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
