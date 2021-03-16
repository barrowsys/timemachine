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
#![allow(missing_docs)]

use std::collections::HashMap;

mod time;
pub use time::Clock;
#[doc(inline)]
pub use time::Time;

pub enum Error {
    Generic(String),
    EmptyTimeMachine,
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default)]
pub struct TimeMachine<S: Clone> {
    edges: HashMap<Time, S>,
}
#[cfg(feature = "napchart")]
impl TimeMachine<Option<napchart::ElementData>> {
    pub fn from_napchart(lane: &napchart::ChartLane) -> TimeMachine<Option<napchart::ElementData>> {
        let mut tm = TimeMachine::new();
        for elem in lane.elements.iter() {
            tm.add_transition(Time::from_minutes(elem.start), Some(elem.data.clone()));
            if !tm.edges.contains_key(&Time::from_minutes(elem.end)) {
                tm.add_transition(Time::from_minutes(elem.end), None);
            }
        }
        tm
    }
}
impl<S: Clone + Default> TimeMachine<S> {
    // pub fn add_state(&mut self, start_time: Time, end_time: Time, state: S) {
    //     self.add_transition(start_time, state);
    //     if !self.edges.contains_key(&end_time) {
    //         self.add_transition(end_time, S::default());
    //     }
    // }
    // ^^^^ this doesnt handle edge cases very well
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
    pub fn get_state_or_default(&self, time: &Time) -> Result<S> {
        if self.edges.is_empty() {
            Ok(S::default())
        } else {
            self.get_state(time)
        }
    }
}
impl<S: Clone> TimeMachine<S> {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }
    pub fn map_states<F, R>(self, mapfn: F) -> TimeMachine<R>
    where
        F: Fn(S) -> R,
        R: Clone,
    {
        TimeMachine {
            edges: self.edges.into_iter().map(|(k, v)| (k, mapfn(v))).collect(),
        }
    }
    pub fn add_transition(&mut self, time: Time, state: S) {
        self.edges.insert(time, state);
    }
    pub fn get_state(&self, time: &Time) -> Result<S> {
        if self.edges.is_empty() {
            return Err(Error::EmptyTimeMachine);
        }
        let mut ret = Time::midnight();
        let mut lt = ret.clone();
        for edge in self.edges.keys() {
            if time >= edge && edge >= &ret {
                ret = edge.clone();
            }
            if edge > &lt {
                lt = edge.clone();
            }
        }
        // println!("{}, {}", time, ret);
        if let Some(state) = self.edges.get(&ret).cloned() {
            Ok(state)
        } else {
            Ok(self.edges.get(&lt).cloned().unwrap())
        }
    }
}
