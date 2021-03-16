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

use console::Term;
use napchart::Napchart;
#[allow(deprecated)]
use std::thread::sleep_ms;
use timemachine::{Clock, TimeMachine};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let napchart = Napchart::get_from_server("3tbkt")?;

    let tm: TimeMachine<Option<napchart::ElementData>> =
        TimeMachine::from_napchart(&napchart.lanes[0]);

    let tm: TimeMachine<State> = tm.map_states_or_default(|elem| {
        // use map_states_or_default to .unwrap_or_default the return value
        if let Some(e) = elem {
            match e.color.as_str() {
                "red" => Some(State::DuskDawnRed),
                "gray" => Some(State::NightDark),
                _ => None, // no valid mapping, element is replaced with State::default()
                           // Note that the element is not ignored how u might think, it is replaced with the default state
                           // Surrounding elements do not expand to fill the space
            }
        } else {
            None
        }
    });

    // Display Code
    let term = Term::stdout();
    term.write_line("")?;
    term.hide_cursor()?;
    for t in Clock::minutes() {
        term.write_line(&format!("The time is {:#.2}    ", &t))?;
        term.write_str(match tm.get_state(&t).ok().unwrap() {
            State::NightDark => "it is dark           ",
            State::DuskDawnRed => "the red light is on  ",
            State::DayWhite => "the white light is on",
        })?;
        #[allow(deprecated)]
        sleep_ms(25);
        term.move_cursor_up(1)?;
        term.move_cursor_left(21)?;
    }
    term.show_cursor()?;
    term.move_cursor_down(2)?;
    Ok(())
}
