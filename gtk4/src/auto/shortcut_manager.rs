// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct ShortcutManager(Interface<ffi::GtkShortcutManager>);

    match fn {
        get_type => || ffi::gtk_shortcut_manager_get_type(),
    }
}

pub const NONE_SHORTCUT_MANAGER: Option<&ShortcutManager> = None;

pub trait ShortcutManagerExt: 'static {}

impl<O: IsA<ShortcutManager>> ShortcutManagerExt for O {}

impl fmt::Display for ShortcutManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ShortcutManager")
    }
}
