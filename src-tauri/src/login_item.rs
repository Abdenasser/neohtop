use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Unavailable,
    Disabled,
    Enabled,
    RequiresApproval,
}

#[tauri::command]
pub fn login_item_status() -> Status {
    #[cfg(target_os = "macos")]
    return macos::status();
    #[cfg(not(target_os = "macos"))]
    Status::Unavailable
}

#[tauri::command]
pub fn set_login_item_enabled(enabled: bool) -> Result<Status, String> {
    #[cfg(target_os = "macos")]
    return macos::set_enabled(enabled);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = enabled;
        Ok(Status::Unavailable)
    }
}

#[cfg(target_os = "macos")]
pub use macos::starts_in_background;

#[cfg(target_os = "macos")]
mod macos {
    use super::Status;
    use objc2::{
        class, msg_send,
        rc::Retained,
        runtime::{AnyClass, AnyObject, Bool},
    };

    #[link(name = "ServiceManagement", kind = "framework")]
    extern "C" {}

    fn main_app() -> Option<Retained<AnyObject>> {
        // ServiceManagement is present on older macOS releases, but SMAppService starts at 13.
        let class = AnyClass::get(c"SMAppService")?;
        // SAFETY: mainAppService returns the framework's Objective-C service object.
        unsafe { msg_send![class, mainAppService] }
    }

    fn service_status(service: &AnyObject) -> Status {
        // SAFETY: SMAppServiceStatus is an NSInteger.
        let raw: isize = unsafe { msg_send![service, status] };
        match raw {
            1 => Status::Enabled,
            2 => Status::RequiresApproval,
            _ => Status::Disabled,
        }
    }

    pub fn status() -> Status {
        main_app()
            .as_deref()
            .map_or(Status::Unavailable, service_status)
    }

    pub fn set_enabled(enabled: bool) -> Result<Status, String> {
        let Some(service) = main_app() else {
            return Ok(Status::Unavailable);
        };
        let current = service_status(&service);
        if current == Status::Enabled && enabled || current == Status::Disabled && !enabled {
            return Ok(current);
        }
        // SAFETY: Both methods are synchronous and take an optional NSError**.
        let result: Bool = unsafe {
            let no_error = std::ptr::null_mut::<*mut AnyObject>();
            if enabled {
                msg_send![&*service, registerAndReturnError: no_error]
            } else {
                msg_send![&*service, unregisterAndReturnError: no_error]
            }
        };
        let current = service_status(&service);
        if result.as_bool()
            || (enabled && matches!(current, Status::Enabled | Status::RequiresApproval))
            || (!enabled && current == Status::Disabled)
        {
            Ok(current)
        } else {
            Err("macOS could not update the login item".into())
        }
    }

    pub fn starts_in_background() -> bool {
        const OPEN_APPLICATION: u32 = u32::from_be_bytes(*b"oapp");
        const PROPERTY_DATA: u32 = u32::from_be_bytes(*b"prdt");
        const LAUNCHED_AS_LOGIN_ITEM: u32 = u32::from_be_bytes(*b"lgit");

        // SAFETY: Tauri calls setup inside applicationDidFinishLaunching on the main
        // thread. The descriptors use AEEventID, AEKeyword and DescType (all UInt32).
        unsafe {
            let manager: Retained<AnyObject> =
                msg_send![class!(NSAppleEventManager), sharedAppleEventManager];
            let event: Option<Retained<AnyObject>> = msg_send![&*manager, currentAppleEvent];
            let Some(event) = event else {
                // A login launch can arrive without an Apple event.
                return status() == Status::Enabled;
            };
            let event_id: u32 = msg_send![&*event, eventID];
            if event_id != OPEN_APPLICATION {
                return false;
            }
            let mark: Option<Retained<AnyObject>> =
                msg_send![&*event, paramDescriptorForKeyword: PROPERTY_DATA];
            mark.is_some_and(|mark| {
                let code: u32 = msg_send![&*mark, enumCodeValue];
                code == LAUNCHED_AS_LOGIN_ITEM
            })
        }
    }
}
