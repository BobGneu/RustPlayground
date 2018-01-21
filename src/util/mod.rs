pub struct OS {}

impl OS {
    // TODO: Replace this method with a version that returns the actual OS info. 
    // Should be a function that returns something like Window 10 x64 - v155551
    pub fn name() -> &'static str {
    if cfg!(windows) {
        return &"Windows";
    } else if cfg!(unix) {
        return &"Unix";
    } else if cfg!(macos) {
        return &"MacOS";
    }

    return &"Unknown";
    }

    // TODO: Find OS Version
    pub fn version() -> &'static str {
        return &"0.0.0";
    }
}
