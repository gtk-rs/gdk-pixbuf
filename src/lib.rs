// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]

extern crate gdk_pixbuf_sys as ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;
#[macro_use]
extern crate glib;
extern crate gio;
extern crate libc;

#[cfg(feature = "futures")]
extern crate fragile;
#[cfg(feature = "futures")]
extern crate futures_core;

pub use glib::{
    Cast,
    Continue,
    Error,
    IsA,
    Object,
    StaticType,
    ToValue,
    Type,
    TypedValue,
    Value,
};

mod auto;

mod animation;
mod pixbuf;
pub mod prelude;

pub use auto::*;

pub use self::animation::{
    PixbufAnimation,
    PixbufAnimationIter,
    PixbufAnimationExt,
};
