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

/// timemachine error type
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    /// Unimplemented function. This is my bad.
    NotImplemented,
    /// Attempted to get the current state of an empty TimeMachine. This is your bad.
    EmptyTimeMachine,
    /// Generic Error
    Generic(String),
}
pub(crate) type Result<T> = std::result::Result<T, ErrorKind>;

// use crate::impl_from;
// #[doc(hidden)]
// #[macro_export]
// macro_rules! impl_from {
//     ($from:path, $to:expr) => {
//         impl From<$from> for ErrorKind {
//             fn from(e: $from) -> Self {
//                 $to(e)
//             }
//         }
//     };
// }
