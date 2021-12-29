use core::ffi::c_void;
use std::ptr::NonNull;

/// # Safety
/// The IDENTIFIER must match the official identifier for the given extension, otherwise
/// the extension data could be misinterpreted, and UB could occur
pub unsafe trait Extension<'a>: Sized + 'a {
    const IDENTIFIER: *const u8;

    /// # Safety
    /// The extension pointer must be valid
    unsafe fn from_extension_ptr(ptr: NonNull<c_void>) -> &'a Self;
}

/// # Safety
/// The IDENTIFIER must match the official identifier for the given extension, otherwise
/// the extension data could be misinterpreted, and UB could occur
pub unsafe trait ExtensionDescriptor<'a, P>: Extension<'a> {
    type ExtensionInterface: 'static;

    const INTERFACE: &'static Self::ExtensionInterface;

    fn from_implementation() -> &'a Self {
        let ptr = NonNull::from(Self::INTERFACE).cast();
        unsafe { Self::from_extension_ptr(ptr) }
    }
}

pub trait ToShared<'a>: Extension<'a> {
    type Shared: Send + Sync + 'a;

    fn to_shared(&self) -> &Self::Shared;
}
