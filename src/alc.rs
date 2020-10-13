#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::os::raw::{c_char, c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct alc_device {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct alc_context {
    _unused: [u8; 0],
}

pub type alc_boolean = c_char;
pub type alc_char = c_char;
pub type alc_byte = c_char;
pub type alc_ubyte = c_uchar;
pub type alc_short = c_short;
pub type alc_ushort = c_ushort;
pub type alc_int = c_int;
pub type alc_uint = c_uint;
pub type alc_sizei = c_int;
pub type alc_enum = c_int;
pub type alc_float = c_float;
pub type alc_double = c_double;
pub type alc_void = c_void;

pub const ALC_INVALID: alc_enum = 0;
pub const ALC_VERSION_0_1: alc_enum = 1;
pub const ALC_FALSE: alc_enum = 0;
pub const ALC_TRUE: alc_enum = 1;
pub const ALC_FREQUENCY: alc_enum = 4103;
pub const ALC_REFRESH: alc_enum = 4104;
pub const ALC_SYNC: alc_enum = 4105;
pub const ALC_MONO_SOURCES: alc_enum = 4112;
pub const ALC_STEREO_SOURCES: alc_enum = 4113;
pub const ALC_NO_ERROR: alc_enum = 0;
pub const ALC_INVALID_DEVICE: alc_enum = 40961;
pub const ALC_INVALID_CONTEXT: alc_enum = 40962;
pub const ALC_INVALID_ENUM: alc_enum = 40963;
pub const ALC_INVALID_VALUE: alc_enum = 40964;
pub const ALC_OUT_OF_MEMORY: alc_enum = 40965;
pub const ALC_MAJOR_VERSION: alc_enum = 4096;
pub const ALC_MINOR_VERSION: alc_enum = 4097;
pub const ALC_ATTRIBUTES_SIZE: alc_enum = 4098;
pub const ALC_ALL_ATTRIBUTES: alc_enum = 4099;
pub const ALC_DEFAULT_DEVICE_SPECIFIER: alc_enum = 4100;
pub const ALC_DEVICE_SPECIFIER: alc_enum = 4101;
pub const ALC_EXTENSIONS: alc_enum = 4102;
pub const ALC_EXT_CAPTURE: alc_enum = 1;
pub const ALC_CAPTURE_DEVICE_SPECIFIER: alc_enum = 784;
pub const ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER: alc_enum = 785;
pub const ALC_CAPTURE_SAMPLES: alc_enum = 786;
pub const ALC_ENUMERATE_ALL_EXT: alc_enum = 1;
pub const ALC_DEFAULT_ALL_DEVICES_SPECIFIER: alc_enum = 4114;
pub const ALC_ALL_DEVICES_SPECIFIER: alc_enum = 4115;

#[link(name="OpenAL", kind="framework")]
extern "C" {
    #[link_name="alcCreateContext"]
    pub fn alc_create_context(device: *mut alc_device, attrlist: *const alc_int) -> *mut alc_context;

    #[link_name="alcMakeContextCurrent"]
    pub fn alc_make_context_current(context: *mut alc_context) -> alc_boolean;

    #[link_name="alcProcessContext"]
    pub fn alc_process_context(context: *mut alc_context);

    #[link_name="alcSuspendContext"]
    pub fn alc_suspend_context(context: *mut alc_context);

    #[link_name="alcDestroyContext"]
    pub fn alc_destroy_context(context: *mut alc_context);

    #[link_name="alcGetCurrentContext"]
    pub fn alc_get_current_context() -> *mut alc_context;

    #[link_name="alcGetContextsDevice"]
    pub fn alc_get_contexts_device(context: *mut alc_context) -> *mut alc_device;

    #[link_name="alcOpenDevice"]
    pub fn alc_open_device(devicename: *const alc_char) -> *mut alc_device;

    #[link_name="alcCloseDevice"]
    pub fn alc_close_device(device: *mut alc_device) -> alc_boolean;

    #[link_name="alcGetError"]
    pub fn alc_get_error(device: *mut alc_device) -> alc_enum;

    #[link_name="alcIsExtensionPresen"]
    pub fn alc_is_extension_present(device: *mut alc_device, extname: *const alc_char) -> alc_boolean;

    #[link_name="alcGetProcAddress"]
    pub fn alc_get_proc_address(device: *mut alc_device, fname: *const alc_char) -> *mut alc_void;

    #[link_name="alcGetEnumValue"]
    pub fn alc_get_enum_value(device: *mut alc_device, enumname: *const alc_char) -> alc_enum;

    #[link_name="alcGetString"]
    pub fn alc_get_string(device: *mut alc_device, param: alc_enum) -> *const alc_char;

    #[link_name="alcGetIntegerv"]
    pub fn alc_get_integerv(
        device: *mut alc_device,
        param: alc_enum,
        size: alc_sizei,
        values: *mut alc_int,
    );

    #[link_name="alcCreateContext"]
    pub fn alc_capture_open_device(
        devicename: *const alc_char,
        frequency: alc_uint,
        format: alc_enum,
        buffersize: alc_sizei,
    ) -> *mut alc_device;

    #[link_name="alcCaptureCloseDevice"]
    pub fn alc_capture_close_device(device: *mut alc_device) -> alc_boolean;

    #[link_name="alcCaptureStart"]
    pub fn alc_capture_start(device: *mut alc_device);

    #[link_name="alcCreateContext"]
    pub fn alc_capture_stop(device: *mut alc_device);

    #[link_name="alcCaptureSamples"]
    pub fn alc_capture_samples(device: *mut alc_device, buffer: *mut alc_void, samples: alc_sizei);
}
