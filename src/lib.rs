//#![feature(const_cstr_unchecked)]
#![allow(unused)]

use std::{ffi::CStr, marker::PhantomData, mem::ManuallyDrop};

#[cfg(feature = "enable")]
mod macros;

#[cfg(not(feature = "enable"))]
#[path = "macros_disabled.rs"]
mod macros;

pub use macros::*;

pub type ColorType = u32;
pub mod colors;

pub mod sys {
    #![allow(non_camel_case_types)]
    pub use libc::{c_char, c_int, size_t};
    pub use std::ffi::c_void;

    #[repr(C)]
    pub struct source_location_data {
        pub name: *const c_char,
        pub function: *const c_char,
        pub file: *const c_char,
        pub line: u32,
        pub color: u32,
    }

    #[repr(C)]
    pub struct zone_context {
        pub id: u32,
        pub active: c_int,
    }

    #[link(name = "tracy")]
    extern "C" {
        pub fn ___tracy_emit_zone_begin(
            srcloc: *const source_location_data,
            active: c_int,
        ) -> zone_context;
        pub fn ___tracy_emit_zone_begin_callstack(
            srcloc: *const source_location_data,
            depth: c_int,
            active: c_int,
        ) -> zone_context;
        pub fn ___tracy_emit_zone_end(ctx: zone_context);
        pub fn ___tracy_emit_zone_text(
            ctx: zone_context,
            txt: *const c_char,
            size: size_t,
        );
        pub fn ___tracy_emit_zone_name(
            ctx: zone_context,
            txt: *const c_char,
            size: size_t,
        );

        pub fn ___tracy_emit_frame_mark(name: *const c_char);
        pub fn ___tracy_emit_frame_mark_start(name: *const c_char);
        pub fn ___tracy_emit_frame_mark_end(name: *const c_char);
        pub fn ___tracy_emit_frame_image(
            image: *const c_void,
            w: u16,
            h: u16,
            offset: u8,
            flip: c_int,
        );
        pub fn ___tracy_emit_plot(name: *const c_char, val: f64);
    }
}

pub struct SourceLocation {
    pub function: &'static str,
    pub file: &'static str,
    pub line: u32,
}

pub struct ZoneContext {
    context: ManuallyDrop<sys::zone_context>,
    marker: PhantomData<SourceLocationData>,
}

pub type CallstackDepth = libc::c_int;

impl<'a> ZoneContext {
    #[inline]
    pub fn new(loc: &SourceLocationData, active: bool) -> Self {
        Self {
            context: ManuallyDrop::new(unsafe {
                sys::___tracy_emit_zone_begin(
                    &loc.data as *const _,
                    if active { 1 } else { 0 },
                )
            }),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_callstack(
        loc: &SourceLocationData,
        depth: CallstackDepth,
        active: bool,
    ) -> Self {
        Self {
            context: ManuallyDrop::new(unsafe {
                sys::___tracy_emit_zone_begin_callstack(
                    &loc.data as *const _,
                    depth,
                    if active { 1 } else { 0 },
                )
            }),
            marker: PhantomData,
        }
    }
}

impl Drop for ZoneContext {
    #[inline]
    fn drop(&mut self) {
        //we could avoid this unsafe by manually copying (or deriving Close, though that doesn't
        //seem useful outside this specific use-case), but this is obviously safe also.
        unsafe {
            sys::___tracy_emit_zone_end(ManuallyDrop::take(&mut self.context))
        };
    }
}

pub struct SourceLocationData {
    data: sys::source_location_data,
    marker: PhantomData<&'static str>,
}

impl<'a> SourceLocationData {
    #[inline]
    pub const fn with_name_and_color(
        loc: &SourceLocation,
        name: &'static str,
        color: ColorType,
    ) -> Self {
        Self {
            data: sys::source_location_data {
                name: unsafe { name.as_ptr() as *const sys::c_char },
                function: loc.function.as_ptr() as *const sys::c_char,
                file: loc.file.as_ptr() as *const sys::c_char,
                line: loc.line,
                color,
            },
            marker: PhantomData,
        }
    }

    #[inline]
    pub const fn with_color(loc: &SourceLocation, color: ColorType) -> Self {
        Self {
            data: sys::source_location_data {
                name: std::ptr::null(),
                function: loc.function.as_ptr() as *const sys::c_char,
                file: loc.file.as_ptr() as *const sys::c_char,
                line: loc.line,
                color,
            },
            marker: PhantomData,
        }
    }

    #[inline]
    pub const fn with_name(loc: &SourceLocation, name: &'static str) -> Self {
        Self::with_name_and_color(loc, name, 0)
    }

    #[inline]
    pub const fn without_name_or_color(loc: &SourceLocation) -> Self {
        Self::with_color(loc, 0)
    }
}

#[test]
fn it_works() {
    println!("Generating some test data to verify this basically works...");
    {
        zone_ncs!("it_works", 0, 1, true);

        for i in 0..10 {
            println!("step");
            for i in 0..20_000 {
                let val = (i as f64).sin();
                plot!("counter", val);
                println!("tick: {}", val);
            }
            frame_mark!();
        }
    }

    assert!(true);
}
