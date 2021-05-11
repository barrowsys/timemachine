/*
 * --------------------
 * THIS FILE IS LICENSED UNDER THE FOLLOWING TERMS
 *
 * this code may not be used for any purpose. be gay, do crime
 *
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
/// ### Basic Usage
/// Aside from the [`noon`](#method.noon) and [`midnight`](#method.midnight) functions,
/// the primary way to make a Time object is with the new_h(m(s)?)? functions.
/// ```
/// use timemachine::Time;
///
/// let eleven_am = Time::new_h(11);
/// let one_thirty_pm = Time::new_hm(13, 30);
/// let five_seconds_to_noon = Time::new_hms(11, 59, 55);
/// // Note that the above functions are shorthands for the below tuple constructions
/// assert_eq!(eleven_am, Time(11, 0, 0));
/// assert_eq!(one_thirty_pm, Time(13, 30, 0));
/// assert_eq!(five_seconds_to_noon, Time(11, 59, 55));
///
/// assert_eq!(format!("{:+}", eleven_am), String::from("11:00 AM"));
/// assert_eq!(format!("{:+}", one_thirty_pm), String::from("01:30 PM"));
/// assert_eq!(format!("{:+}", five_seconds_to_noon), String::from("11:59:55 AM"));
/// ```
///
/// ### Conversion to and from base units
/// Time objects are nice, but it's helpful to be able to convert them into simpler units,
/// such as u16 minutes ([as_minutes](#method.as_minutes))
/// and u32 seconds ([as_seconds](#method.as_seconds)) past midnight.
/// You can also convert them back with [from_minutes](#method.from_minutes) and
/// [from_seconds](#method.from_seconds).
///
/// For more information see [the relevant impl](#base-unit-conversion).
///
/// ### format!ing
/// Time objects implement various features found in std::fmt and can be printed prettily.
///
/// For more information see [impl Display](#impl-Display).
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
    /// Returns a Time representing noon
    /// ```
    /// use timemachine::Time;
    ///
    /// let noon = Time::noon();
    /// assert_eq!(noon, Time(12, 0, 0));
    /// ```
    pub fn noon() -> Self {
        Self(12, 0, 0)
    }
    /// Returns a time at the given hour.
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
    /// Finds the number of seconds from &self until &other.  
    /// This is *baaaasically* subtraction mod 24\*60\*60,
    /// but the function name makes it clear which operand is which.
    /// ```
    /// use timemachine::Time;
    ///
    /// let midnight = Time::midnight();
    /// let two_past = Time::from_seconds(2);
    /// let two_til = Time::from_seconds(-2);
    /// // From 00:00:00 until 00:00:02 = 2 seconds
    /// assert_eq!(midnight.secs_until(&two_past), 2);
    /// // From 23:59:58 until 00:00:00 = 2 seconds
    /// assert_eq!(two_til.secs_until(&midnight), 2);
    /// // From 23:59:58 until 00:00:02 = 4 seconds
    /// assert_eq!(two_til.secs_until(&two_past), 4);
    /// // From 00:00:02 until 23:59:58 = 4 less than 86400 seconds
    /// assert_eq!(two_past.secs_until(&two_til), 86400 - 4);
    /// ```
    pub fn secs_until(&self, future: &Self) -> u32 {
        let selfs = self.as_seconds();
        let mut others = future.as_seconds();
        if others < selfs {
            others += 86400;
        }
        others
            .checked_sub(selfs)
            .expect("Something went wrong finding the difference of times, probably a bug?")
    }
}
/// ### Base unit conversion
/// Time objects are nice, but it's also helpful to convert them into base units
/// relative to midnight: minutes past midnight with [`as_minutes`](#method.as_minutes)
/// and seconds past midnight with [`as_seconds`](#method.as_seconds).
/// ```
/// use timemachine::Time;
///
/// let one_am = Time::new_h(1);
/// let one_oh_one = Time::new_hm(1, 1);
/// let one_oh_one_oh_one = Time::new_hms(1, 1, 1);
///
/// assert_eq!(one_am.as_minutes(), 60);
/// assert_eq!(one_am.as_seconds(), 3600);
///
/// assert_eq!(one_oh_one.as_minutes(), 60 + 1);
/// assert_eq!(one_oh_one.as_seconds(), 3600 + 60);
///
/// assert_eq!(one_oh_one_oh_one.as_minutes(), 60 + 1); // as_minutes() truncates seconds
/// assert_eq!(one_oh_one_oh_one.as_seconds(), 3600 + 60 + 1);
/// ```
///
/// You can also convert back with [`from_minutes`](#method.from_minutes) and
/// [`from_seconds`](#method.from_seconds).
/// These functions can take negative values, which represent an offset *before* midnight.
/// ```
/// use timemachine::Time;
///
/// // 01:00 AM example
/// let one_am = Time::new_hms(1, 0, 0);
/// let from_mins = Time::from_minutes(60);
/// let from_secs = Time::from_seconds(3600);
/// assert_eq!(one_am, from_mins);
/// assert_eq!(one_am, from_secs);
/// assert_eq!(from_mins, from_secs);
///
/// // 10:30 PM example
/// let ten_thirty_pm = Time::new_hms(22, 30, 0);
/// let from_mins = Time::from_minutes(1350);
/// let from_neg_mins = Time::from_minutes(-90);
/// assert_eq!(ten_thirty_pm, from_mins);
/// assert_eq!(ten_thirty_pm, from_neg_mins);
/// assert_eq!(from_mins, from_neg_mins);
///
/// // 11:59:57 PM example
/// let three_seconds_to_midnight = Time::new_hms(23, 59, 57);
/// let from_secs = Time::from_seconds(-3);
/// assert_eq!(three_seconds_to_midnight, from_secs);
/// ```
impl Time {
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
    /// Returns a time object for a given minute offset past midnight.
    /// Negative values are offset before midnight.
    ///
    /// Inverse of [as_minutes](#method.as_minutes)
    /// ```
    /// use timemachine::Time;
    /// let one_past = Time::from_minutes(1);
    /// let one_before = Time::from_minutes(-1);
    /// assert_eq!(one_past, Time(0, 1, 0));
    /// assert_eq!(one_before, Time(23, 59, 0));
    /// ```
    ///
    /// Inverse of [as_minutes](#method.as_minutes)
    pub fn from_minutes(minutes: i16) -> Self {
        Self::new_hm(
            minutes.div_euclid(60).rem_euclid(24) as u8,
            minutes.rem_euclid(60) as u8,
        )
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
    /// Returns a time object for a given second offset past midnight.
    /// Negative values are offset before midnight.
    ///
    /// Inverse of [as_seconds](#method.as_seconds)
    /// ```
    /// use timemachine::Time;
    ///
    /// let one_past = Time::from_seconds(1);
    /// let one_before = Time::from_seconds(-1);
    /// assert_eq!(one_past, Time(0, 0, 1));
    /// assert_eq!(one_before, Time(23, 59, 59));
    /// ```
    ///
    /// Inverse of [as_seconds](#method.as_seconds)
    pub fn from_seconds(seconds: i32) -> Self {
        let minutes = seconds.div_euclid(60);
        Self::new_hms(
            minutes.div_euclid(60).rem_euclid(24) as u8,
            minutes.rem_euclid(60) as u8,
            seconds.rem_euclid(60) as u8,
        )
    }
}
/// Ordering is relative to midnight, it's just self.as_seconds().cmp(&other.as_seconds())
impl cmp::PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
/// Ordering is relative to midnight, it's just self.as_seconds().cmp(&other.as_seconds())
impl cmp::Ord for Time {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_seconds().cmp(&other.as_seconds())
    }
}
/// Time objects implement various features found in [`std::fmt`](std::fmt)
/// and will be printed prettily when [`Display`](std::fmt::Display)ed.
/// - Basic Display (24 Hour)
///     ```
///     # use timemachine::Time;
///     let one_ten_pm = Time::new_hm(13, 10);
///     let two_and_five = Time::new_hms(2, 0, 5);
///     assert_eq!(
///         format!("{}", one_ten_pm),
///         "13:10".to_string()
///     );
///     assert_eq!(
///         format!("{}", two_and_five),
///         "02:00:05".to_string()
///     );
///     ```
/// - [Signed Display](std::fmt#sign0) (12 Hour)
///     ```
///     # use timemachine::Time;
///     # let one_ten_pm = Time::new_hm(13, 10);
///     # let two_and_five = Time::new_hms(2, 0, 5);
///     assert_eq!(
///         format!("{:+}", one_ten_pm),
///         "01:10 PM".to_string()
///     );
///     assert_eq!(
///         format!("{:+}", two_and_five),
///         "02:00:05 AM".to_string()
///     );
///     ```
/// - [Explicit Precision](std::fmt#precision)
///     ```
///     # use timemachine::Time;
///     # let one_ten_pm = Time::new_hm(13, 10);
///     # let two_and_five = Time::new_hms(2, 0, 5);
///     assert_eq!(format!("{:.1}", one_ten_pm), "13".to_string());
///     assert_eq!(format!("{:.2}", one_ten_pm), "13:10".to_string());
///     assert_eq!(format!("{:.3}", one_ten_pm), "13:10:00".to_string());
///     assert_eq!(format!("{:.1}", two_and_five), "02".to_string());
///     assert_eq!(format!("{:.2}", two_and_five), "02:00".to_string());
///     assert_eq!(format!("{:.3}", two_and_five), "02:00:05".to_string());
///     ```
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
        if f.sign_plus() {
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
        if f.sign_plus() {
            if self.0 < 12 {
                write!(f, " AM")?;
            } else {
                write!(f, " PM")?;
            }
        }
        Ok(())
    }
}

/// [Time](crate::Time) range iterator for convenience
///
/// Clocks are iterators from a start time to an end time, stepping by a given increment.
///
/// Simple clocks can be made with the [`seconds`](#method.seconds), [`minutes`](#method.minutes),
/// and [`hours`](#method.hours) functions.
/// To set a custom step size, use [`every`](#method.every).
///
/// The start can be set as a time with [`start`](#method.start), or manually with
/// [`start_h`](#method.start_h), [`start_hm`](#method.start_hm), and
/// [`start_hms`](#method.start_hms).
///
/// The end can be set as a time with [`end`](#method.end), or manually with
/// [`end_h`](#method.end_h), [`end_hm`](#method.end_hm), and
/// [`end_hms`](#method.end_hms).
///
/// The default values for start and end are `Time(0, 0, 0)` and `Time(24, 0, 0)`,
/// so if you don't manually set either the clock will iterate from 00:00:00 until 23:59:59 (if
/// stepping by seconds).
/// ```
/// use timemachine::Time;
/// use timemachine::Clock;
///
/// let mut clock = Clock::minutes();
///
/// assert_eq!(clock.next().unwrap(), Time::new_hms(0, 0, 0));
/// assert_eq!(clock.next().unwrap(), Time::new_hms(0, 1, 0));
/// // ...
/// assert_eq!(clock.last().unwrap(), Time::new_hms(23, 59, 0));
/// ```
///
/// ```
/// use timemachine::Time;
/// use timemachine::Clock;
///
/// let mut clock = Clock::minutes()
///     .start_hms(3, 45, 5)
///     .end_hms(4, 37, 0);
///
/// assert_eq!(clock.next().unwrap(), Time::new_hms(3, 45, 5));
/// assert_eq!(clock.next().unwrap(), Time::new_hms(3, 46, 5));
/// // ...
/// assert_eq!(clock.last().unwrap(), Time::new_hms(4, 36, 5));
/// ```
pub struct Clock {
    hour: u8,
    minute: u8,
    second: u8,
    hour_inc: u8,
    minute_inc: u8,
    second_inc: u8,
    hour_end: u8,
    minute_end: u8,
    second_end: u8,
}
impl Clock {
    /// Creates a new clock with a tick rate of 1 second.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::seconds();
    ///
    /// assert_eq!(clock.next().unwrap(), Time::midnight());
    /// assert_eq!(clock.next().unwrap(), Time(0, 0, 1));
    /// assert_eq!(clock.next().unwrap(), Time(0, 0, 2));
    /// ```
    pub fn seconds() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            hour_inc: 0,
            minute_inc: 0,
            second_inc: 1,
            hour_end: 24,
            minute_end: 0,
            second_end: 0,
        }
    }
    /// Creates a new clock with a tick rate of 1 minute.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::minutes();
    ///
    /// assert_eq!(clock.next().unwrap(), Time::midnight());
    /// assert_eq!(clock.next().unwrap(), Time(0, 1, 0));
    /// assert_eq!(clock.next().unwrap(), Time(0, 2, 0));
    /// ```
    pub fn minutes() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            hour_inc: 0,
            minute_inc: 1,
            second_inc: 0,
            hour_end: 24,
            minute_end: 0,
            second_end: 0,
        }
    }
    /// Creates a new clock with a tick rate of 1 hour.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::hours();
    ///
    /// assert_eq!(clock.next().unwrap(), Time::midnight());
    /// assert_eq!(clock.next().unwrap(), Time(1, 0, 0));
    /// assert_eq!(clock.next().unwrap(), Time(2, 0, 0));
    /// ```
    pub fn hours() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            hour_inc: 1,
            minute_inc: 0,
            second_inc: 0,
            hour_end: 24,
            minute_end: 0,
            second_end: 0,
        }
    }
    /// Creates a new clock with a given tick rate.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::every(0, 1, 30);
    ///
    /// assert_eq!(clock.next().unwrap(), Time::midnight());
    /// assert_eq!(clock.next().unwrap(), Time(0, 1, 30));
    /// assert_eq!(clock.next().unwrap(), Time(0, 3, 0));
    /// ```
    pub fn every(hours: u8, minutes: u8, seconds: u8) -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            hour_inc: hours,
            minute_inc: minutes,
            second_inc: seconds,
            hour_end: 24,
            minute_end: 0,
            second_end: 0,
        }
    }
    /// Builder function to add a start offset to a clock.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::minutes()
    ///     .start(&Time(3, 0, 0));
    ///
    /// assert_eq!(clock.next().unwrap(), Time(3, 0, 0));
    /// assert_eq!(clock.next().unwrap(), Time(3, 1, 0));
    /// assert_eq!(clock.next().unwrap(), Time(3, 2, 0));
    /// ```
    pub fn start(self, time: &Time) -> Self {
        Self {
            hour: time.0,
            minute: time.1,
            second: time.2,
            ..self
        }
    }
    /// Builder function to add a start offset in hours to a clock.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::minutes().start_h(3);
    ///
    /// assert_eq!(clock.next().unwrap(), Time(3, 0, 0));
    /// assert_eq!(clock.next().unwrap(), Time(3, 1, 0));
    /// assert_eq!(clock.next().unwrap(), Time(3, 2, 0));
    /// ```
    pub fn start_h(self, hour: u8) -> Self {
        Self {
            hour: hour % 24,
            minute: 0,
            second: 0,
            ..self
        }
    }
    /// Builder function to add a start offset in hours and minutes to a clock.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::minutes().start_hm(3, 30);
    ///
    /// assert_eq!(clock.next().unwrap(), Time(3, 30, 0));
    /// assert_eq!(clock.next().unwrap(), Time(3, 31, 0));
    /// assert_eq!(clock.next().unwrap(), Time(3, 32, 0));
    /// ```
    pub fn start_hm(self, hour: u8, minute: u8) -> Self {
        Self {
            hour: hour % 24,
            minute: minute % 60,
            second: 0,
            ..self
        }
    }
    /// Builder function to add a start offset in hours, minutes, and seconds to a clock.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::minutes().start_hms(3, 30, 54);
    ///
    /// assert_eq!(clock.next().unwrap(), Time(3, 30, 54));
    /// assert_eq!(clock.next().unwrap(), Time(3, 31, 54));
    /// assert_eq!(clock.next().unwrap(), Time(3, 32, 54));
    /// ```
    pub fn start_hms(self, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour: hour % 24,
            minute: minute % 60,
            second: second % 60,
            ..self
        }
    }
    /// Builder function to add an end time to a clock.
    ///
    /// End time is exclusive.
    /// The last element of the iterator is the element before the end time.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::minutes()
    ///     .start(&Time(3, 0, 0))
    ///     .end(&Time(4, 0, 0));
    ///
    /// assert_eq!(clock.next().unwrap(), Time(3, 0, 0));
    /// assert_eq!(clock.next().unwrap(), Time(3, 1, 0));
    /// // ...
    /// assert_eq!(clock.last().unwrap(), Time(3, 59, 0));
    /// ```
    pub fn end(self, time: &Time) -> Self {
        Self {
            hour_end: time.0,
            minute_end: time.1,
            second_end: time.2,
            ..self
        }
    }
    /// Builder function to add an end time in hours to a clock.
    ///
    /// End time is exclusive.
    /// The last element of the iterator is the element before the end time.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::minutes().start_h(3).end_h(4);
    ///
    /// assert_eq!(clock.next().unwrap(), Time(3, 0, 0));
    /// assert_eq!(clock.next().unwrap(), Time(3, 1, 0));
    /// // ...
    /// assert_eq!(clock.last().unwrap(), Time(3, 59, 0));
    /// ```
    pub fn end_h(self, hour: u8) -> Self {
        Self {
            hour_end: hour % 24,
            minute_end: 0,
            second_end: 0,
            ..self
        }
    }
    /// Builder function to add an end time in hours and minutes to a clock.
    ///
    /// End time is exclusive.
    /// The last element of the iterator is the element before the end time.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::minutes().start_hm(3, 30).end_hm(4, 15);
    ///
    /// assert_eq!(clock.next().unwrap(), Time(3, 30, 0));
    /// assert_eq!(clock.next().unwrap(), Time(3, 31, 0));
    /// // ...
    /// assert_eq!(clock.last().unwrap(), Time(4, 14, 0));
    /// ```
    pub fn end_hm(self, hour: u8, minute: u8) -> Self {
        Self {
            hour_end: hour % 24,
            minute_end: minute % 60,
            second_end: 0,
            ..self
        }
    }
    /// Builder function to add an end time in hours, minutes, and seconds to a clock.
    ///
    /// End time is exclusive.
    /// The last element of the iterator is the element before the end time.
    /// ```
    /// use timemachine::Clock;
    /// use timemachine::Time;
    ///
    /// let mut clock = Clock::minutes().start_hms(3, 30, 54).end_hms(4, 15, 25);
    ///
    /// assert_eq!(clock.next().unwrap(), Time(3, 30, 54));
    /// assert_eq!(clock.next().unwrap(), Time(3, 31, 54));
    /// // ...
    /// assert_eq!(clock.last().unwrap(), Time(4, 14, 54));
    /// ```
    pub fn end_hms(self, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour_end: hour % 24,
            minute_end: minute % 60,
            second_end: second % 60,
            ..self
        }
    }
}
impl Iterator for Clock {
    type Item = Time;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hour >= self.hour_end
            && self.minute >= self.minute_end
            && self.second >= self.second_end
        {
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
            Some(r)
        }
    }
}
