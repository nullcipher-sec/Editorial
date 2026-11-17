use crate::app::{App, SubView};
use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn render_recruitment(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(area);

    let tab_text = match app.sub_view {
        SubView::List1 => " [1. Candidatos en Proceso]  |  [D] Retirar Postulante  |  [P] Postular a Vacante  |  [V] Ver Vacantes",
        SubView::List2 => " [2. Vacantes de Empleo]  |  [P] Agregar Candidato a esta Vacante  |  [C] Ver Candidatos",
        SubView::Form1 => " [FORMULARIO: Agregar Candidato a una Vacante Laboral]",
        _ => " [Módulo de Reclutamiento y Selección]",
    };

    let title_block = Paragraph::new(Line::from(vec![
        Span::styled(" RECLUTAMIENTO Y SELECCIÓN ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
        Span::styled(tab_text, Style::default().fg(SULTANA_THEME.text_dim)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(SULTANA_THEME.border)));
    frame.render_widget(title_block, chunks[0]);

    match app.sub_view {
        SubView::List1 => render_candidates_table(frame, chunks[1], app),
        SubView::List2 => render_openings_table(frame, chunks[1], app),
        SubView::Form1 => render_new_candidate_form(frame, chunks[1], app),
        _ => render_candidates_table(frame, chunks[1], app),
    }
}

fn render_candidates_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["#", "Nombre del Candidato", "Fase del Proceso", "Vacante Postulada"]
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

        let opening_title = app
            .openings
            .iter()
            .find(|o| o.id == cand.job_opening_id)
            .map(|o| o.title.clone())
            .unwrap_or_else(|| cand.job_opening_id.to_string()[..8].to_string());

        Row::new(vec![
            Cell::from(format!("{}", idx + 1)).style(style),
            Cell::from(cand.full_name.clone()).style(style),
            Cell::from(cand.stage.clone()).style(Style::default().fg(SULTANA_THEME.warning).add_modifier(Modifier::BOLD)),
            Cell::from(opening_title).style(style),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
            Constraint::Percentage(30),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Seguimiento de Postulantes — [↑/↓] Navegar, [D] Retirar Postulante, [P] Postular a Vacante, [V] Vacantes ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.primary)),
    );

    frame.render_widget(table, area);
}

fn render_openings_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["#", "Puesto / Vacante", "Área Destino", "Estado", "Acción de Reclutamiento"]
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

        let action_cell = if o.open {
            Cell::from("[P] Agregar Candidato").style(Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD))
        } else {
            Cell::from("(Vacante Cerrada)").style(Style::default().fg(SULTANA_THEME.text_dim))
        };

        Row::new(vec![
            Cell::from(format!("{}", idx + 1)).style(style),
            Cell::from(o.title.clone()).style(style),
            Cell::from(o.area.display_name()).style(style),
            Cell::from(Line::from(status_span)),
            action_cell,
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(14),
            Constraint::Percentage(22),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Vacantes Disponibles — [↑/↓] Elegir vacante, [P] Agregar Candidato a esta vacante, [C] Candidatos ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.primary)),
    );

    frame.render_widget(table, area);
}

fn render_new_candidate_form(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(14), Constraint::Min(2)])
        .split(area);

    let f1 = if app.active_field == 0 { "_" } else { "" };
    let f2 = if app.active_field == 1 { "_" } else { "" };
    let f3 = if app.active_field == 2 { "_" } else { "" };

    let vacancy_text = if let Some(op) = &app.selected_opening {
        format!("{} ({})", op.title, op.area.display_name())
    } else {
        "Ninguna vacante seleccionada".to_string()
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Vacante de Empleo:    ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled(vacancy_text, Style::default().fg(SULTANA_THEME.success).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
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
            Span::styled("  Resumen CV / Perfil:  ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_3, f3), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Tab] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Siguiente campo    ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled("[Enter] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Guardar y Postular Candidato    ", Style::default().fg(SULTANA_THEME.success)),
            Span::styled("[Esc] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Cancelar", Style::default().fg(SULTANA_THEME.danger)),
        ]),
    ];

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(" Formulario: Agregar Candidato a una Vacante ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.accent)),
    );
    frame.render_widget(p, chunks[0]);
}
