// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use crate::RoundedRect;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct BorderNode(Object<ffi::GskBorderNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_border_node_get_type(),
    }
}

impl BorderNode {
    //#[doc(alias = "gsk_border_node_new")]
    //pub fn new(outline: &RoundedRect, border_width: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 4, border_color: /*Unimplemented*/FixedArray TypeId { ns_id: 11, id: 80 }; 4) -> BorderNode {
    //    unsafe { TODO: call ffi:gsk_border_node_new() }
    //}

    #[doc(alias = "gsk_border_node_peek_colors")]
    pub fn peek_colors(&self) -> Option<gdk::RGBA> {
        unsafe { from_glib_none(ffi::gsk_border_node_peek_colors(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_border_node_peek_outline")]
    pub fn peek_outline(&self) -> Option<RoundedRect> {
        unsafe { from_glib_none(ffi::gsk_border_node_peek_outline(self.to_glib_none().0)) }
    }
}

impl fmt::Display for BorderNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BorderNode")
    }
}
