// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]

extern crate gdk_pixbuf_sys;
extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
#[macro_use]
extern crate glib;
extern crate gio;
extern crate libc;

mod auto;

mod pixbuf_animation;
mod pixbuf_animation_iter;
mod pixbuf;
pub mod prelude;

pub use auto::*;

pub use self::pixbuf_animation_iter::PixbufAnimationIter;
