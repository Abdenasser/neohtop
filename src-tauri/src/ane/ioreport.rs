use core_foundation::{
    array::CFArray,
    base::{CFAllocatorRef, CFType, CFTypeRef, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef, CFMutableDictionary, CFMutableDictionaryRef},
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use std::{
    cell::RefCell,
    ffi::{c_char, c_void, CStr},
    ptr,
    sync::OnceLock,
};

const DRIVER_BUNDLE: &str = "com.apple.driver.AppleT8152CLPC";
// macOS 27.2.1. Keep the mapping below limited to the tested build.
const TESTED_BUILD: &CStr = c"26B131";
// T8152 ANE energy in nJ, as mapped by macmon and checked on this build.
// https://github.com/vladkens/macmon/blob/main/docs/clpc-discovery.md
const ANE_COUNTER_ID: i64 = (24 << 32) | 0x9380_c6bb;

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOServiceMatching(name: *const c_char) -> CFDictionaryRef;
    fn IOServiceGetMatchingService(port: u32, matching: CFDictionaryRef) -> u32;
    fn IORegistryEntryCreateCFProperty(
        entry: u32,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: u32,
    ) -> CFTypeRef;
    fn IORegistryEntryGetRegistryEntryID(entry: u32, id: *mut u64) -> i32;
    fn IOObjectRelease(object: u32) -> i32;
}

type Dictionary = CFDictionary<CFString, CFType>;
type CopyChannels = unsafe extern "C" fn(u64, u64) -> CFDictionaryRef;
type Subscribe = unsafe extern "C" fn(
    *const c_void,
    CFMutableDictionaryRef,
    *mut CFDictionaryRef,
    u64,
    *const c_void,
) -> CFTypeRef;
type Sample = unsafe extern "C" fn(CFTypeRef, CFDictionaryRef, *const c_void) -> CFDictionaryRef;
type ChannelId = unsafe extern "C" fn(CFDictionaryRef) -> u64;
type ReadInteger = unsafe extern "C" fn(CFDictionaryRef, i32) -> i64;

struct Api {
    copy_channels: CopyChannels,
    subscribe: Subscribe,
    sample: Sample,
    channel_id: ChannelId,
    read_integer: ReadInteger,
}

impl Api {
    fn load() -> Option<Self> {
        // Resolve the private API at runtime so unavailable symbols leave this
        // reading disabled instead of preventing the application from starting.
        let library = unsafe {
            libc::dlopen(
                c"/usr/lib/libIOReport.dylib".as_ptr(),
                libc::RTLD_LAZY | libc::RTLD_LOCAL,
            )
        };
        if library.is_null() {
            return None;
        }
        let functions = [
            c"IOReportCopyAllChannels",
            c"IOReportCreateSubscription",
            c"IOReportCreateSamples",
            c"IOReportChannelGetChannelID",
            c"IOReportSimpleGetIntegerValue",
        ]
        .map(|name| unsafe { libc::dlsym(library, name.as_ptr()) });
        if functions.iter().any(|symbol| symbol.is_null()) {
            unsafe { libc::dlclose(library) };
            return None;
        }
        let [copy, subscribe, sample, id, read] = functions;
        // SAFETY: These are the tested IOReport C signatures. Keep the library
        // open for the lifetime of the process and its sampling threads.
        Some(unsafe {
            Self {
                copy_channels: std::mem::transmute::<*mut c_void, CopyChannels>(copy),
                subscribe: std::mem::transmute::<*mut c_void, Subscribe>(subscribe),
                sample: std::mem::transmute::<*mut c_void, Sample>(sample),
                channel_id: std::mem::transmute::<*mut c_void, ChannelId>(id),
                read_integer: std::mem::transmute::<*mut c_void, ReadInteger>(read),
            }
        })
    }
}

fn driver_id() -> Option<u64> {
    static DRIVER: OnceLock<Option<u64>> = OnceLock::new();
    *DRIVER.get_or_init(|| {
        let mut build = [0_u8; 32];
        let mut length = build.len();
        let code = unsafe {
            libc::sysctlbyname(
                c"kern.osversion".as_ptr(),
                build.as_mut_ptr().cast(),
                &mut length,
                ptr::null_mut(),
                0,
            )
        };
        if code != 0 || CStr::from_bytes_with_nul(build.get(..length)?).ok()? != TESTED_BUILD {
            return None;
        }
        let key = CFString::new("CFBundleIdentifier");
        let mut id = 0;
        // IOKit consumes the matching dictionary. The returned service and
        // property each own one reference, released independently.
        let (status, bundle) = unsafe {
            let matching = IOServiceMatching(c"AppleCLPC".as_ptr());
            if matching.is_null() {
                return None;
            }
            let driver = IOServiceGetMatchingService(0, matching);
            if driver == 0 {
                return None;
            }
            let bundle =
                IORegistryEntryCreateCFProperty(driver, key.as_concrete_TypeRef(), ptr::null(), 0);
            let status = IORegistryEntryGetRegistryEntryID(driver, &mut id);
            IOObjectRelease(driver);
            let bundle = (!bundle.is_null()).then(|| CFType::wrap_under_create_rule(bundle));
            (status, bundle)
        };
        let bundle = bundle?.downcast::<CFString>()?;
        (status == 0 && bundle.to_string() == DRIVER_BUNDLE).then_some(id)
    })
}

pub fn supported() -> bool {
    driver_id().is_some()
}

fn energy_channel(driver: u64) -> CFMutableDictionary<CFString, CFType> {
    let simple_energy = 1_i64 | (2 << 16) | (1 << 32);
    let nanojoules = (3_i64 << 56) | (118 << 32);
    let legend = CFArray::from_CFTypes(&[
        CFNumber::from(ANE_COUNTER_ID).as_CFType(),
        CFNumber::from(simple_energy).as_CFType(),
        CFString::new("ANE").as_CFType(),
    ]);
    let info = Dictionary::from_CFType_pairs(&[(
        CFString::new("IOReportChannelUnit"),
        CFNumber::from(nanojoules).as_CFType(),
    )]);
    CFMutableDictionary::from_CFType_pairs(&[
        (
            CFString::new("DriverID"),
            CFNumber::from(driver as i64).as_CFType(),
        ),
        (
            CFString::new("DriverName"),
            CFString::new("AppleCLPC").as_CFType(),
        ),
        (
            CFString::new("IOReportGroupName"),
            CFString::new("CLPC").as_CFType(),
        ),
        (
            CFString::new("IOReportSubGroupName"),
            CFString::new("Energy Counters").as_CFType(),
        ),
        (CFString::new("IOReportChannelInfo"), info.as_CFType()),
        (CFString::new("LegendChannel"), legend.as_CFType()),
    ])
}

struct Subscription {
    api: &'static Api,
    handle: CFType,
    channels: Dictionary,
}

impl Subscription {
    fn new(driver: u64) -> Option<Self> {
        static API: OnceLock<Option<Api>> = OnceLock::new();
        let api = API.get_or_init(Api::load).as_ref()?;
        let all = unsafe { (api.copy_channels)(0, 0) };
        // These APIs return owned Core Foundation objects when successful.
        let all = (!all.is_null()).then(|| unsafe { Dictionary::wrap_under_create_rule(all) })?;
        let mut request = CFMutableDictionary::from(&all);
        let channel = energy_channel(driver).as_CFType();
        let channels = CFArray::from_CFTypes(&[channel]);
        request.set(CFString::new("IOReportChannels"), channels.as_CFType());

        let mut normalized = ptr::null();
        let handle = unsafe {
            (api.subscribe)(
                ptr::null(),
                request.as_concrete_TypeRef(),
                &mut normalized,
                0,
                ptr::null(),
            )
        };
        let subscription =
            (!handle.is_null()).then(|| unsafe { CFType::wrap_under_create_rule(handle) });
        let channels = (!normalized.is_null())
            .then(|| unsafe { Dictionary::wrap_under_create_rule(normalized) });
        Some(Self {
            api,
            handle: subscription?,
            channels: channels?,
        })
    }

    fn read(&self) -> Option<u64> {
        let raw = unsafe {
            (self.api.sample)(
                self.handle.as_CFTypeRef(),
                self.channels.as_concrete_TypeRef(),
                ptr::null(),
            )
        };
        let sample =
            (!raw.is_null()).then(|| unsafe { Dictionary::wrap_under_create_rule(raw) })?;
        let list = sample
            .find(&CFString::new("IOReportChannels"))?
            .downcast::<CFArray>()?;
        if list.len() != 1 {
            return None;
        }
        let item = *list.get(0)?;
        let channel = unsafe { CFType::wrap_under_get_rule(item) }.downcast::<CFDictionary>()?;
        let channel = channel.as_concrete_TypeRef();
        if unsafe { (self.api.channel_id)(channel) } != ANE_COUNTER_ID as u64 {
            return None;
        }
        u64::try_from(unsafe { (self.api.read_integer)(channel, 0) }).ok()
    }
}

thread_local! {
    // Keep Core Foundation subscription objects on their creating thread.
    // Each subscription reads the same cumulative driver energy counter.
    static SUBSCRIPTION: RefCell<Option<Subscription>> = const { RefCell::new(None) };
}

pub fn read_energy() -> Option<u64> {
    let driver = driver_id()?;
    SUBSCRIPTION.with_borrow_mut(|subscription| {
        if subscription.is_none() {
            *subscription = Subscription::new(driver);
        }
        let energy = subscription.as_ref()?.read();
        if energy.is_none() {
            *subscription = None;
        }
        energy
    })
}

#[cfg(test)]
mod tests {
    use super::read_energy;

    #[test]
    #[ignore = "requires an M6 on macOS build 26B131"]
    fn native_counter_is_readable_from_multiple_threads() {
        let first = read_energy().expect("read ANE energy on the supported configuration");
        let second = std::thread::spawn(read_energy)
            .join()
            .expect("join ANE sampling thread")
            .expect("read ANE energy from a separate subscription");
        assert!(second >= first);
    }
}
