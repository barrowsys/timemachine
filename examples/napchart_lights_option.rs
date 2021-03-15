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
use timemachine::{Clock, Time, TimeMachine};

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    NightDark,
    DuskDawnRed,
    DayWhite,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let napchart = Napchart::get_from_server("cse2j")?;
    let tm: TimeMachine<Option<State>> = TimeMachine::from_napchart(&napchart.lanes[0], |elem| {
        match elem.color.as_str() {
            "red" => Some(Some(State::DuskDawnRed)), // Outer Some() indicates the mapping is valid
            "gray" => Some(Some(State::NightDark)), // Inner Some() is the Option<State> that gets added to the time machine
            "blue" => Some(Some(State::DayWhite)),
            _ => None, // Return None to indicate there is no mapping and replace it with State::default()
            // Really, you should use a type that implements default as your state type,
            // rather than wrapping it in an Option<T>.
            // See examples/napchart_lights.rs for something that makes more sense
        }
    });
    let term = Term::stdout();
    term.write_line("")?;
    term.hide_cursor()?;
    let mut err_count = 0;
    for t in Clock::minutes() {
        term.write_line(&format!("The time is {:#.2}    ", &t))?;
        term.write_str(match tm.get_state(&t).ok().unwrap() {
            Some(State::NightDark) => "it is dark           ",
            Some(State::DuskDawnRed) => "the red light is on  ",
            Some(State::DayWhite) => "the white light is on",
            None => {
                if t.as_minutes() % 5 == 0 {
                    // napchart only allows 5 minute precision
                    err_count += 1;
                    for i in 0..err_count {
                        term.write_line("")?;
                    }
                    term.write_str(&format!("!!!!! at {}, state is None!!", &t))?;
                    // term.write_line("");
                    term.move_cursor_left(28)?;
                    term.move_cursor_up(err_count)?;
                }
                "ERROR                "
            }
        })?;
        sleep_ms(25);
        term.move_cursor_up(1)?;
        term.move_cursor_left(21)?;
    }
    term.show_cursor()?;
    term.move_cursor_down(2 + err_count)?;
    Ok(())
}
