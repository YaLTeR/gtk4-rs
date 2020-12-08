// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ShortcutAction;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct ActivateAction(Object<ffi::GtkActivateAction, ffi::GtkActivateActionClass>) @extends ShortcutAction;

    match fn {
        get_type => || ffi::gtk_activate_action_get_type(),
    }
}

impl ActivateAction {
    #[doc(alias = "gtk_activate_action_get")]
    pub fn get() -> Option<ActivateAction> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_activate_action_get()) }
    }
}

impl fmt::Display for ActivateAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ActivateAction")
    }
}
