use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_footer(frame: &mut Frame, area: Rect, context_hint: &str) {
    let line = Line::from(vec![
        Span::styled(
            " [Q] ",
            Style::default()
                .fg(SULTANA_THEME.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Salir  ", Style::default().fg(SULTANA_THEME.text_bright)),
        Span::styled(
            " [Esc] ",
            Style::default()
                .fg(SULTANA_THEME.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Volver/Menú  ", Style::default().fg(SULTANA_THEME.text_bright)),
        Span::styled(
            " [↑/↓] ",
            Style::default()
                .fg(SULTANA_THEME.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Moverse  ", Style::default().fg(SULTANA_THEME.text_bright)),
        Span::styled(
            " [Enter] ",
            Style::default()
                .fg(SULTANA_THEME.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Seleccionar  ", Style::default().fg(SULTANA_THEME.text_bright)),
        Span::styled(
            format!("|  {}", context_hint),
            Style::default().fg(SULTANA_THEME.text_dim),
        ),
    ]);

    let widget = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(SULTANA_THEME.border)),
    );

    frame.render_widget(widget, area);
}
