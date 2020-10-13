#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::os::raw::{c_char, c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

pub type al_boolean = c_char;
pub type al_char = c_char;
pub type al_byte = c_char;
pub type al_ubyte = c_uchar;
pub type al_short = c_short;
pub type al_ushort = c_ushort;
pub type al_int = c_int;
pub type al_uint = c_uint;
pub type al_sizei = c_int;
pub type al_enum = c_int;
pub type al_float = c_float;
pub type al_double = c_double;
pub type al_void = c_void;

pub const AL_NONE: al_enum = 0;
pub const AL_FALSE: al_enum = 0;
pub const AL_TRUE: al_enum = 1;
pub const AL_SOURCE_RELATIVE: al_enum = 0x202;
pub const AL_CONE_INNER_ANGLE: al_enum = 0x1001;
pub const AL_CONE_OUTER_ANGLE: al_enum = 0x1002;
pub const AL_PITCH: al_enum = 0x1003;
pub const AL_POSITION: al_enum = 0x1004;
pub const AL_DIRECTION: al_enum = 0x1005;
pub const AL_VELOCITY: al_enum = 0x1006;
pub const AL_LOOPING: al_enum = 0x1007;
pub const AL_BUFFER: al_enum = 0x1009;
pub const AL_GAIN: al_enum = 0x100A;
pub const AL_MIN_GAIN: al_enum = 0x100D;
pub const AL_MAX_GAIN: al_enum = 0x100E;
pub const AL_ORIENTATION: al_enum = 0x100F;
pub const AL_SOURCE_STATE: al_enum = 0x1010;
pub const AL_INITIAL: al_enum = 0x1011;
pub const AL_PLAYING: al_enum = 0x1012;
pub const AL_PAUSED: al_enum = 0x1013;
pub const AL_STOPPED: al_enum = 0x1014;
pub const AL_BUFFERS_QUEUED: al_enum = 0x1015;
pub const AL_BUFFERS_PROCESSED: al_enum = 0x1016;
pub const AL_REFERENCE_DISTANCE: al_enum = 0x1020;
pub const AL_ROLLOFF_FACTOR: al_enum = 0x1021;
pub const AL_CONE_OUTER_GAIN: al_enum = 0x1022;
pub const AL_MAX_DISTANCE: al_enum = 0x1023;
pub const AL_SEC_OFFSET: al_enum = 0x1024;
pub const AL_SAMPLE_OFFSET: al_enum = 0x1025;
pub const AL_BYTE_OFFSET: al_enum = 0x1026;
pub const AL_SOURCE_TYPE: al_enum = 0x1027;
pub const AL_STATIC: al_enum = 0x1028;
pub const AL_STREAMING: al_enum = 0x1029;
pub const AL_UNDETERMINED: al_enum = 0x1030;
pub const AL_FORMAT_MONO8: al_enum = 0x1100;
pub const AL_FORMAT_MONO16: al_enum = 0x1101;
pub const AL_FORMAT_STEREO8: al_enum = 0x1102;
pub const AL_FORMAT_STEREO16: al_enum = 0x1103;
pub const AL_FREQUENCY: al_enum = 0x2001;
pub const AL_BITS: al_enum = 0x2002;
pub const AL_CHANNELS: al_enum = 0x2003;
pub const AL_SIZE: al_enum = 0x2004;
pub const AL_UNUSED: al_enum = 0x2010;
pub const AL_PENDING: al_enum = 0x2011;
pub const AL_PROCESSED: al_enum = 0x2012;
pub const AL_NO_ERROR: al_enum = 0;
pub const AL_INVALID_NAME: al_enum = 0xA001;
pub const AL_INVALID_ENUM: al_enum = 0xA002;
pub const AL_INVALID_VALUE: al_enum = 0xA003;
pub const AL_INVALID_OPERATION: al_enum = 0xA004;
pub const AL_OUT_OF_MEMORY: al_enum = 0xA005;
pub const AL_VENDOR: al_enum = 0xB001;
pub const AL_VERSION: al_enum = 0xB002;
pub const AL_RENDERER: al_enum = 0xB003;
pub const AL_EXTENSIONS: al_enum = 0xB004;
pub const AL_DOPPLER_FACTOR: al_enum = 0xC000;
pub const AL_DOPPLER_VELOCITY: al_enum = 0xC001;
pub const AL_SPEED_OF_SOUND: al_enum = 0xC003;
pub const AL_DISTANCE_MODEL: al_enum = 0xD000;
pub const AL_INVERSE_DISTANCE: al_enum = 0xD001;
pub const AL_INVERSE_DISTANCE_CLAMPED: al_enum = 0xD002;
pub const AL_LINEAR_DISTANCE: al_enum = 0xD003;
pub const AL_LINEAR_DISTANCE_CLAMPED: al_enum = 0xD004;
pub const AL_EXPONENT_DISTANCE: al_enum = 0xD005;
pub const AL_EXPONENT_DISTANCE_CLAMPED: al_enum = 0xD006;

#[link(name="OpenAL", kind="framework")]
extern "C" {
    #[link_name = "alDopplerFactor"]
    pub fn al_doppler_factor(value: al_float);

    #[link_name = "alDopplerVelocity"]
    pub fn al_doppler_velocity(value: al_float);

    #[link_name = "alSpeedOfSound"]
    pub fn al_speed_of_sound(value: al_float);

    #[link_name = "alDistanceModel"]
    pub fn al_distance_model(distance_model: al_enum);

    #[link_name = "alEnable"]
    pub fn al_enable(capability: al_enum);

    #[link_name = "alDisable"]
    pub fn al_disable(capability: al_enum);

    #[link_name = "alIsEnabled"]
    pub fn al_is_enabled(capability: al_enum) -> al_boolean;

    #[link_name = "alGetString"]
    pub fn al_get_string(param: al_enum) -> *const al_char;

    #[link_name = "alGetBooleanv"]
    pub fn al_get_booleanv(param: al_enum, values: *mut al_boolean);

    #[link_name = "alGetIntegerv"]
    pub fn al_get_integerv(param: al_enum, values: *mut al_int);

    #[link_name = "alGetFloatv"]
    pub fn al_get_floatv(param: al_enum, values: *mut al_float);

    #[link_name = "alGetDoublev"]
    pub fn al_get_doublev(param: al_enum, values: *mut al_double);

    #[link_name = "alGetBoolean"]
    pub fn al_get_boolean(param: al_enum) -> al_boolean;

    #[link_name = "alGetInteger"]
    pub fn al_get_integer(param: al_enum) -> al_int;

    #[link_name = "alGetFloat"]
    pub fn al_get_float(param: al_enum) -> al_float;

    #[link_name = "alGetDouble"]
    pub fn al_get_double(param: al_enum) -> al_double;

    #[link_name = "alGetError"]
    pub fn al_get_error() -> al_enum;

    #[link_name = "alIsExtensionPresent"]
    pub fn al_is_extension_present(extname: *const al_char) -> al_boolean;

    #[link_name = "alGetProcAddress"]
    pub fn al_get_proc_address(fname: *const al_char) -> *mut al_void;

    #[link_name = "alGetEnumValue"]
    pub fn al_get_enum_value(ename: *const al_char) -> al_enum;

    #[link_name = "alListenerf"]
    pub fn al_listenerf(param: al_enum, value: al_float);

    #[link_name = "alListener3f"]
    pub fn al_listener3f(param: al_enum, value1: al_float, value2: al_float, value3: al_float);

    #[link_name = "alListenerfv"]
    pub fn al_listenerfv(param: al_enum, values: *const al_float);

    #[link_name = "alListeneri"]
    pub fn al_listeneri(param: al_enum, value: al_int);

    #[link_name = "alListener3i"]
    pub fn al_listener3i(param: al_enum, value1: al_int, value2: al_int, value3: al_int);

    #[link_name = "alListeneriv"]
    pub fn al_listeneriv(param: al_enum, values: *const al_int);

    #[link_name = "alGetListenerf"]
    pub fn al_get_listenerf(param: al_enum, value: *mut al_float);

    #[link_name = "alGetListener3f"]
    pub fn al_get_listener3f(param: al_enum, value1: *mut al_float, value2: *mut al_float, value3: *mut al_float);

    #[link_name = "alGetListenerfv"]
    pub fn al_get_listenerfv(param: al_enum, values: *mut al_float);

    #[link_name = "alGetListeneri"]
    pub fn al_get_listeneri(param: al_enum, value: *mut al_int);

    #[link_name = "alGetListener3i"]
    pub fn al_get_listener3i(param: al_enum, value1: *mut al_int, value2: *mut al_int, value3: *mut al_int);

    #[link_name = "alGetListeneriv"]
    pub fn al_get_listeneriv(param: al_enum, values: *mut al_int);

    #[link_name = "alGenSources"]
    pub fn al_gen_sources(n: al_sizei, sources: *mut al_uint);

    #[link_name = "alDeleteSources"]
    pub fn al_delete_sources(n: al_sizei, sources: *const al_uint);

    #[link_name = "alIsSource"]
    pub fn al_is_source(source: al_uint);

    #[link_name = "alSourcef"]
    pub fn al_sourcef(source: al_uint, param: al_enum, value: al_float);

    #[link_name = "alSource3f"]
    pub fn al_source3f(source: al_uint, param: al_enum, value1: al_float, value2: al_float, value3: al_float);

    #[link_name = "alSourcefv"]
    pub fn al_sourcefv(source: al_uint, param: al_enum, values: *const al_float);

    #[link_name = "alSourcei"]
    pub fn al_sourcei(source: al_uint, param: al_enum, value: al_int);

    #[link_name = "alSource3i"]
    pub fn al_source3i(source: al_uint, param: al_enum, value1: al_int, value2: al_int, value3: al_int);

    #[link_name = "alSourceiv"]
    pub fn al_sourceiv(source: al_uint, param: al_enum, values: *const al_int);

    #[link_name = "alGetSourcef"]
    pub fn al_get_sourcef(source: al_uint, param: al_enum, value: *mut al_float);

    #[link_name = "alGetSource3f"]
    pub fn al_get_source3f(source: al_uint, param: al_enum, value1: *mut al_float, value2: *mut al_float, value3: *mut al_float);

    #[link_name = "alGetSourcefv"]
    pub fn al_get_sourcefv(source: al_uint, param: al_enum, values: *mut al_float);

    #[link_name = "alGetSourcei"]
    pub fn al_get_sourcei(source: al_uint,  param: al_enum, value: *mut al_int);

    #[link_name = "alGetSource3i"]
    pub fn al_get_source3i(source: al_uint, param: al_enum, value1: *mut al_int, value2: *mut al_int, value3: *mut al_int);

    #[link_name = "alGetSourceiv"]
    pub fn al_get_sourceiv(source: al_uint,  param: al_enum, values: *mut al_int);

    #[link_name = "alSourcePlayv"]
    pub fn al_source_playv(n: al_sizei, sources: *const al_uint);

    #[link_name = "alSourceStopv"]
    pub fn al_source_stopv(n: al_sizei, sources: *const al_uint);

    #[link_name = "alSourceRewindv"]
    pub fn al_source_rewindv(n: al_sizei, sources: *const al_uint);

    #[link_name = "alSourcePausev"]
    pub fn al_source_pausev(n: al_sizei, sources: *const al_uint);

    #[link_name = "alSourcePlay"]
    pub fn al_source_play(source: al_uint);

    #[link_name = "alSourceStop"]
    pub fn al_source_stop(source: al_uint);

    #[link_name = "alSourceRewind"]
    pub fn al_source_rewind(source: al_uint);

    #[link_name = "alSourcePause"]
    pub fn al_source_pause(source: al_uint);

    #[link_name = "alSourceQueueBuffers"]
    pub fn al_source_queue_buffers(source: al_uint, n: al_sizei, buffers: *const al_uint);

    #[link_name = "alSourceUnqueueBuffers"]
    pub fn al_source_unqueue_buffers(source: al_uint, n: al_sizei, buffers: *const al_uint);

    #[link_name = "alGenBuffers"]
    pub fn al_gen_buffers(n: al_sizei, buffers: *mut al_uint);

    #[link_name = "alDeleteBuffers"]
    pub fn al_delete_buffers(n: al_sizei, buffers: *const al_uint);

    #[link_name = "alIsBuffer"]
    pub fn al_is_buffer(buffer: al_uint);

    #[link_name = "alBufferData"]
    pub fn al_buffer_data(buffer: al_uint, format: al_enum, data: *const al_void, size: al_sizei, freq: al_sizei);

    #[link_name = "alBufferf"]
    pub fn al_bufferf(buffer: al_uint, param: al_enum, value: al_float);

    #[link_name = "alBuffer3f"]
    pub fn al_buffer3f(buffer: al_uint, param: al_enum, value1: al_float, value2: al_float, value3: al_float);

    #[link_name = "alBufferfv"]
    pub fn al_bufferfv(buffer: al_uint, param: al_enum, values: *const al_float);

    #[link_name = "alBufferi"]
    pub fn al_bufferi(buffer: al_uint, param: al_enum, value: al_int);

    #[link_name = "alBuffer3i"]
    pub fn al_buffer3i(buffer: al_uint, param: al_enum, value1: al_int, value2: al_int, value3: al_int);

    #[link_name = "alBufferiv"]
    pub fn al_bufferiv(buffer: al_uint, param: al_enum, values: *const al_int);

    #[link_name = "alGetBufferf"]
    pub fn al_get_bufferf(buffer: al_uint, param: al_enum, value: *mut al_float);

    #[link_name = "alGetBuffer3f"]
    pub fn al_get_buffer3f(buffer: al_uint, param: al_enum, value1: *mut al_float, value2: *mut al_float, value3: *mut al_float);

    #[link_name = "alGetBufferfv"]
    pub fn al_get_bufferfv(buffer: al_uint, param: al_enum, values: *mut al_float);

    #[link_name = "alGetBufferi"]
    pub fn al_get_bufferi(buffer: al_uint, param: al_enum, value: *mut al_int);

    #[link_name = "alGetBuffer3i"]
    pub fn al_get_buffer3i(buffer: al_uint, param: al_enum, value1: *mut al_int, value2: *mut al_int, value3: *mut al_int);

    #[link_name = "alGetBufferiv"]
    pub fn al_get_bufferiv(buffer: al_uint, param: al_enum, values: *mut al_int);
}
