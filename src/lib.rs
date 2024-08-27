// Copyright 2024 Gavin John
// SPDX-License-Identifier: GPL-3.0-or-later
extern crate ar_drivers;

use ar_drivers::ARGlasses;

#[derive(Debug)]
pub enum Error {
    None,
    IoError,
    HidError,
    SerialPortError,
    NoGlassesFound,
    NotImplemented,
    PacketTimeout,
    NullPointer,
    Other,
    Unknown,
}

#[derive(Debug)]
pub enum DisplayMode {
    Unknown,
    SameOnBoth,
    Stereo,
    HalfSBS,
    HighRefreshRate,
    HighRefreshRateSBS,
}

fn ar_drivers_error_to_error(err: ar_drivers::Error) -> Error {
    match err {
        ar_drivers::Error::IoError(_) => Error::IoError,
        ar_drivers::Error::HidError(_) => Error::HidError,
        ar_drivers::Error::SerialPortError(_) => Error::SerialPortError,
        ar_drivers::Error::NotFound => Error::NoGlassesFound,
        ar_drivers::Error::PacketTimeout => Error::PacketTimeout,
        ar_drivers::Error::Other(_) => Error::Other,
        _ => Error::Unknown,
    }
}

fn ar_drivers_display_mode_to_display_mode(mode: ar_drivers::DisplayMode) -> DisplayMode {
    match mode {
        ar_drivers::DisplayMode::SameOnBoth => DisplayMode::SameOnBoth,
        ar_drivers::DisplayMode::Stereo => DisplayMode::Stereo,
        ar_drivers::DisplayMode::HalfSBS => DisplayMode::HalfSBS,
        ar_drivers::DisplayMode::HighRefreshRate => DisplayMode::HighRefreshRate,
        ar_drivers::DisplayMode::HighRefreshRateSBS => DisplayMode::HighRefreshRateSBS,
    }
}

#[no_mangle]
pub extern "C" fn any_glasses() -> (*mut Box<dyn ARGlasses>, Error) {
    match ar_drivers::any_glasses() {
        Ok(glasses) => (Box::into_raw(Box::new(glasses)), Error::None),
        Err(e) => (std::ptr::null_mut(), ar_drivers_error_to_error(e)),
    }
}

#[no_mangle]
pub extern "C" fn free_glasses(glasses: *mut Box<dyn ARGlasses>) {
    if !glasses.is_null() {
        unsafe {
            Box::from_raw(glasses);
        }
    }
}

#[no_mangle]
pub extern "C" fn get_glasses_serial(glasses: *mut Box<dyn ARGlasses>) -> (*const std::os::raw::c_char, Error) {
    if glasses.is_null() {
        return (std::ptr::null(), Error::NullPointer);
    }
    unsafe {
        let glasses = &mut *glasses;
        match glasses.serial() {
            Ok(serial) => (std::ffi::CString::new(serial).unwrap().into_raw(), Error::None),
            Err(e) => (std::ptr::null(), ar_drivers_error_to_error(e)),
        }
    }
}

#[no_mangle]
pub extern "C" fn get_glasses_name(glasses: *mut Box<dyn ARGlasses>) -> (*const std::os::raw::c_char, Error) {
    if glasses.is_null() {
        return (std::ptr::null(), Error::NullPointer);
    }
    unsafe {
        let glasses = &*glasses;
        (glasses.name().as_ptr() as *const std::os::raw::c_char, Error::None)
    }
}

#[no_mangle]
pub extern "C" fn get_glasses_display_fov(glasses: *mut Box<dyn ARGlasses>) -> (f32, Error) {
    if glasses.is_null() {
        return (0.0, Error::NullPointer);
    }
    unsafe {
        let glasses = &*glasses;
        (glasses.display_fov(), Error::None)
    }
}

#[no_mangle]
pub extern "C" fn get_glasses_display_delay(glasses: *mut Box<dyn ARGlasses>) -> (u64, Error) {
    if glasses.is_null() {
        return (0, Error::NullPointer);
    }
    unsafe {
        let glasses = &*glasses;
        (glasses.display_delay(), Error::None)
    }
}

#[no_mangle]
pub extern "C" fn get_glasses_display_mode(glasses: *mut Box<dyn ARGlasses>) -> (DisplayMode, Error) {
    if glasses.is_null() {
        return (DisplayMode::Unknown, Error::NullPointer);
    }
    unsafe {
        let glasses = &mut *glasses;
        match glasses.get_display_mode() {
            Ok(mode) => (ar_drivers_display_mode_to_display_mode(mode), Error::None),
            Err(e) => (DisplayMode::Unknown, ar_drivers_error_to_error(e)),
        }
    }
}
