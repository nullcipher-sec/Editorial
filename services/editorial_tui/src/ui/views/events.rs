use crate::app::{App, SubView};
use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn render_events(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(area);

    let tab_text = match app.sub_view {
        SubView::List1 => " [Listado de Eventos]  |  [N] Programar Evento  |  [D] Eliminar Evento",
        SubView::Form1 => " [FORMULARIO: Programación de Evento Literario]",
        _ => " [Módulo de Eventos]",
    };

    let title_block = Paragraph::new(Line::from(vec![
        Span::styled(" GESTIÓN DE EVENTOS Y LANZAMIENTOS ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
        Span::styled(tab_text, Style::default().fg(SULTANA_THEME.text_dim)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(SULTANA_THEME.border)));
    frame.render_widget(title_block, chunks[0]);

    match app.sub_view {
        SubView::List1 => render_events_table(frame, chunks[1], app),
        SubView::Form1 => render_new_event_form(frame, chunks[1], app),
        _ => render_events_table(frame, chunks[1], app),
    }
}

fn render_events_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["#", "Nombre del Evento", "Ubicación / Sede", "Inicio", "Fin"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.events.iter().enumerate().map(|(idx, ev)| {
        let is_selected = idx == app.list_index;
        let style = if is_selected {
            Style::default().fg(SULTANA_THEME.selected_fg).bg(SULTANA_THEME.selected_bg)
        } else {
            Style::default().fg(SULTANA_THEME.text_bright)
        };

        let start_str = ev.starts_at.format("%Y-%m-%d %H:%M").to_string();
        let end_str = ev
            .ends_at
            .map(|e| e.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "Indefinido".to_string());

        Row::new(vec![
            Cell::from(format!("{}", idx + 1)).style(style),
            Cell::from(ev.name.clone()).style(style),
            Cell::from(ev.location.clone().unwrap_or_else(|| "Virtual / Sin sede".to_string())).style(style),
            Cell::from(start_str).style(style),
            Cell::from(end_str).style(style),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Agenda de Eventos — [↑/↓] Navegar, [N] Programar Evento, [D] Eliminar Evento, [Esc] Volver ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.primary)),
    );

    frame.render_widget(table, area);
}

fn render_new_event_form(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(12), Constraint::Min(2)])
        .split(area);

    let f1 = if app.active_field == 0 { "_" } else { "" };
    let f2 = if app.active_field == 1 { "_" } else { "" };
    let f3 = if app.active_field == 2 { "_" } else { "" };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Nombre del Evento:  ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_1, f1), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Ubicación / Sala:   ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_2, f2), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Descripción breve:  ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_3, f3), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Tab] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Siguiente campo    ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled("[Enter] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Programar Evento    ", Style::default().fg(SULTANA_THEME.success)),
            Span::styled("[Esc] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Cancelar", Style::default().fg(SULTANA_THEME.danger)),
        ]),
    ];

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(" Programar Nuevo Evento Literario (Valida fechas y campos) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.accent)),
    );
    frame.render_widget(p, chunks[0]);
}
