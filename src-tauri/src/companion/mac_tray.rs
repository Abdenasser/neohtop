use std::cell::RefCell;
use tauri::{AppHandle, LogicalPosition, LogicalSize, Rect};

use objc2::{
    define_class, rc::Retained, sel, AnyThread, DefinedClass, MainThreadMarker, MainThreadOnly,
};
use objc2_app_kit::{
    NSAccessibility, NSApplication, NSEventMask, NSEventModifierFlags, NSEventType, NSImage,
    NSMenu, NSScreen, NSSquareStatusItemLength, NSStatusBar, NSStatusBarButton, NSStatusItem,
};
use objc2_foundation::{NSData, NSObject, NSPoint, NSSize, NSString};

struct StatusItem {
    item: Retained<NSStatusItem>,
    _actions: Retained<Actions>,
}

thread_local! {
    static TRAY: RefCell<Option<StatusItem>> = const { RefCell::new(None) };
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = AppHandle]
    struct Actions;

    impl Actions {
        #[unsafe(method(activate:))]
        fn activate(&self, button: &NSStatusBarButton) {
            let event = NSApplication::sharedApplication(self.mtm()).currentEvent();
            if event.is_some_and(|event| {
                event.r#type() == NSEventType::RightMouseUp ||
                    (event.r#type() == NSEventType::LeftMouseUp &&
                        event.modifierFlags().contains(NSEventModifierFlags::Control))
            }) {
                self.show_menu(button);
            } else {
                super::toggle_overview(self.ivars(), button_rect(button));
            }
        }

        #[unsafe(method(showOverview:))]
        fn show_overview(&self, _: &NSObject) {
            let _ = super::show(self.ivars(), None);
        }

        #[unsafe(method(openMain:))]
        fn open_main(&self, _: &NSObject) {
            let _ = super::open_main(self.ivars());
        }

        #[unsafe(method(quit:))]
        fn quit(&self, _: &NSObject) {
            self.ivars().exit(0);
        }
    }
);

impl Actions {
    fn show_menu(&self, button: &NSStatusBarButton) {
        let menu = NSMenu::new(self.mtm());
        let selectors = [sel!(showOverview:), sel!(openMain:), sel!(quit:)];
        for (label, action) in super::TRAY_LABELS.into_iter().zip(selectors) {
            // SAFETY: Each selector is implemented on Actions and takes one sender.
            unsafe {
                let item = menu.addItemWithTitle_action_keyEquivalent(
                    &NSString::from_str(label),
                    Some(action),
                    &NSString::from_str(""),
                );
                item.setTarget(Some(self));
            }
        }
        // An attached NSStatusItem menu consumes the default AXPress action.
        // Present it only for an explicit secondary click.
        menu.popUpMenuPositioningItem_atLocation_inView(None, NSPoint::new(0.0, 0.0), Some(button));
    }
}

fn button_rect(button: &NSStatusBarButton) -> Option<Rect> {
    let window = button.window()?;
    let frame = window.frame();
    let scale = window.backingScaleFactor();
    let screen = NSScreen::screens(button.mtm()).firstObject()?;
    let top = screen.frame().size.height - frame.origin.y - frame.size.height;
    Some(Rect {
        position: LogicalPosition::new(frame.origin.x, top)
            .to_physical::<i32>(scale)
            .into(),
        size: LogicalSize::new(frame.size.width, frame.size.height)
            .to_physical::<u32>(scale)
            .into(),
    })
}

pub(super) fn rect() -> Option<Rect> {
    let mtm = MainThreadMarker::new()?;
    TRAY.with_borrow(|slot| {
        let button = slot.as_ref()?.item.button(mtm)?;
        button_rect(&button)
    })
}

fn ghost_image() -> Result<Retained<NSImage>, String> {
    let ghost = super::ghost();
    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, ghost.width(), ghost.height());
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .and_then(|mut writer| writer.write_image_data(ghost.rgba()))
        .map_err(|error| error.to_string())?;
    let image = NSImage::initWithData(NSImage::alloc(), &NSData::with_bytes(&bytes))
        .ok_or("Could not create the menu bar image")?;
    image.setSize(NSSize::new(18.0, 18.0));
    image.setTemplate(true);
    Ok(image)
}

pub(super) fn configure(app: &AppHandle, enabled: bool, tooltip: &str) -> Result<(), String> {
    let mtm = MainThreadMarker::new().ok_or("Menu bar unavailable off the main thread")?;
    TRAY.with_borrow_mut(|slot| {
        if !enabled {
            if let Some(tray) = slot.take() {
                NSStatusBar::systemStatusBar().removeStatusItem(&tray.item);
            }
            return Ok(());
        }
        if slot.is_none() {
            let image = ghost_image()?;
            let bar = NSStatusBar::systemStatusBar();
            let item = bar.statusItemWithLength(NSSquareStatusItemLength);
            let Some(button) = item.button(mtm) else {
                bar.removeStatusItem(&item);
                return Err("Menu bar button unavailable".into());
            };
            let actions: Retained<Actions> = unsafe {
                // SAFETY: NSObject initializes the subclass allocated above.
                objc2::msg_send![super(mtm.alloc::<Actions>().set_ivars(app.clone())), init]
            };
            button.setImage(Some(&image));
            // SAFETY: The action takes the button as its sender and the target is retained.
            unsafe {
                button.setTarget(Some(&actions));
                button.setAction(Some(sel!(activate:)));
            }
            button.sendActionOn(NSEventMask::LeftMouseUp | NSEventMask::RightMouseUp);
            *slot = Some(StatusItem {
                item,
                _actions: actions,
            });
        }
        if let Some(button) = slot.as_ref().and_then(|tray| tray.item.button(mtm)) {
            let label = NSString::from_str(tooltip);
            button.setToolTip(Some(&label));
            button.setAccessibilityLabel(Some(&label));
        }
        Ok(())
    })
}
