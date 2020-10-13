mod al;
mod alc;

pub use al::*;
pub use alc::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_alc_open_device() {
        let device = unsafe {
            alc_open_device(ptr::null())
        };

        assert_eq!(device.is_null(), false);

        unsafe {
            alc_close_device(device);
        }
    }

    #[test]
    fn test_alc_create_context() {
        let device = unsafe {
            alc_open_device(ptr::null())
        };

        assert_eq!(device.is_null(), false);

        let context = unsafe {
            alc_create_context(device, ptr::null())
        };

        assert_eq!(device.is_null(), false);

        unsafe {
            alc_destroy_context(context);
            alc_close_device(device);
        }
    }
}
