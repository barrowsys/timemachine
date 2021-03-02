/*
 * --------------------
 * THIS FILE IS LICENSED UNDER MIT
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */

use std::cmp;
use std::fmt;

/// A time of day as an (hour, minute, second) offset from midnight
///
/// # format!ing
///
/// Time objects implement various features found in std::fmt
/// ```
/// use timemachine::Time;
///
/// let fifteenthirty = Time::new_hm(15, 30); // 15:30:00
/// // Basic display
/// println!("{}", fifteenthirty); // Outputs "15:30"
/// // Alternate display
/// println!("{:#}", fifteenthirty); // Outputs "03:30 PM"
/// // Force seconds precision
/// println!("{:.3}", fifteenthirty); // Outputs "15:30:00"
/// // Force minutes precision (default for Times with 0 seconds component)
/// println!("{:.2}", fifteenthirty); // Outputs "15:30"
/// // Force hour precision
/// println!("{:.1}", fifteenthirty); // Outputs "15"
///
/// let sevensecondspastone = Time::new_hms(1, 0, 7); // 01:00:07
/// // Basic display
/// println!("{}", sevensecondspastone); // Outputs "01:00:07"
/// // Alternate display
/// println!("{:#}", sevensecondspastone); // Outputs "01:00:07 AM"
/// // Force seconds precision (default for Times with nonzero seconds component)
/// println!("{:.3}", sevensecondspastone); // Outputs "01:00:07"
/// // Force minutes precision
/// println!("{:.2}", sevensecondspastone); // Outputs "01:00"
/// // Force hour precision
/// println!("{:.1}", sevensecondspastone); // Outputs "01"
/// ```
///
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Time(pub u8, pub u8, pub u8);
impl Time {
    /// Returns a Time representing midnight
    /// ```
    /// use timemachine::Time;
    ///
    /// let midnight = Time::midnight();
    /// assert_eq!(midnight, Time(0, 0, 0));
    /// ```
    pub fn midnight() -> Self {
        Self(0, 0, 0)
    }
    /// Returns a time at the given hour
    /// ```
    /// use timemachine::Time;
    ///
    /// let noon = Time::new_h(12);
    /// assert_eq!(noon, Time(12, 0, 0));
    /// ```
    pub fn new_h(hour: u8) -> Self {
        Self(hour, 0, 0)
    }
    /// Returns a time at the given hour and minute
    /// ```
    /// use timemachine::Time;
    ///
    /// let threethirty = Time::new_hm(3, 30);
    /// assert_eq!(threethirty, Time(3, 30, 0));
    /// ```
    pub fn new_hm(hour: u8, minute: u8) -> Self {
        Self(hour, minute, 0)
    }
    /// Returns a time at the given hour, minute, and second
    /// ```
    /// use timemachine::Time;
    ///
    /// let twofiftyandthree = Time::new_hms(2, 50, 3);
    /// assert_eq!(twofiftyandthree, Time(2, 50, 3));
    /// ```
    pub fn new_hms(hour: u8, minute: u8, second: u8) -> Self {
        Self(hour, minute, second)
    }
    /// Returns a u16 representing minutes past midnight
    ///
    /// Inverse of [from_minutes](#method.from_minutes)
    /// ```
    /// use timemachine::Time;
    ///
    /// let onethirty = Time::new_hm(1, 30);
    /// assert_eq!(onethirty.as_minutes(), 90);
    /// ```
    /// Note that seconds are truncated!
    /// ```
    /// use timemachine::Time;
    ///
    /// let almost0001 = Time::new_hms(0, 0, 59);
    /// assert_eq!(almost0001.as_minutes(), 0);
    /// ```
    pub fn as_minutes(&self) -> u16 {
        (self.0 as u16) * 60 + (self.1 as u16)
    }
    /// Returns a time object for a given minute offset past midnight
    ///
    /// Inverse of [as_minutes](#method.as_minutes)
    pub fn from_minutes(minutes: u16) -> Self {
        Self::new_hm(((minutes / 60) % 24) as u8, (minutes % 60) as u8)
    }
    /// Returns a u32 representing seconds past midnight
    ///
    /// Inverse of [from_seconds](#method.from_seconds)
    /// ```
    /// use timemachine::Time;
    ///
    /// let twominutesfourty = Time::new_hms(0, 2, 40);
    /// assert_eq!(twominutesfourty.as_seconds(), 160);
    /// ```
    pub fn as_seconds(&self) -> u32 {
        ((self.0 as u32) * 60 + (self.1 as u32)) * 60 + (self.2 as u32)
    }
    /// Returns a time object for a given second offset past midnight
    ///
    /// Inverse of [as_seconds](#method.as_seconds)
    pub fn from_seconds(seconds: u32) -> Self {
        let minutes = seconds / 60;
        Self::new_hms(
            ((minutes / 60) % 24) as u8,
            (minutes % 60) as u8,
            (seconds % 60) as u8,
        )
    }
}
impl cmp::PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl cmp::Ord for Time {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_seconds().cmp(&other.as_seconds())
    }
}
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut print_mins = true;
        let mut print_secs = self.2 != 0;
        if let Some(precision) = f.precision() {
            if precision == 1 {
                print_mins = false;
                print_secs = false;
            } else if precision == 2 {
                print_secs = false;
            } else if precision >= 3 {
                print_secs = true;
            }
        }
        if f.alternate() {
            if self.0 % 12 == 0 {
                write!(f, "12")?;
            } else {
                write!(f, "{:02}", self.0 % 12)?;
            }
        } else {
            write!(f, "{:02}", self.0 % 24)?;
        }
        if print_mins {
            write!(f, ":{:02}", self.1 % 60)?;
            if print_secs {
                write!(f, ":{:02}", self.2 % 60)?;
            }
        }
        if f.alternate() {
            if self.0 < 12 {
                write!(f, " AM")?;
            } else {
                write!(f, " PM")?;
            }
        }
        Ok(())
    }
}

pub struct Clock {
    hour: u8,
    minute: u8,
    second: u8,
    hour_inc: u8,
    minute_inc: u8,
    second_inc: u8,
}
impl Clock {
    pub fn seconds() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            hour_inc: 0,
            minute_inc: 0,
            second_inc: 1,
        }
    }
    pub fn minutes() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            hour_inc: 0,
            minute_inc: 1,
            second_inc: 0,
        }
    }
    pub fn hours() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            hour_inc: 1,
            minute_inc: 0,
            second_inc: 0,
        }
    }
    pub fn every(hours: u8, minutes: u8, seconds: u8) -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            hour_inc: hours,
            minute_inc: minutes,
            second_inc: seconds,
        }
    }
    pub fn start_h(self, hour: u8) -> Self {
        Self {
            hour: hour % 24,
            minute: 0,
            second: 0,
            ..self
        }
    }
    pub fn start_hm(self, hour: u8, minute: u8) -> Self {
        Self {
            hour: hour % 24,
            minute: minute % 60,
            second: 0,
            ..self
        }
    }
    pub fn start_hms(self, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour: hour % 24,
            minute: minute % 60,
            second: second % 60,
            ..self
        }
    }
}
impl Iterator for Clock {
    type Item = Time;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hour >= 24 {
            None
        } else {
            let r = Time::new_hms(self.hour, self.minute, self.second);
            self.hour += self.hour_inc;
            self.minute += self.minute_inc;
            self.second += self.second_inc;
            if self.second >= 60 {
                self.minute += self.second / 60;
                self.second %= 60;
            }
            if self.minute >= 60 {
                self.hour += self.minute / 60;
                self.minute %= 60;
            }
            // if self.hour >= 24 {
            //     self.day += self.hour / 24;
            //     self.hour = self.hour % 24;
            // }
            Some(r)
        }
    }
}
