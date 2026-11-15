use crate::auth::Session;
use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_header(frame: &mut Frame, area: Rect, session: Option<&Session>, mock_mode: bool) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let title_line = Line::from(vec![
        Span::styled(
            " SULTANA DEL LAGO EDITORES ",
            Style::default()
                .fg(SULTANA_THEME.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "| Terminal Interna",
            Style::default().fg(SULTANA_THEME.text_dim),
        ),
    ]);

    let title_widget = Paragraph::new(title_line).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(SULTANA_THEME.border)),
    );
    frame.render_widget(title_widget, chunks[0]);

    let mode_tag = if mock_mode {
        Span::styled(
            "[SIMULACIÓN EN MEMORIA] ",
            Style::default()
                .fg(SULTANA_THEME.warning)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            "[CORE; FALLBACK DE SIMULACIÓN] ",
            Style::default()
                .fg(SULTANA_THEME.success)
                .add_modifier(Modifier::BOLD),
        )
    };

    let user_info = if let Some(s) = session {
        vec![
            mode_tag,
            Span::styled("Usuario: ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(
                format!("{} ", s.user.display_name),
                Style::default()
                    .fg(SULTANA_THEME.text_bright)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("({}) ", s.user.area.display_name()),
                Style::default().fg(SULTANA_THEME.accent),
            ),
        ]
    } else {
        vec![
            mode_tag,
            Span::styled(
                "No autenticado ",
                Style::default().fg(SULTANA_THEME.danger),
            ),
        ]
    };

    let status_widget = Paragraph::new(Line::from(user_info))
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(SULTANA_THEME.border)),
        );
    frame.render_widget(status_widget, chunks[1]);
}
