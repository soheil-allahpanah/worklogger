//! Bottom keybinding hints in the style of Zellij's status bar.

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, Mode};
use crate::theme;
use domain::traits::WorklogRepository;

struct Hint<'a> {
    key: &'a str,
    action: &'a str,
}

/// Renders mode-aware keybinding hints into `area`.
pub fn view<R: WorklogRepository>(frame: &mut Frame, app: &App<R>, area: Rect) {
    let hints: &[Hint] = match app.mode {
        Mode::Normal => &[
            Hint {
                key: "Ctrl c",
                action: "QUIT",
            },
            Hint {
                key: "q",
                action: "QUIT",
            },
            Hint {
                key: "/",
                action: "SEARCH",
            },
            Hint {
                key: "n",
                action: "ADD",
            },
            Hint {
                key: "d",
                action: "DELETE",
            },
            Hint {
                key: "o",
                action: "OPEN",
            },
            Hint {
                key: "e",
                action: "EXPORT",
            },
            Hint {
                key: "j/k",
                action: "NAVIGATE",
            },
        ],
        Mode::Search => &[
            Hint {
                key: "Enter",
                action: "APPLY",
            },
            Hint {
                key: "Esc",
                action: "CANCEL",
            },
        ],
        Mode::AddModal => &[
            Hint {
                key: "Tab",
                action: "NEXT FIELD",
            },
            Hint {
                key: "Enter",
                action: "SAVE",
            },
            Hint {
                key: "Esc",
                action: "CANCEL",
            },
        ],
        Mode::DeleteModal => &[
            Hint {
                key: "y",
                action: "YES",
            },
            Hint {
                key: "n",
                action: "NO",
            },
            Hint {
                key: "Tab",
                action: "SWITCH",
            },
            Hint {
                key: "Enter",
                action: "SELECT",
            },
        ],
        Mode::OpenModal => &[Hint {
            key: "Esc",
            action: "CLOSE",
        }],
    };

    let mut spans = Vec::new();
    for (i, hint) in hints.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  |  ", Style::default().fg(theme::BORDER)));
        }
        spans.push(Span::styled(
            hint.key,
            Style::default()
                .fg(theme::AMBER)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            hint.action,
            Style::default().fg(theme::MUTED),
        ));
    }

    let bar = Paragraph::new(Line::from(spans)).style(Style::default().bg(theme::BG));
    frame.render_widget(bar, area);
}
