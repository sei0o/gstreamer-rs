// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Container;
use Extractable;
use TimelineElement;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Group(Object<ffi::GESGroup, ffi::GESGroupClass, GroupClass>) @extends Container, TimelineElement, @implements Extractable;

    match fn {
        get_type => || ffi::ges_group_get_type(),
    }
}

impl Group {
    pub fn new() -> Group {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ges_group_new())
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GROUP: Option<&Group> = None;

pub trait GroupExt: 'static {
    fn get_property_duration(&self) -> u64;

    fn set_property_duration(&self, duration: u64);

    fn get_property_in_point(&self) -> u64;

    fn set_property_in_point(&self, in_point: u64);

    fn get_property_max_duration(&self) -> u64;

    fn set_property_max_duration(&self, max_duration: u64);

    fn get_property_priority(&self) -> u32;

    fn set_property_priority(&self, priority: u32);

    fn get_property_start(&self) -> u64;

    fn set_property_start(&self, start: u64);

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_in_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Group>> GroupExt for O {
    fn get_property_duration(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"duration\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_duration(&self, duration: u64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"duration\0".as_ptr() as *const _, Value::from(&duration).to_glib_none().0);
        }
    }

    fn get_property_in_point(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"in-point\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_in_point(&self, in_point: u64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"in-point\0".as_ptr() as *const _, Value::from(&in_point).to_glib_none().0);
        }
    }

    fn get_property_max_duration(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"max-duration\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_max_duration(&self, max_duration: u64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"max-duration\0".as_ptr() as *const _, Value::from(&max_duration).to_glib_none().0);
        }
    }

    fn get_property_priority(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"priority\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_priority(&self, priority: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"priority\0".as_ptr() as *const _, Value::from(&priority).to_glib_none().0);
        }
    }

    fn get_property_start(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"start\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_start(&self, start: u64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"start\0".as_ptr() as *const _, Value::from(&start).to_glib_none().0);
        }
    }

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::duration\0".as_ptr() as *const _,
                transmute(notify_duration_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_in_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::in-point\0".as_ptr() as *const _,
                transmute(notify_in_point_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_max_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::max-duration\0".as_ptr() as *const _,
                transmute(notify_max_duration_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::priority\0".as_ptr() as *const _,
                transmute(notify_priority_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::start\0".as_ptr() as *const _,
                transmute(notify_start_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_duration_trampoline<P>(this: *mut ffi::GESGroup, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Group> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Group::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_in_point_trampoline<P>(this: *mut ffi::GESGroup, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Group> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Group::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_max_duration_trampoline<P>(this: *mut ffi::GESGroup, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Group> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Group::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_priority_trampoline<P>(this: *mut ffi::GESGroup, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Group> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Group::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_start_trampoline<P>(this: *mut ffi::GESGroup, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Group> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Group::from_glib_borrow(this).unsafe_cast())
}
