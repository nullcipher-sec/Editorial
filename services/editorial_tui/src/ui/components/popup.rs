use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupKind {
    Info,
    Success,
    Warning,
    Error,
}

impl PopupKind {
    pub fn color(&self) -> Color {
        match self {
            PopupKind::Info => SULTANA_THEME.primary,
            PopupKind::Success => SULTANA_THEME.success,
            PopupKind::Warning => SULTANA_THEME.warning,
            PopupKind::Error => SULTANA_THEME.danger,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            PopupKind::Info => " INFORMACIÓN ",
            PopupKind::Success => " OPERACIÓN EXITOSA ",
            PopupKind::Warning => " ADVERTENCIA ",
            PopupKind::Error => " ERROR DEL SISTEMA ",
        }
    }
}

pub fn render_popup(frame: &mut Frame, kind: PopupKind, message: &str) {
    let size = frame.area();

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Percentage(30),
        ])
        .split(size);

    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(popup_layout[1])[1];

    frame.render_widget(Clear, area);

    let color = kind.color();
    let block = Block::default()
        .title(Span::styled(
            kind.title(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color));

    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            message,
            Style::default().fg(SULTANA_THEME.text_bright),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Presione [Enter] o [Esc] para continuar",
            Style::default().fg(SULTANA_THEME.text_dim),
        )),
    ];

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
