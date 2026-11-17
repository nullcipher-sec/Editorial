use crate::app::{App, SubView};
use crate::domain::ArticleStatus;
use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(area);

    let tab_text = match app.sub_view {
        SubView::List1 => match app.articles.get(app.list_index).map(|a| a.status) {
            Some(ArticleStatus::Draft) => " [Listado de Artículos]  |  [N] Nuevo  |  [R] Enviar a Revisión  |  [D] Eliminar",
            Some(ArticleStatus::Review) => " [Listado de Artículos]  |  [N] Nuevo  |  [P] Publicar  |  [D] Eliminar",
            Some(ArticleStatus::Published) => " [Listado de Artículos]  |  [N] Nuevo  |  [D] Eliminar",
            _ => " [Listado de Artículos]  |  [N] Nuevo",
        },
        SubView::Detail => " [Lectura de Artículo]  |  [Esc] Volver a la Lista",
        SubView::Form1 => " [FORMULARIO: Redactar Nuevo Artículo Editorial]",
        _ => " [Módulo de Contenido Editorial]",
    };

    let title_block = Paragraph::new(Line::from(vec![
        Span::styled(" PRODUCCIÓN EDITORIAL Y CONTENIDO ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
        Span::styled(tab_text, Style::default().fg(SULTANA_THEME.text_dim)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(SULTANA_THEME.border)));
    frame.render_widget(title_block, chunks[0]);

    match app.sub_view {
        SubView::List1 => render_articles_table(frame, chunks[1], app),
        SubView::Detail => render_article_detail(frame, chunks[1], app),
        SubView::Form1 => render_new_article_form(frame, chunks[1], app),
        _ => render_articles_table(frame, chunks[1], app),
    }
}

fn render_articles_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["#", "Título de la Noticia / Artículo", "Estado Editorial", "Publicado el"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.articles.iter().enumerate().map(|(idx, a)| {
        let is_selected = idx == app.list_index;
        let (status_color, status_text) = match a.status {
            ArticleStatus::Draft => (SULTANA_THEME.text_dim, "BORRADOR"),
            ArticleStatus::Review => (SULTANA_THEME.warning, "EN REVISIÓN"),
            ArticleStatus::Published => (SULTANA_THEME.success, "PUBLICADO"),
        };

        let style = if is_selected {
            Style::default().fg(SULTANA_THEME.selected_fg).bg(SULTANA_THEME.selected_bg)
        } else {
            Style::default().fg(SULTANA_THEME.text_bright)
        };

        let pub_date = a
            .published_at
            .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "No publicado".to_string());

        Row::new(vec![
            Cell::from(format!("{}", idx + 1)).style(style),
            Cell::from(a.title.clone()).style(style),
            Cell::from(status_text).style(Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Cell::from(pub_date).style(style),
        ])
    });

    let selected_status = app.articles.get(app.list_index).map(|a| a.status);
    let contextual_actions = match selected_status {
        Some(ArticleStatus::Draft) => "[Enter] Leer detalle  |  [R] Revisión  |  [D] Eliminar  |  [N] Nuevo",
        Some(ArticleStatus::Review) => "[Enter] Leer detalle  |  [P] Publicar  |  [D] Eliminar  |  [N] Nuevo",
        Some(ArticleStatus::Published) => "[Enter] Leer detalle  |  [D] Eliminar  |  [N] Nuevo",
        None => "[N] Redactar Nuevo Artículo",
    };

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Percentage(55),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(format!(" Artículos — {} ", contextual_actions))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.primary)),
    );

    frame.render_widget(table, area);
}

fn render_article_detail(frame: &mut Frame, area: Rect, app: &App) {
    if let Some(article) = &app.selected_article {
        let (status_color, status_text) = match article.status {
            ArticleStatus::Draft => (SULTANA_THEME.text_dim, "BORRADOR"),
            ArticleStatus::Review => (SULTANA_THEME.warning, "EN REVISIÓN"),
            ArticleStatus::Published => (SULTANA_THEME.success, "PUBLICADO"),
        };

        let lines = vec![
            Line::from(vec![
                Span::styled("Título: ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
                Span::styled(&article.title, Style::default().fg(SULTANA_THEME.text_bright).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Estado: ", Style::default().fg(SULTANA_THEME.text_dim)),
                Span::styled(status_text, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Resumen: ", Style::default().fg(SULTANA_THEME.text_dim)),
                Span::styled(article.summary.as_deref().unwrap_or("Sin resumen"), Style::default().fg(SULTANA_THEME.text_bright)),
            ]),
            Line::from(""),
            Line::from(Span::styled("--- CUERPO DEL ARTÍCULO ---", Style::default().fg(SULTANA_THEME.border))),
            Line::from(""),
            Line::from(Span::styled(&article.body, Style::default().fg(SULTANA_THEME.text_bright))),
            Line::from(""),
            Line::from(Span::styled("[Esc] Volver a la lista de artículos", Style::default().fg(SULTANA_THEME.accent))),
        ];

        let p = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" Detalle de Noticia Editorial ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(SULTANA_THEME.primary)),
            )
            .wrap(Wrap { trim: true });
        frame.render_widget(p, area);
    } else {
        let p = Paragraph::new("No hay artículo seleccionado.")
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(p, area);
    }
}

fn render_new_article_form(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(14), Constraint::Min(2)])
        .split(area);

    let f1 = if app.active_field == 0 { "_" } else { "" };
    let f2 = if app.active_field == 1 { "_" } else { "" };
    let f3 = if app.active_field == 2 { "_" } else { "" };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Título (mínimo 5 caracteres):        ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_1, f1), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Resumen Breve (opcional):            ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_2, f2), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Cuerpo Completo (mínimo 20 caracteres): ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(format!("{}{}", app.input_3, f3), Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Tab] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Siguiente campo    ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled("[Enter] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Guardar en Borrador    ", Style::default().fg(SULTANA_THEME.success)),
            Span::styled("[Esc] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Cancelar", Style::default().fg(SULTANA_THEME.danger)),
        ]),
    ];

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(" Redactar Nuevo Artículo (Validado con Validator) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SULTANA_THEME.accent)),
    );
    frame.render_widget(p, chunks[0]);
}
