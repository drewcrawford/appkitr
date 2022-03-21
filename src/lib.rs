/**
# Drew's fast Rust AppKit bindings

Provides select Rust bindings for Apple [AppKit](https://developer.apple.com/documentation/appkit) framework.  This may be compared to,
* [appkit crate](https://crates.io/crates/appkit)
* [cacao](https://crates.io/crates/cacao)
* [objrs_framework_appkit](https://crates.io/crates/objrs_frameworks_app_kit)
* [fruitbasket](https://crates.io/crates/fruitbasket)

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features of this library:

* As part of the [objr](https://github.com/drewcrawford/objr) ecosystem, classes such as NSView **can be subclassed directly in Rust**.
* Fast.  This crate is *significantly* faster than other approaches.  If you are interested in writing performance-critical realtime applications, this is the solution for you.
    * The full set of optimization is far too many to list, but the big idea is to either do what native ObjC/Swift applications do, or do something faster.
    * Compile-time selectors.  Most Rust crates do a runtime lookup for ObjC methods, which involves acquiring a lock and other yucky stuff, either on the first call or every call.  Instead, we do what real ObjC compilers do, which is way faster.  For more details, see [objr](https://github.com/drewcrawford/objr)
    * Smart pointers that provide global ARC inference.  Like ARC, you don't need to write manual retain/release calls.  Unlike ARC, the compiler
      usually doesn't need to write them either, meaning lower runtime memory management cost than even native code.  For more details, see [objr](https://github.com/drewcrawford/objr).
    * Runtime autorelease eliding, which keeps your objects out of autoreleasepools in common cases.  For more details, see [objr](https://github.com/drewcrawford/objr).
    * Pointer packing for optional types so they fit in a `u64`.  For more details, see [objr](https://github.com/drewcrawford/objr)
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Low-level.  These bindings assume familiarity with bound APIs and are not documented separately.  For details on how they work, see the native documentation.
* Free for noncommercial or "small commercial" use.

# Implementation status
The following APIs are at least partially implemented.  These implementations are incomplete but contain common functions or "the ones I use".

The [objr](https://github.com/drewcrawford/objr) macro system makes it very ergonomic to add new bindings for specific missing features
or new APIs.

* NSView - several common APIs.  In particular, subclassing of this type is fairly well tested in my other projects.
* NSWindow and related types - several common APIs.
* NSApplication - a few APIs
* NSScreen - a few APIs
* NSGraphics, NSBackingStoreType, NSDeviceDescriptionKey - few APIs
*/

mod nswindow;
mod nsgraphics;
mod nsview;
mod nsapplication;
mod nsscreen;
mod nsevent;

pub use nsgraphics::{NSBackingStoreType,NSDeviceDescriptionKey};
pub use nsview::NSView;
pub use nswindow::{NSWindow,NSWindowStyleMask,NSWindowOcclusionState};
pub use nsapplication::NSApplication;
pub use nsscreen::NSScreen;
pub use nsevent::NSEvent;


#[link(name="AppKit",kind="framework")]
extern {}


