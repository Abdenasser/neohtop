use core_foundation::{
    base::{CFAllocatorRef, CFType, CFTypeRef, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use std::ffi::{c_char, c_void, CStr};

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOServiceMatching(name: *const c_char) -> CFDictionaryRef;
    fn IOServiceGetMatchingServices(
        port: u32,
        matching: CFDictionaryRef,
        iterator: *mut u32,
    ) -> i32;
    fn IOIteratorNext(iterator: u32) -> u32;
    fn IOIteratorIsValid(iterator: u32) -> i32;
    fn IOObjectRelease(object: u32) -> i32;
    fn IORegistryEntryGetRegistryEntryID(entry: u32, id: *mut u64) -> i32;
    fn IORegistryEntryCreateCFProperty(
        entry: u32,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: u32,
    ) -> CFTypeRef;
    fn IORegistryEntryGetName(entry: u32, name: *mut c_char) -> i32;
}

// Each iterator and service returned by IOKit owns a reference.
pub struct IoObject(u32);

impl Drop for IoObject {
    fn drop(&mut self) {
        unsafe { IOObjectRelease(self.0) };
    }
}

impl IoObject {
    pub fn id(&self) -> Option<u64> {
        let mut id = 0;
        (unsafe { IORegistryEntryGetRegistryEntryID(self.0, &mut id) } == 0).then_some(id)
    }

    pub fn name(&self) -> Option<String> {
        let mut buffer = [0_u8; 128];
        // IOKit writes the name into an io_name_t buffer (128 bytes).
        if unsafe { IORegistryEntryGetName(self.0, buffer.as_mut_ptr().cast()) } != 0 {
            return None;
        }
        Some(
            CStr::from_bytes_until_nul(&buffer)
                .ok()?
                .to_string_lossy()
                .into_owned(),
        )
    }

    pub fn property(&self, key: &str) -> Option<CFType> {
        let key = CFString::new(key);
        let value = unsafe {
            IORegistryEntryCreateCFProperty(self.0, key.as_concrete_TypeRef(), std::ptr::null(), 0)
        };
        (!value.is_null()).then(|| unsafe { CFType::wrap_under_create_rule(value) })
    }
}

pub struct IoServices {
    iterator: IoObject,
}

impl IoServices {
    pub fn matching(class: &CStr) -> Option<Self> {
        let matching = unsafe { IOServiceMatching(class.as_ptr()) };
        if matching.is_null() {
            return None;
        }
        let mut raw = 0;
        // IOServiceGetMatchingServices consumes the matching dictionary.
        if unsafe { IOServiceGetMatchingServices(0, matching, &mut raw) } != 0 || raw == 0 {
            return None;
        }
        Some(Self {
            iterator: IoObject(raw),
        })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { IOIteratorIsValid(self.iterator.0) != 0 }
    }
}

impl Iterator for IoServices {
    type Item = IoObject;

    fn next(&mut self) -> Option<Self::Item> {
        let raw = unsafe { IOIteratorNext(self.iterator.0) };
        (raw != 0).then(|| IoObject(raw))
    }
}

pub fn dictionary_number(dict: &CFDictionary, key: &str) -> Option<CFNumber> {
    let key = CFString::new(key);
    let pointer = *dict.find(key.as_concrete_TypeRef().cast::<c_void>())?;
    let value = unsafe { CFType::wrap_under_get_rule(pointer) };
    value.downcast::<CFNumber>()
}
