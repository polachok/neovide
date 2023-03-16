use crate::{settings::SETTINGS, window::WindowSettings};

use cocoa::appkit::{NSColor, NSWindow};
use cocoa::base::{id, nil};
use csscolorparser::Color;
use objc::{rc::autoreleasepool, runtime::YES};
use winit::{platform::macos::WindowExtMacOS, window::Window};

pub fn set_appearance(window: &Window) {
    use objc::{sel, sel_impl};
    unsafe fn NSAppearance(name: cocoa::base::id) -> id {
        objc::msg_send!(objc::class!(NSAppearance), appearanceNamed: name)
    }
    unsafe {
        let app_id = cocoa::appkit::NSAppearanceNameVibrantDark;
        let app = NSAppearance(app_id);
        let ns_window: id = window.ns_window() as id;
        let _: () = objc::msg_send!(ns_window, setAppearance: app);
    }
}

pub fn draw_background(window: &Window) {
    if let Ok(color) = &SETTINGS
        .get::<WindowSettings>()
        .background_color
        .parse::<Color>()
    {
        autoreleasepool(|| unsafe {
            let [red, green, blue, alpha] = color.to_array();
            let ns_window: id = window.ns_window() as id;
            let ns_background =
                NSColor::colorWithSRGBRed_green_blue_alpha_(nil, red, green, blue, alpha);
            ns_window.setBackgroundColor_(ns_background);
            ns_window.setTitlebarAppearsTransparent_(YES);
        });
    };
    set_appearance(window);
}
