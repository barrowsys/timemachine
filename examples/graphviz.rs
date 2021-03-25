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

use timemachine::{Time, TimeMachine};

fn oldmain() {
    let mut tm: TimeMachine<i32> = TimeMachine::new();
    tm.add_transition(Time::new_h(3), -50);
    tm.add_transition(Time::new_h(6), 73);
    tm.add_transition(Time::noon(), 25);
    tm.add_transition(Time::new_h(18), -37);
    println!("{}", tm.generate_graphviz().unwrap());
}

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    NightDark,
    DuskDawnRed,
    DayWhite,
}
impl Default for State {
    fn default() -> State {
        State::DayWhite
    }
}
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = napchart::api::BlockingClient::default();
    let napchart = api_client.get("cse2j")?;

    let tm: TimeMachine<Option<napchart::ElementData>> =
        TimeMachine::from_napchart(&napchart.lanes[0]);

    let tm: TimeMachine<String> = tm.map_states(|e| match e {
        Some(data) => data.color,
        None => String::from("blank"),
    });
    println!("{}", tm.generate_graphviz().unwrap());
    Ok(())
}
