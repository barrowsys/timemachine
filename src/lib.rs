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
#![deny(missing_docs)]
//! The `timemachine` crate handles daily time-based state transitions
//!
//! [![GitHub last commit](https://img.shields.io/github/last-commit/barrowsys/timemachine)](https://github.com/barrowsys/timemachine)
//! [![Crates.io](https://img.shields.io/crates/v/timemachine)](https://crates.io/crates/timemachine/)
//! [![Docs.rs](https://docs.rs/timemachine/badge.svg)](https://docs.rs/timemachine)
//!
//! For the main chunk of docs, see [TimeMachine](crate::TimeMachine).
//!
//! [Napchart](napchart) support can be included with crate feature `napchart`.

use std::collections::HashMap;

mod time;
pub use time::Clock;
#[doc(inline)]
pub use time::Time;

mod error;
pub use error::ErrorKind;
use error::Result;

/// A time-based mod 24(\*60\*60) state machine.
///
/// State transitions are added with [`add_transition`](#method.add_transition),
/// and you can get the state at a given time with [`get_state`](#method.get_state).
///
/// If your state type implements [Default](core::default::Default),
/// you can also use [`get_state_or_default`](#method.get_state_or_default)
/// and [`map_states_or_default`](#method.map_states_or_default).
///
/// If the crate feature `napchart` is enabled, you can also use
/// [`from_napchart`](#method.from_napchart).
/// ### Examples
/// Simple AM = true/PM = false timemachine
/// ```
/// use timemachine::Time;
/// use timemachine::TimeMachine;
///
/// let mut tm: TimeMachine<bool> = TimeMachine::new();
/// tm.add_transition(Time::midnight(), true);
/// tm.add_transition(Time::noon(), false);
/// assert_eq!(tm.get_state(&Time::new_h(3)).unwrap(), true);
/// assert_eq!(tm.get_state(&Time::new_h(20)).unwrap(), false);
/// ```
#[derive(Debug, Default)]
pub struct TimeMachine<S: Clone> {
    edges: HashMap<Time, S>,
}
impl<S: Clone> TimeMachine<S> {
    /// Creates a new, empty timemachine.
    ///
    /// Hypothetically you can call TimeMachine::default() but the compiler gets mad if your state
    /// type doesn't also implement default.
    ///
    /// ```
    /// use timemachine::TimeMachine;
    /// use timemachine::Time;
    /// use timemachine::ErrorKind;
    ///
    /// let mut tm: TimeMachine<i32> = TimeMachine::new();
    ///
    /// assert_eq!(tm.get_state(&Time::midnight()), Err(ErrorKind::EmptyTimeMachine));
    /// ```
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }
    /// Maps a TimeMachine\<S\> into a TimeMachine\<R\> with a mapping function.
    ///
    /// ```
    /// use timemachine::TimeMachine;
    /// use timemachine::Time;
    ///
    /// let mut tm: TimeMachine<i32> = TimeMachine::new();
    /// // Adds a transition to state `-50` at midnight.
    /// tm.add_transition(Time::midnight(), -50);
    /// // Adds a transition to state `73` at 6AM.
    /// tm.add_transition(Time::new_h(6), 73);
    /// // Adds a transition to state `25` at noon.
    /// tm.add_transition(Time::noon(), 25);
    /// // Adds a transition to state `-37` at 6PM.
    /// tm.add_transition(Time::new_h(18), -37);
    ///
    /// // Map the TimeMachine<i32> to a TimeMachine<bool> by mapping
    /// // positive values to true and negative values to false
    /// let mut tm: TimeMachine<bool> = tm.map_states(|s| s >= 0);
    ///
    /// assert_eq!(tm.get_state(&Time::new_h(1)).unwrap(), false);
    /// assert_eq!(tm.get_state(&Time::new_h(7)).unwrap(), true);
    /// assert_eq!(tm.get_state(&Time::new_h(13)).unwrap(), true);
    /// assert_eq!(tm.get_state(&Time::new_h(19)).unwrap(), false);
    /// ```
    pub fn map_states<F, R>(self, mapfn: F) -> TimeMachine<R>
    where
        F: Fn(S) -> R,
        R: Clone,
    {
        TimeMachine {
            edges: self.edges.into_iter().map(|(k, v)| (k, mapfn(v))).collect(),
        }
    }
    /// Adds a transition to the given state at the given time to the timemachine.
    ///
    /// ```
    /// use timemachine::TimeMachine;
    /// use timemachine::Time;
    ///
    /// let mut tm: TimeMachine<bool> = TimeMachine::new();
    /// // Adds a transition to state `false` at midnight.
    /// tm.add_transition(Time::midnight(), false);
    /// // Adds a transition to state `true` at noon.
    /// tm.add_transition(Time::noon(), true);
    /// assert_eq!(tm.get_state(&Time::new_h(6)).unwrap(), false);
    /// assert_eq!(tm.get_state(&Time::new_h(18)).unwrap(), true);
    /// ```
    pub fn add_transition(&mut self, time: Time, state: S) {
        self.edges.insert(time, state);
    }
    fn get_edges(&self, time: &Time) -> Result<((Time, S), (Time, S))> {
        if self.edges.is_empty() {
            return Err(ErrorKind::EmptyTimeMachine);
        }
        let mut prev_edge = Time::midnight();
        let mut next_edge = Time::from_minutes(-1);
        let mut last_time = prev_edge.clone();
        let mut first_time = next_edge.clone();
        for edge in self.edges.keys() {
            if time >= edge && edge >= &prev_edge {
                prev_edge = edge.clone();
            }
            if time < edge && edge < &next_edge {
                next_edge = edge.clone();
            }
            if edge > &last_time {
                last_time = edge.clone();
            }
            if edge < &first_time {
                first_time = edge.clone();
            }
        }
        let prev_ret = if let Some(state) = self.edges.get(&prev_edge) {
            (prev_edge.clone(), state.clone())
        } else {
            (
                last_time.clone(),
                self.edges.get(&last_time).unwrap().clone(),
            )
        };
        let next_ret = if let Some(state) = self.edges.get(&next_edge) {
            (next_edge.clone(), state.clone())
        } else {
            (
                first_time.clone(),
                self.edges.get(&first_time).unwrap().clone(),
            )
        };
        Ok((prev_ret, next_ret))
    }
    /// Returns the state that the timemachine is in at the given time.
    ///
    /// ```
    /// use timemachine::TimeMachine;
    /// use timemachine::Time;
    ///
    /// let mut tm: TimeMachine<bool> = TimeMachine::new();
    /// tm.add_transition(Time::midnight(), false);
    /// tm.add_transition(Time::noon(), true);
    ///
    /// let state_3am = tm.get_state(&Time::new_h(3)).unwrap();
    /// assert_eq!(state_3am, false);
    ///
    /// let state_6am = tm.get_state(&Time::new_h(6)).unwrap();
    /// assert_eq!(state_6am, false);
    ///
    /// let state_6pm = tm.get_state(&Time::new_h(18)).unwrap();
    /// assert_eq!(state_6pm, true);
    ///
    /// let state_9pm = tm.get_state(&Time::new_h(21)).unwrap();
    /// assert_eq!(state_9pm, true);
    /// ```
    pub fn get_state(&self, time: &Time) -> Result<S> {
        Ok(self.get_edges(time)?.0 .1)
    }
    /// Returns a tuple of the current state, the progress through the current state,
    /// and the next state.
    /// This is useful for things like interpolating a value between two states.
    ///
    /// ```
    /// use timemachine::TimeMachine;
    /// use timemachine::Time;
    ///
    /// let mut tm: TimeMachine<bool> = TimeMachine::new();
    /// tm.add_transition(Time::midnight(), false);
    /// tm.add_transition(Time::noon(), true);
    ///
    /// // At 3AM, our current state is false, we are 25% through the current state,
    /// // and our next state is true
    /// let progress_3am = tm.get_state_progress(&Time::new_h(3)).unwrap();
    /// assert_eq!(progress_3am, (false, 0.25f64, true));
    ///
    /// // At 6AM, our current state is false, we are 50% through the current state,
    /// // and our next state is true
    /// let progress_6am = tm.get_state_progress(&Time::new_h(6)).unwrap();
    /// assert_eq!(progress_6am, (false, 0.5f64, true));
    ///
    /// // At 6PM, our current state is true, we are 50% through the current state,
    /// // and our next state is false
    /// let progress_6pm = tm.get_state_progress(&Time::new_h(18)).unwrap();
    /// assert_eq!(progress_6pm, (true, 0.5f64, false));
    ///
    /// // At 9PM, our current state is true, we are 75% through the current state,
    /// // and our next state is false
    /// let progress_9pm = tm.get_state_progress(&Time::new_h(21)).unwrap();
    /// assert_eq!(progress_9pm, (true, 0.75f64, false));
    /// ```
    pub fn get_state_progress(&self, _time: &Time) -> Result<(S, f64, S)> {
        let ((pedge, pstate), (nedge, nstate)) = self.get_edges(_time)?;
        let seconds_between: f64 = pedge.secs_until(&nedge).into();
        let seconds_elapsed: f64 = pedge.secs_until(_time).into();
        let progress = seconds_elapsed / seconds_between;
        Ok((pstate, progress, nstate))
    }
}
/// Extra functions if your state type implements [Default](core::default::Default)
impl<S: Clone + Default> TimeMachine<S> {
    /// Maps a TimeMachine\<S\> into a TimeMachine\<R: Default\> with a mapping function.
    ///
    /// If the mapping function returns Some, the state is set to that value.
    /// If the mapping function returns None, the state is set to default().
    ///
    /// ```
    /// use timemachine::TimeMachine;
    /// use timemachine::Time;
    /// use std::convert::TryInto;
    ///
    /// let mut tm: TimeMachine<i8> = TimeMachine::new();
    /// // Adds a transition to state `-50` at midnight.
    /// tm.add_transition(Time::midnight(), -50);
    /// // Adds a transition to state `73` at 6AM.
    /// tm.add_transition(Time::new_h(6), 73);
    /// // Adds a transition to state `25` at noon.
    /// tm.add_transition(Time::noon(), 25);
    /// // Adds a transition to state `-37` at 6PM.
    /// tm.add_transition(Time::new_h(18), -37);
    ///
    /// // Map the TimeMachine<i8> to a TimeMachine<u8> by mapping
    /// // positive values to themselves and negative values to default
    /// let mut tm: TimeMachine<u8> = tm.map_states_or_default(|s| s.try_into().ok());
    ///
    /// assert_eq!(tm.get_state(&Time::new_h(1)).unwrap(), 0);
    /// assert_eq!(tm.get_state(&Time::new_h(7)).unwrap(), 73);
    /// assert_eq!(tm.get_state(&Time::new_h(13)).unwrap(), 25);
    /// assert_eq!(tm.get_state(&Time::new_h(19)).unwrap(), 0);
    /// ```
    pub fn map_states_or_default<F, R>(self, mapfn: F) -> TimeMachine<R>
    where
        F: Fn(S) -> Option<R>,
        R: Clone + Default,
    {
        TimeMachine {
            edges: self
                .edges
                .into_iter()
                .map(|(k, v)| (k, mapfn(v).unwrap_or_default()))
                .collect(),
        }
    }
    /// Returns the state that the timemachine is in at the given time.
    /// If the timemachine is empty, it will return S.default().
    ///
    /// This method does not handle any errors except EmptyTimeMachine.
    ///
    /// ```
    /// use timemachine::TimeMachine;
    /// use timemachine::Time;
    /// use timemachine::ErrorKind;
    ///
    /// let mut tm: TimeMachine<i32> = TimeMachine::new();
    ///
    /// assert_eq!(tm.get_state(&Time::midnight()), Err(ErrorKind::EmptyTimeMachine));
    /// assert_eq!(tm.get_state_or_default(&Time::midnight()), Ok(0));
    /// ```
    pub fn get_state_or_default(&self, time: &Time) -> Result<S> {
        if self.edges.is_empty() {
            Ok(S::default())
        } else {
            self.get_state(time)
        }
    }
}
#[cfg(feature = "napchart")]
impl TimeMachine<Option<napchart::ElementData>> {
    /// Creates a TimeMachine\<Option\<<napchart::ElementData>\>\> from a <napchart::ChartLane>.
    ///
    /// Elements in the lane will be added as Some([ElementData](napchart::ElementData)).
    /// Empty space in the lane will be added as None.
    ///
    /// You can use [`map_states`](#method.map_states) or
    /// [`map_states_or_default`](#method.map_states_or_default)
    /// to turn this into something more useful.
    /// ```
    /// use napchart::api::BlockingClient;
    /// use timemachine::TimeMachine;
    /// use timemachine::Time;
    ///
    /// let client = BlockingClient::default();
    /// // chart link: https://napchart.com/jex3y
    /// let chart = client.get("jex3y").unwrap();
    /// let lane = chart.lanes.get(0).unwrap();
    /// let mut tm = TimeMachine::from_napchart(&lane)
    ///     // Map Option<ElementData> to String by taking data.color, otherwise "blank"
    ///     .map_states(|e| match e {
    ///         Some(data) => data.color,
    ///         None => String::from("blank"),
    ///     });
    ///
    /// let state_4am = tm.get_state(&Time::new_h(4)).unwrap();
    /// assert_eq!(state_4am, String::from("red"));
    ///
    /// let state_noon = tm.get_state(&Time::new_h(12)).unwrap();
    /// assert_eq!(state_noon, String::from("blue"));
    ///
    /// let state_8pm = tm.get_state(&Time::new_h(20)).unwrap();
    /// assert_eq!(state_8pm, String::from("blank"));
    /// ```
    pub fn from_napchart(lane: &napchart::ChartLane) -> TimeMachine<Option<napchart::ElementData>> {
        let mut tm = TimeMachine::new();
        for elem in lane.elements_iter() {
            tm.add_transition(
                Time::from_minutes(elem.start as i16),
                Some(elem.data.clone()),
            );
            if !tm.edges.contains_key(&Time::from_minutes(elem.end as i16)) {
                tm.add_transition(Time::from_minutes(elem.end as i16), None);
            }
        }
        tm
    }
}
#[cfg(feature = "graphviz")]
impl<S: Clone + std::fmt::Display> TimeMachine<S> {
    /// WIP function to generate a graphviz spec from a timemachine
    pub fn generate_graphviz(&self) -> Result<String> {
        if self.edges.is_empty() {
            return Err(ErrorKind::EmptyTimeMachine);
        }
        let mut buffer = String::new();
        let mut times: Vec<&Time> = self.edges.keys().collect();
        times.sort();
        let mut times2 = times.clone();
        let time2 = times2.remove(0);
        times2.push(time2);
        let midnight = self.get_edges(&Time::midnight())?.0;
        buffer.push_str(&format!("    t{}\n", midnight.0).replace(":", "_"));
        for (time, time2) in times.iter().zip(times2.iter()) {
            let ftime = format!("t{}", time).replace(":", "_");
            let ftime2 = format!("t{}", time2).replace(":", "_");
            buffer.push_str(&format!(
                "    {} [label=\"{}\"]\n",
                ftime,
                self.edges.get(time).unwrap()
            ));
            buffer.push_str(&format!(
                "    {} -> {} [label=\"{}\"]\n",
                ftime, ftime2, time2
            ));
        }
        // for edge in self.edges.iter() {
        //     buffer.push_str(format!("    {} -> {} [label=\"{}\"]\n",
        // }
        Ok(format!("digraph {{\n{}}}", buffer))
    }
}
