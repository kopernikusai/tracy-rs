#[macro_export]
macro_rules! cstr {
    ( $( $str: expr ),* ) => (unsafe{ ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($( $str ),* , "\0").as_bytes()) })
}

#[macro_export]
macro_rules! loc {
    () => {{
        use std::ffi::CStr;
        $crate::SourceLocation {
            //https://github.com/z33ky/tracy-rs/issues/1
            function: $crate::cstr!(module_path!(), "::fn(?)"),
            file: $crate::cstr!(file!()),
            line: line!(),
        }
    }};
}

#[cfg(not(feature = "nightly"))]
#[macro_export]
macro_rules! src_loc_data {
    () => {
        $crate::SourceLocationData::without_name_or_color(&$crate::loc!())
    };

    (name=$name: expr) => {
        $crate::SourceLocationData::with_name(&$crate::loc!(), $crate::cstr!($name))
    };

    (name=$name: expr, color=$color: expr) => {
        $crate::SourceLocationData::with_name_and_color(
            &$crate::loc!(),
            $crate::cstr!($name),
            $color,
        )
    };

    (color=$color: expr) => {
        $crate::SourceLocationData::with_color(&$crate::loc!(), $color)
    };
}

#[cfg(feature = "nightly")]
#[macro_export]
macro_rules! src_loc_data {
    () => {{
        const LOC: $crate::SourceLocationData =
            $crate::SourceLocationData::without_name_or_color(&$crate::loc!());
        LOC
    }};

    (name=$name: expr) => {{
        const LOC: $crate::SourceLocationData =
            $crate::SourceLocationData::with_name(&$crate::loc!(), $crate::cstr!($name));
        LOC
    }};

    (name=$name: expr, color=$color: expr) => {{
        const LOC: $crate::SourceLocationData = $crate::SourceLocationData::with_name_and_color(
            &$crate::loc!(),
            $crate::cstr!($name),
            $color,
        );
        LOC
    }};

    (color=$color: expr) => {{
        const LOC: $crate::SourceLocationData =
            $crate::SourceLocationData::with_color(&$crate::loc!(), $color);
        LOC
    }};
}

#[macro_export]
macro_rules! zone {
    ($active: expr) => {
        let _zone = $crate::ZoneContext::new(&$crate::src_loc_data!(), $active);
    };
}

#[macro_export]
macro_rules! zone_n {
    ($name: expr, $active: expr) => {
        let _zone = $crate::ZoneContext::new(&$crate::src_loc_data!(name = $name), $active);
    };
}

#[macro_export]
macro_rules! zone_c {
    ($color: expr, $active: expr) => {
        let _zone = $crate::ZoneContext::new(&$crate::src_loc_data!(color = $color), $active);
    };
}

#[macro_export]
macro_rules! zone_nc {
    ($name: expr, $color: expr, $active: expr) => {
        let _zone = $crate::ZoneContext::new(
            &$crate::src_loc_data!(name = $name, color = $color),
            $active,
        );
    };
}

#[macro_export]
macro_rules! zone_s {
    ($depth: expr, $active: expr) => {
        let _zone = $crate::ZoneContext::with_callstack(&$crate::src_loc_data!(), $depth, $active);
    };
}

#[macro_export]
macro_rules! zone_ns {
    ($name: expr, $depth: expr, $active: expr) => {
        let _zone = $crate::ZoneContext::with_callstack(
            &$crate::src_loc_data!(name = $name),
            $depth,
            $active,
        );
    };
}

#[macro_export]
macro_rules! zone_cs {
    ($color: expr, $depth: expr, $active: expr) => {
        let _zone = $crate::ZoneContext::with_callstack(
            &$crate::src_loc_data!(color = $color),
            $depth,
            $active,
        );
    };
}

#[macro_export]
macro_rules! zone_ncs {
    ($name: expr, $color: expr, $depth: expr, $active: expr) => {
        let _zone = $crate::ZoneContext::with_callstack(
            &$crate::src_loc_data!(name = $name, color = $color),
            $depth,
            $active,
        );
    };
}

#[macro_export]
macro_rules! frame_mark {
    () => {
        unsafe { $crate::sys::___tracy_emit_frame_mark(::core::ptr::null()) };
    };

    ($name: expr) => {
        unsafe { $crate::sys::___tracy_emit_frame_mark($crate::cstr!($name)) };
    };
}

#[macro_export]
macro_rules! frame_mark_start {
    ($name: expr) => {
        unsafe { $crate::sys::___tracy_emit_frame_mark_start($crate::cstr!($name)) };
    };
}

#[macro_export]
macro_rules! frame_mark_end {
    ($name: expr) => {
        unsafe { $crate::sys::___tracy_emit_frame_mark_end($crate::cstr!($name)) };
    };
}

//TODO: bounds-checked variant
#[macro_export]
macro_rules! frame_mark_image_unchecked {
    ($image: expr, $width: expr, $height: expr, $offset: expr, $flip: expr) => {
        $crate::sys::___tracy_emit_frame_mark_image(
            $image as *const ::libc::c_void,
            $width,
            $height,
            $offset,
            if $flip { 1 } else { 0 },
        );
    };
}

#[macro_export]
macro_rules! plot {
    ($name: expr, $value: expr) => {
        unsafe { $crate::sys::___tracy_emit_plot($crate::cstr!($name).as_ptr(), $value) }
    };
}
