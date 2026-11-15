use crate::app::{App, SubView};
use crate::domain::CaseStatus;
use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn render_attention(frame: &mut Frame, area: Rect, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(area);

    let tab_text = match app.sub_view {
        SubView::List1 => " [Casos de Soporte]  |  (Presione [C] para ver Clientes)  |  (Presione [N] para Nuevo Caso)",
        SubView::List2 => " (Presione [V] para ver Casos)  |  [Listado de Clientes]  |  (Presione [R] para Registrar Cliente)",
        SubView::Form1 => " [FORMULARIO: Registrar Nuevo Cliente]",
        SubView::Form2 => " [FORMULARIO: Registrar Nuevo Caso de Soporte]",
        _ => " [Módulo de Atención al Cliente]",
    };

    let title_block = Paragraph::new(Line::from(vec![
        Span::styled(" ÁREA DE ATENCIÓN AL CLIENTE ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
        Span::styled(tab_text, Style::default().fg(SULTANA_THEME.text_dim)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(SULTANA_THEME.border)));
    frame.render_widget(title_block, main_chunks[0]);

    match app.sub_view {
        SubView::List1 => render_cases_table(frame, main_chunks[1], app),
        SubView::List2 => render_clients_table(frame, main_chunks[1], app),
        SubView::Form1 => render_new_client_form(frame, main_chunks[1], app),
        SubView::Form2 => render_new_case_form(frame, main_chunks[1], app),
        _ => render_cases_table(frame, main_chunks[1], app),
    }
}

fn render_cases_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["#", "Asunto del Caso", "Estado", "ID Caso", "Acción rápida"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.cases.iter().enumerate().map(|(idx, c)| {
        let is_selected = idx == app.list_index;
        let status_color = match c.status {
            CaseStatus::Open => SULTANA_THEME.danger,
            CaseStatus::InProgress => SULTANA_THEME.warning,
            CaseStatus::Closed => SULTANA_THEME.success,
        };

        let style = if is_selected {
            Style::default().fg(SULTANA_THEME.selected_fg).bg(SULTANA_THEME.selected_bg)
        } else {
            Style::default().fg(SULTANA_THEME.text_bright)
        };

        Row::new(vec![
            Cell::from(format!("{}", idx + 1)).style(style),
            Cell::from(c.subject.clone()).style(style),
            Cell::from(c.status.display_name()).style(Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Cell::from(c.id.to_string()[..8].to_string()).style(style),
            Cell::from("[S] Cambiar Estado").style(Style::default().fg(SULTANA_THEME.accent)),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Percentage(45),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Casos de Atención Registrados — Use [↑/↓] para navegar, [S] para avanzar estado ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.primary)),
    );

    frame.render_widget(table, area);
}

fn render_clients_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["#", "Nombre o Razón Social", "Correo Electrónico", "Teléfono de Contacto"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.clients.iter().enumerate().map(|(idx, cl)| {
        let is_selected = idx == app.list_index;
        let style = if is_selected {
            Style::default().fg(SULTANA_THEME.selected_fg).bg(SULTANA_THEME.selected_bg)
        } else {
            Style::default().fg(SULTANA_THEME.text_bright)
        };

        Row::new(vec![
            Cell::from(format!("{}", idx + 1)).style(style),
            Cell::from(cl.full_name.clone()).style(style),
            Cell::from(cl.email.clone().unwrap_or_else(|| "N/A".to_string())).style(style),
            Cell::from(cl.phone.clone().unwrap_or_else(|| "N/A".to_string())).style(style),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(25),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Directorio de Clientes — Use [R] para nuevo cliente, [V] para volver a casos ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.primary)),
    );

    frame.render_widget(table, area);
}

fn render_new_client_form(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(2)])
        .split(area);

    let f1 = if app.active_field == 0 { "_" } else { "" };
    let f2 = if app.active_field == 1 { "_" } else { "" };
    let f3 = if app.active_field == 2 { "_" } else { "" };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Nombre o Empresa: ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_1, f1), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Correo Electrónico: ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_2, f2), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Teléfono:          ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_3, f3), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Tab] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Siguiente campo    ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled("[Enter] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Guardar Cliente    ", Style::default().fg(SULTANA_THEME.success)),
            Span::styled("[Esc] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Cancelar", Style::default().fg(SULTANA_THEME.danger)),
        ]),
    ];

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(" Registrar Nuevo Cliente (Validado con Validator) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.accent)),
    );
    frame.render_widget(p, chunks[0]);
}

fn render_new_case_form(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(12), Constraint::Min(2)])
        .split(area);

    let client_name = if !app.clients.is_empty() {
        &app.clients[0].full_name
    } else {
        "Cliente Predeterminado"
    };

    let f1 = if app.active_field == 0 { "_" } else { "" };
    let f2 = if app.active_field == 1 { "_" } else { "" };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Cliente Asignado: ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(client_name, Style::default().fg(SULTANA_THEME.accent)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Asunto del Caso:  ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_1, f1), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Descripción:      ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_2, f2), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Tab] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Siguiente campo    ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled("[Enter] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Crear Caso    ", Style::default().fg(SULTANA_THEME.success)),
            Span::styled("[Esc] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Cancelar", Style::default().fg(SULTANA_THEME.danger)),
        ]),
    ];

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(" Registrar Caso de Soporte Técnico ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.accent)),
    );
    frame.render_widget(p, chunks[0]);
}
