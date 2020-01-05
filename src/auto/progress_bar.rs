// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use Buildable;
use LayoutManager;
use Orientable;
use Orientation;
use Overflow;
use Widget;

glib_wrapper! {
    pub struct ProgressBar(Object<gtk_sys::GtkProgressBar, gtk_sys::GtkProgressBarClass, ProgressBarClass>) @extends Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_progress_bar_get_type(),
    }
}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_progress_bar_new()).unsafe_cast() }
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct ProgressBarBuilder {
    ellipsize: Option<pango::EllipsizeMode>,
    fraction: Option<f64>,
    inverted: Option<bool>,
    pulse_step: Option<f64>,
    show_text: Option<bool>,
    text: Option<String>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
}

impl ProgressBarBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ProgressBar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref ellipsize) = self.ellipsize {
            properties.push(("ellipsize", ellipsize));
        }
        if let Some(ref fraction) = self.fraction {
            properties.push(("fraction", fraction));
        }
        if let Some(ref inverted) = self.inverted {
            properties.push(("inverted", inverted));
        }
        if let Some(ref pulse_step) = self.pulse_step {
            properties.push(("pulse-step", pulse_step));
        }
        if let Some(ref show_text) = self.show_text {
            properties.push(("show-text", show_text));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new(ProgressBar::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn ellipsize(mut self, ellipsize: pango::EllipsizeMode) -> Self {
        self.ellipsize = Some(ellipsize);
        self
    }

    pub fn fraction(mut self, fraction: f64) -> Self {
        self.fraction = Some(fraction);
        self
    }

    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = Some(inverted);
        self
    }

    pub fn pulse_step(mut self, pulse_step: f64) -> Self {
        self.pulse_step = Some(pulse_step);
        self
    }

    pub fn show_text(mut self, show_text: bool) -> Self {
        self.show_text = Some(show_text);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_PROGRESS_BAR: Option<&ProgressBar> = None;

pub trait ProgressBarExt: 'static {
    fn get_ellipsize(&self) -> pango::EllipsizeMode;

    fn get_fraction(&self) -> f64;

    fn get_inverted(&self) -> bool;

    fn get_pulse_step(&self) -> f64;

    fn get_show_text(&self) -> bool;

    fn get_text(&self) -> Option<GString>;

    fn pulse(&self);

    fn set_ellipsize(&self, mode: pango::EllipsizeMode);

    fn set_fraction(&self, fraction: f64);

    fn set_inverted(&self, inverted: bool);

    fn set_pulse_step(&self, fraction: f64);

    fn set_show_text(&self, show_text: bool);

    fn set_text(&self, text: Option<&str>);

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fraction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pulse_step_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ProgressBar>> ProgressBarExt for O {
    fn get_ellipsize(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(gtk_sys::gtk_progress_bar_get_ellipsize(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_fraction(&self) -> f64 {
        unsafe { gtk_sys::gtk_progress_bar_get_fraction(self.as_ref().to_glib_none().0) }
    }

    fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_progress_bar_get_inverted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pulse_step(&self) -> f64 {
        unsafe { gtk_sys::gtk_progress_bar_get_pulse_step(self.as_ref().to_glib_none().0) }
    }

    fn get_show_text(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_progress_bar_get_show_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_progress_bar_get_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pulse(&self) {
        unsafe {
            gtk_sys::gtk_progress_bar_pulse(self.as_ref().to_glib_none().0);
        }
    }

    fn set_ellipsize(&self, mode: pango::EllipsizeMode) {
        unsafe {
            gtk_sys::gtk_progress_bar_set_ellipsize(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_fraction(&self, fraction: f64) {
        unsafe {
            gtk_sys::gtk_progress_bar_set_fraction(self.as_ref().to_glib_none().0, fraction);
        }
    }

    fn set_inverted(&self, inverted: bool) {
        unsafe {
            gtk_sys::gtk_progress_bar_set_inverted(
                self.as_ref().to_glib_none().0,
                inverted.to_glib(),
            );
        }
    }

    fn set_pulse_step(&self, fraction: f64) {
        unsafe {
            gtk_sys::gtk_progress_bar_set_pulse_step(self.as_ref().to_glib_none().0, fraction);
        }
    }

    fn set_show_text(&self, show_text: bool) {
        unsafe {
            gtk_sys::gtk_progress_bar_set_show_text(
                self.as_ref().to_glib_none().0,
                show_text.to_glib(),
            );
        }
    }

    fn set_text(&self, text: Option<&str>) {
        unsafe {
            gtk_sys::gtk_progress_bar_set_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ellipsize_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkProgressBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ProgressBar>,
        {
            let f: &F = &*(f as *const F);
            f(&ProgressBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ellipsize\0".as_ptr() as *const _,
                Some(transmute(notify_ellipsize_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_fraction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fraction_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkProgressBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ProgressBar>,
        {
            let f: &F = &*(f as *const F);
            f(&ProgressBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::fraction\0".as_ptr() as *const _,
                Some(transmute(notify_fraction_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkProgressBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ProgressBar>,
        {
            let f: &F = &*(f as *const F);
            f(&ProgressBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute(notify_inverted_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pulse_step_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pulse_step_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkProgressBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ProgressBar>,
        {
            let f: &F = &*(f as *const F);
            f(&ProgressBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pulse-step\0".as_ptr() as *const _,
                Some(transmute(notify_pulse_step_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkProgressBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ProgressBar>,
        {
            let f: &F = &*(f as *const F);
            f(&ProgressBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-text\0".as_ptr() as *const _,
                Some(transmute(notify_show_text_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkProgressBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ProgressBar>,
        {
            let f: &F = &*(f as *const F);
            f(&ProgressBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ProgressBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProgressBar")
    }
}
