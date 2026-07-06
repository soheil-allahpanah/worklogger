//! The root message type and the control-flow outcome of an update.
//!
//! Following the Elm architecture, every input is translated into a [`Msg`]
//! (see [`crate::app::from_key`]), and [`crate::app::update`] consumes it to
//! mutate the model. Each screen/dialog owns its own message enum; the root
//! [`Msg`] simply routes to the active one.

use crate::components::{search_bar, table};
use crate::dialogs::{add, delete, edit, open};

/// A message describing something that happened (a key press, a tick, etc.).
#[derive(Debug, Clone)]
pub enum Msg {
    /// The clock ticked; advance time-based state (cursor blink, status expiry).
    Tick,
    /// Quit the application.
    Quit,
    /// A message for the main worklog table screen.
    Table(table::Msg),
    /// A message for the search bar.
    Search(search_bar::Msg),
    /// A message for the "add worklog" dialog.
    Add(add::Msg),
    /// A message for the "delete worklog" dialog.
    Delete(delete::Msg),
    /// A message for the "edit worklog" dialog.
    Edit(edit::Msg),
    /// A message for the "open worklog details" dialog.
    Open(open::Msg),
}

/// What the runtime loop should do after an update.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Keep running.
    Continue,
    /// Exit the application.
    Quit,
}
