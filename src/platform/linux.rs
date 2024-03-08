use crate::ClipboardProvider;

use raw_window_handle::HasDisplayHandle;
#[cfg(feature = "wayland")]
use raw_window_handle::RawDisplayHandle;
use std::error::Error;

#[cfg(feature = "wayland")]
pub use clipboard_wayland as wayland;
#[cfg(feature = "x11")]
pub use clipboard_x11 as x11;

pub unsafe fn connect<W: HasDisplayHandle>(
    window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn Error>> {
    #[cfg(not(any(feature = "x11", feature = "wayland")))]
    compile_error!("No backend (X11 or Wayland) selected for Linux platform.");

    let clipboard = match window.display_handle()?.as_raw() {
        #[cfg(feature = "wayland")]
        RawDisplayHandle::Wayland(handle) => {
            Box::new(wayland::Clipboard::connect(handle.display.as_ptr())) as _
        }
        #[cfg(feature = "x11")]
        _ => Box::new(x11::Clipboard::connect()?) as _,
    };

    Ok(clipboard)
}

#[cfg(feature = "wayland")]
impl ClipboardProvider for wayland::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read()
    }

    fn read_primary(&self) -> Option<Result<String, Box<dyn Error>>> {
        Some(self.read_primary())
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.write(contents)
    }

    fn write_primary(&mut self, contents: String) -> Option<Result<(), Box<dyn Error>>> {
        Some(self.write_primary(contents))
    }
}

#[cfg(feature = "x11")]
impl ClipboardProvider for x11::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read().map_err(Box::from)
    }

    fn read_primary(&self) -> Option<Result<String, Box<dyn Error>>> {
        Some(self.read_primary().map_err(Box::from))
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.write(contents).map_err(Box::from)
    }

    fn write_primary(&mut self, contents: String) -> Option<Result<(), Box<dyn Error>>> {
        Some(self.write_primary(contents).map_err(Box::from))
    }
}
