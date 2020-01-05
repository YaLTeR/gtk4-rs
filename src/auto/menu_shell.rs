// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use Buildable;
use Container;
use DirectionType;
use LayoutManager;
use MenuDirectionType;
use MenuItem;
use Overflow;
use Widget;

glib_wrapper! {
    pub struct MenuShell(Object<gtk_sys::GtkMenuShell, gtk_sys::GtkMenuShellClass, MenuShellClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_menu_shell_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct MenuShellBuilder {
    take_focus: Option<bool>,
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
}

impl MenuShellBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MenuShell {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref take_focus) = self.take_focus {
            properties.push(("take-focus", take_focus));
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
        glib::Object::new(MenuShell::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn take_focus(mut self, take_focus: bool) -> Self {
        self.take_focus = Some(take_focus);
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
}

pub const NONE_MENU_SHELL: Option<&MenuShell> = None;

pub trait MenuShellExt: 'static {
    fn activate_item<P: IsA<Widget>>(&self, menu_item: &P, force_deactivate: bool);

    fn append<P: IsA<MenuItem>>(&self, child: &P);

    fn bind_model<P: IsA<gio::MenuModel>>(
        &self,
        model: Option<&P>,
        action_namespace: Option<&str>,
        with_separators: bool,
    );

    fn cancel(&self);

    fn deactivate(&self);

    fn deselect(&self);

    fn get_parent_shell(&self) -> Option<Widget>;

    fn get_selected_item(&self) -> Option<Widget>;

    fn get_take_focus(&self) -> bool;

    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32);

    fn prepend<P: IsA<Widget>>(&self, child: &P);

    fn select_first(&self, search_sensitive: bool);

    fn select_item<P: IsA<Widget>>(&self, menu_item: &P);

    fn set_take_focus(&self, take_focus: bool);

    fn connect_activate_current<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_current(&self, force_hide: bool);

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cancel(&self);

    fn connect_cycle_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cycle_focus(&self, direction: DirectionType);

    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_insert<F: Fn(&Self, &Widget, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_current<F: Fn(&Self, MenuDirectionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_move_current(&self, direction: MenuDirectionType);

    fn connect_move_selected<F: Fn(&Self, i32) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_selection_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_take_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuShell>> MenuShellExt for O {
    fn activate_item<P: IsA<Widget>>(&self, menu_item: &P, force_deactivate: bool) {
        unsafe {
            gtk_sys::gtk_menu_shell_activate_item(
                self.as_ref().to_glib_none().0,
                menu_item.as_ref().to_glib_none().0,
                force_deactivate.to_glib(),
            );
        }
    }

    fn append<P: IsA<MenuItem>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_menu_shell_append(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn bind_model<P: IsA<gio::MenuModel>>(
        &self,
        model: Option<&P>,
        action_namespace: Option<&str>,
        with_separators: bool,
    ) {
        unsafe {
            gtk_sys::gtk_menu_shell_bind_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
                action_namespace.to_glib_none().0,
                with_separators.to_glib(),
            );
        }
    }

    fn cancel(&self) {
        unsafe {
            gtk_sys::gtk_menu_shell_cancel(self.as_ref().to_glib_none().0);
        }
    }

    fn deactivate(&self) {
        unsafe {
            gtk_sys::gtk_menu_shell_deactivate(self.as_ref().to_glib_none().0);
        }
    }

    fn deselect(&self) {
        unsafe {
            gtk_sys::gtk_menu_shell_deselect(self.as_ref().to_glib_none().0);
        }
    }

    fn get_parent_shell(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_shell_get_parent_shell(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_selected_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_shell_get_selected_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_take_focus(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_menu_shell_get_take_focus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            gtk_sys::gtk_menu_shell_insert(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    fn prepend<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_menu_shell_prepend(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn select_first(&self, search_sensitive: bool) {
        unsafe {
            gtk_sys::gtk_menu_shell_select_first(
                self.as_ref().to_glib_none().0,
                search_sensitive.to_glib(),
            );
        }
    }

    fn select_item<P: IsA<Widget>>(&self, menu_item: &P) {
        unsafe {
            gtk_sys::gtk_menu_shell_select_item(
                self.as_ref().to_glib_none().0,
                menu_item.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_take_focus(&self, take_focus: bool) {
        unsafe {
            gtk_sys::gtk_menu_shell_set_take_focus(
                self.as_ref().to_glib_none().0,
                take_focus.to_glib(),
            );
        }
    }

    fn connect_activate_current<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_current_trampoline<P, F: Fn(&P, bool) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            force_hide: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast(),
                from_glib(force_hide),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-current\0".as_ptr() as *const _,
                Some(transmute(activate_current_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate_current(&self, force_hide: bool) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("activate-current", &[&force_hide])
                .unwrap()
        };
    }

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute(cancel_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cancel(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("cancel", &[])
                .unwrap()
        };
    }

    fn connect_cycle_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cycle_focus_trampoline<P, F: Fn(&P, DirectionType) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            direction: gtk_sys::GtkDirectionType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast(),
                from_glib(direction),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-focus\0".as_ptr() as *const _,
                Some(transmute(cycle_focus_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cycle_focus(&self, direction: DirectionType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("cycle-focus", &[&direction])
                .unwrap()
        };
    }

    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn deactivate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deactivate\0".as_ptr() as *const _,
                Some(transmute(deactivate_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_insert<F: Fn(&Self, &Widget, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn insert_trampoline<P, F: Fn(&P, &Widget, i32) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            child: *mut gtk_sys::GtkWidget,
            position: libc::c_int,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(child),
                position,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert\0".as_ptr() as *const _,
                Some(transmute(insert_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_move_current<F: Fn(&Self, MenuDirectionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_current_trampoline<P, F: Fn(&P, MenuDirectionType) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            direction: gtk_sys::GtkMenuDirectionType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast(),
                from_glib(direction),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-current\0".as_ptr() as *const _,
                Some(transmute(move_current_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_current(&self, direction: MenuDirectionType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("move-current", &[&direction])
                .unwrap()
        };
    }

    fn connect_move_selected<F: Fn(&Self, i32) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_selected_trampoline<
            P,
            F: Fn(&P, i32) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut gtk_sys::GtkMenuShell,
            distance: libc::c_int,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast(), distance).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-selected\0".as_ptr() as *const _,
                Some(transmute(move_selected_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selection_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_done_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-done\0".as_ptr() as *const _,
                Some(transmute(selection_done_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_take_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_take_focus_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuShell,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuShell>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::take-focus\0".as_ptr() as *const _,
                Some(transmute(notify_take_focus_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MenuShell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuShell")
    }
}
