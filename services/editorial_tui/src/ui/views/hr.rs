use crate::app::{App, SubView};
use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn render_hr(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(area);

    let tab_text = match app.sub_view {
        SubView::List1 => " [1. Empleados]  |  (Presione [V] para Vacantes)  |  (Presione [C] para Candidatos)",
        SubView::List2 => " (Presione [E] para Empleados)  |  [2. Vacantes Abiertas]  |  [N] Nueva Vacante",
        SubView::List3 => " (Presione [E] para Empleados)  |  [3. Candidatos en Proceso]  |  [P] Registrar Postulante",
        SubView::Form1 => " [FORMULARIO: Apertura de Nueva Vacante Laboral]",
        SubView::Form2 => " [FORMULARIO: Registrar Nuevo Candidato / Postulante]",
        _ => " [Módulo de Recursos Humanos y Reclutamiento]",
    };

    let title_block = Paragraph::new(Line::from(vec![
        Span::styled(" RECURSOS HUMANOS Y RECLUTAMIENTO ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
        Span::styled(tab_text, Style::default().fg(SULTANA_THEME.text_dim)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(SULTANA_THEME.border)));
    frame.render_widget(title_block, chunks[0]);

    match app.sub_view {
        SubView::List1 => render_employees_table(frame, chunks[1], app),
        SubView::List2 => render_openings_table(frame, chunks[1], app),
        SubView::List3 => render_candidates_table(frame, chunks[1], app),
        SubView::Form1 => render_new_opening_form(frame, chunks[1], app),
        SubView::Form2 => render_new_candidate_form(frame, chunks[1], app),
        _ => render_employees_table(frame, chunks[1], app),
    }
}

fn render_employees_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["Código", "Nombre Completo", "Correo Institucional", "Departamento"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.employees.iter().enumerate().map(|(idx, e)| {
        let is_selected = idx == app.list_index;
        let style = if is_selected {
            Style::default().fg(SULTANA_THEME.selected_fg).bg(SULTANA_THEME.selected_bg)
        } else {
            Style::default().fg(SULTANA_THEME.text_bright)
        };

        Row::new(vec![
            Cell::from(e.employee_code.clone()).style(style),
            Cell::from(e.full_name.clone()).style(style),
            Cell::from(e.email.clone()).style(style),
            Cell::from(e.department.display_name()).style(style),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(12),
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Directorio de Personal — [V] Ver Vacantes, [C] Ver Candidatos ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.primary)),
    );

    frame.render_widget(table, area);
}

fn render_openings_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["#", "Puesto / Vacante", "Área Destino", "Estado"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.openings.iter().enumerate().map(|(idx, o)| {
        let is_selected = idx == app.list_index;
        let style = if is_selected {
            Style::default().fg(SULTANA_THEME.selected_fg).bg(SULTANA_THEME.selected_bg)
        } else {
            Style::default().fg(SULTANA_THEME.text_bright)
        };

        let status_span = if o.open {
            Span::styled("ABIERTA", Style::default().fg(SULTANA_THEME.success).add_modifier(Modifier::BOLD))
        } else {
            Span::styled("CERRADA", Style::default().fg(SULTANA_THEME.text_dim))
        };

        Row::new(vec![
            Cell::from(format!("{}", idx + 1)).style(style),
            Cell::from(o.title.clone()).style(style),
            Cell::from(o.area.display_name()).style(style),
            Cell::from(Line::from(status_span)),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
            Constraint::Percentage(15),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Vacantes Laborales — [N] Crear Nueva Vacante, [E] Volver a Empleados ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.primary)),
    );

    frame.render_widget(table, area);
}

fn render_candidates_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["#", "Nombre del Candidato", "Fase del Proceso", "ID Vacante"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.candidates.iter().enumerate().map(|(idx, cand)| {
        let is_selected = idx == app.list_index;
        let style = if is_selected {
            Style::default().fg(SULTANA_THEME.selected_fg).bg(SULTANA_THEME.selected_bg)
        } else {
            Style::default().fg(SULTANA_THEME.text_bright)
        };

        Row::new(vec![
            Cell::from(format!("{}", idx + 1)).style(style),
            Cell::from(cand.full_name.clone()).style(style),
            Cell::from(cand.stage.clone()).style(Style::default().fg(SULTANA_THEME.warning).add_modifier(Modifier::BOLD)),
            Cell::from(cand.job_opening_id.to_string()[..8].to_string()).style(style),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Percentage(45),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Seguimiento de Reclutamiento — [P] Postular Candidato, [E] Volver a Empleados ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.primary)),
    );

    frame.render_widget(table, area);
}

fn render_new_opening_form(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(2)])
        .split(area);

    let f1 = if app.active_field == 0 { "_" } else { "" };
    let f2 = if app.active_field == 1 { "_" } else { "" };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Título de la Vacante: ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_1, f1), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Descripción breve:    ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_2, f2), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Tab] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Siguiente campo    ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled("[Enter] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Crear Vacante    ", Style::default().fg(SULTANA_THEME.success)),
            Span::styled("[Esc] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Cancelar", Style::default().fg(SULTANA_THEME.danger)),
        ]),
    ];

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(" Formulario de Apertura de Vacante ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.accent)),
    );
    frame.render_widget(p, chunks[0]);
}

fn render_new_candidate_form(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(2)])
        .split(area);

    let f1 = if app.active_field == 0 { "_" } else { "" };
    let f2 = if app.active_field == 1 { "_" } else { "" };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Nombre del Candidato: ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_1, f1), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Correo Electrónico:   ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_2, f2), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Tab] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Siguiente campo    ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled("[Enter] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Registrar Postulación    ", Style::default().fg(SULTANA_THEME.success)),
            Span::styled("[Esc] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Cancelar", Style::default().fg(SULTANA_THEME.danger)),
        ]),
    ];

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(" Registrar Nuevo Postulante a Selección ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.accent)),
    );
    frame.render_widget(p, chunks[0]);
}
