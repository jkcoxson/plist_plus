// jkcoxson

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg : tt) *) => {
        // If the debug environment variable is set, print the message
        if std::env::var("PLIST_DEBUG").is_ok() {
            println!("{}:{} -- {}", file!().split('/').last().unwrap(), line!(), format!($($arg)*))
        }
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($arg : tt) *) => {
        ()
    };
}
