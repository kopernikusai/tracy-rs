#[macro_export]
macro_rules! loc {
    () => {};
}

#[macro_export]
macro_rules! cstr {
    ( $( $str: expr ),* ) => {};
}

//TODO: empty on !cfg(feature = "enable")
#[macro_export]
macro_rules! zone {
    ($active: expr) => {};
}

#[macro_export]
macro_rules! zone_n {
    ($name: expr, $active: expr) => {};
}

#[macro_export]
macro_rules! zone_c {
    ($color: expr, $active: expr) => {};
}

#[macro_export]
macro_rules! zone_nc {
    ($name: expr, $color: expr, $active: expr) => {};
}

#[macro_export]
macro_rules! zone_s {
    ($depth: expr, $active: expr) => {};
}

#[macro_export]
macro_rules! zone_ns {
    ($name: expr, $depth: expr, $active: expr) => {};
}

#[macro_export]
macro_rules! zone_cs {
    ($color: expr, $depth: expr, $active: expr) => {};
}

#[macro_export]
macro_rules! zone_ncs {
    ($name: expr, $color: expr, $depth: expr, $active: expr) => {};
}

#[macro_export]
macro_rules! frame_mark {
    () => {};
}

#[macro_export]
macro_rules! frame_mark_name {
    ($name: expr) => {};
}

#[macro_export]
macro_rules! frame_mark_start {
    ($name: expr) => {};
}

#[macro_export]
macro_rules! frame_mark_end {
    ($name: expr) => {};
}

//TODO: bounds-checked variant
#[macro_export]
macro_rules! frame_mark_image_unchecked {
    ($image: expr, $width: expr, $height: expr, $offset: expr, $flip: expr) => {};
}

#[macro_export]
macro_rules! plot {
    ($name: expr, $val: expr) => {};
}
