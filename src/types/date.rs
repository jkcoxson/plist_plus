// jkcoxson

use crate::{error::PlistError, unsafe_bindings, Plist, PlistType};
use log::trace;
use std::time::Duration;

const MAC_EPOCH: u64 = 978307200; // 01/01/2001

impl Plist {
    /// Returns a new plist with a date.
    /// The duration must represent a time passed since the Unix Epoch.
    ///
    /// Note: the original library expects you to pass the number of seconds
    /// since 01/01/2001 (Mac Epoch). You **don't** need to do this here.
    ///
    /// # Example
    /// ```rust
    /// use std::time::Duration;
    /// let now = SystemTime::now()
    ///    .duration_since(SystemTime::UNIX_EPOCH)
    ///    .unwrap();
    /// Plist::new_date(now);
    /// ```
    pub fn new_date(date: Duration) -> Plist {
        trace!("Generating new date plist");
        // The number of seconds since 01/01/2001
        let duration = date - Duration::from_secs(MAC_EPOCH);
        let secs = duration.as_secs();
        let usecs = duration.as_micros() - (secs * 1000000) as u128;
        unsafe { unsafe_bindings::plist_new_date(secs as i32, usecs as i32) }.into()
    }

    /// Returns a duration (a Unix Timestamp) of the date
    pub fn get_date_val(&self) -> Result<Duration, PlistError> {
        if self.plist_type != PlistType::Date {
            return Err(PlistError::InvalidArg);
        }
        let mut sec = unsafe { std::mem::zeroed() };
        let mut usec = unsafe { std::mem::zeroed() };
        trace!("Getting date value");
        unsafe { unsafe_bindings::plist_get_date_val(self.plist_t, &mut sec, &mut usec) };
        let date = usec as u64 + (sec as u64) * 1000000;
        Ok(Duration::from_micros(date) + Duration::from_secs(MAC_EPOCH))
    }

    /// Sets the date with a Unix Timestamp
    pub fn set_date_val(&self, date: Duration) {
        let duration = date - Duration::from_secs(MAC_EPOCH);
        let secs = duration.as_secs();
        let usecs = duration.as_micros() - (secs * 1000000) as u128;
        trace!("Setting date value");
        unsafe { unsafe_bindings::plist_set_date_val(self.plist_t, secs as i32, usecs as i32) };
    }
}

impl From<Duration> for Plist {
    fn from(value: Duration) -> Self {
        Plist::new_date(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn check_unix_mac_date() {
        let timestamp = 1546635600123456; // Jan 04 2019 21:00:00.123456

        let unix_date = Duration::from_micros(timestamp);
        let unix_plist = Plist::new_date(unix_date);

        let secs = 1546635600 - MAC_EPOCH;
        let usecs = 123456;

        let mac_plist: Plist = unsafe {
            unsafe_bindings::plist_new_date(secs as i32, usecs)
        }.into();

        assert_eq!(
            unix_plist.get_date_val().unwrap(),
            mac_plist.get_date_val().unwrap()
        );
    }

    #[test]
    fn set_random_date() {
        let timestamp = 1546635600123456; // Jan 04 2019 21:00:00.123456

        let date = Duration::from_micros(timestamp);
        let plist = Plist::new_date(
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
        ); // create a new date with a current time
        plist.set_date_val(date); // set a new time

        assert_eq!(
            date,
            plist.get_date_val().unwrap()
        );
    }
}
