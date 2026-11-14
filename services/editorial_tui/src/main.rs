pub mod api;
pub mod app;
pub mod auth;
pub mod config;
pub mod domain;
pub mod ui;

use anyhow::Result;
use api::EditorialApi;
use app::{App, CurrentScreen, SubView};
use config::AppConfig;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use domain::*;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;
use ui::components::PopupKind;
use ui::render_app;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::from_env();
    let mut app = App::new(config);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app_loop(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error en la ejecución de la TUI: {:#}", err);
    }

    Ok(())
}

async fn run_app_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| render_app(f, app))?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key_press(app, key.code).await;
                }
            }
        }
    }
    Ok(())
}

async fn handle_key_press(app: &mut App, code: KeyCode) {
    if app.popup.is_some() {
        if matches!(code, KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ')) {
            app.clear_popup();
        }
        return;
    }

    match code {
        KeyCode::Char('q') | KeyCode::Char('Q')
            if app.current_screen == CurrentScreen::Login
                || app.current_screen == CurrentScreen::MainMenu =>
        {
            app.should_quit = true;
            return;
        }
        _ => {}
    }

    match app.current_screen {
        CurrentScreen::Login => handle_login_keys(app, code).await,
        CurrentScreen::MainMenu => handle_menu_keys(app, code).await,
        CurrentScreen::Attention => handle_attention_keys(app, code).await,
        CurrentScreen::Content => handle_content_keys(app, code).await,
        CurrentScreen::Hr => handle_hr_keys(app, code).await,
        CurrentScreen::Events => handle_events_keys(app, code).await,
    }
}

async fn handle_login_keys(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('1') if app.input_1.is_empty() => {
            login_quick(app, "atencion.agente").await;
        }
        KeyCode::Char('2') if app.input_1.is_empty() => {
            login_quick(app, "prod.editor").await;
        }
        KeyCode::Char('3') if app.input_1.is_empty() => {
            login_quick(app, "rrhh.gerente").await;
        }
        KeyCode::Char('4') if app.input_1.is_empty() => {
            login_quick(app, "recluta.especialista").await;
        }
        KeyCode::Tab => {
            app.active_field = (app.active_field + 1) % 2;
        }
        KeyCode::Char(c) => {
            if app.active_field == 0 {
                app.input_1.push(c);
            } else {
                app.input_2.push(c);
            }
        }
        KeyCode::Backspace => {
            if app.active_field == 0 {
                app.input_1.pop();
            } else {
                app.input_2.pop();
            }
        }
        KeyCode::Enter => {
            if !app.input_1.is_empty() {
                let u = app.input_1.clone();
                let p = app.input_2.clone();
                match app.api.login(&u, &p).await {
                    Ok(session) => {
                        app.session = Some(session);
                        app.current_screen = CurrentScreen::MainMenu;
                        app.clear_inputs();
                        app.set_popup(PopupKind::Success, "Autenticación exitosa. Bienvenido al sistema.".to_string());
                    }
                    Err(e) => app.handle_api_error(e),
                }
            } else {
                app.set_popup(PopupKind::Warning, "Ingrese el nombre de usuario o use [1-4].".to_string());
            }
        }
        _ => {}
    }
}

async fn login_quick(app: &mut App, username: &str) {
    match app.api.login(username, "secret").await {
        Ok(session) => {
            app.session = Some(session);
            app.current_screen = CurrentScreen::MainMenu;
            app.clear_inputs();
            app.set_popup(PopupKind::Success, format!("Sesión iniciada como perfil: {}", username));
        }
        Err(e) => app.handle_api_error(e),
    }
}

async fn handle_menu_keys(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            if app.menu_index > 0 {
                app.menu_index -= 1;
            } else {
                app.menu_index = 4;
            }
        }
        KeyCode::Down => {
            if app.menu_index < 4 {
                app.menu_index += 1;
            } else {
                app.menu_index = 0;
            }
        }
        KeyCode::Enter => match app.menu_index {
            0 => {
                if app.check_area_access(AreaCode::Attention) {
                    app.current_screen = CurrentScreen::Attention;
                    app.sub_view = SubView::List1;
                    app.list_index = 0;
                    app.refresh_data().await;
                }
            }
            1 => {
                if app.check_area_access(AreaCode::Production) {
                    app.current_screen = CurrentScreen::Content;
                    app.sub_view = SubView::List1;
                    app.list_index = 0;
                    app.refresh_data().await;
                }
            }
            2 => {
                let user_area = app.session.as_ref().map(|s| s.user.area);
                if user_area == Some(AreaCode::Hr) || user_area == Some(AreaCode::Recruitment) {
                    app.current_screen = CurrentScreen::Hr;
                    app.sub_view = SubView::List1;
                    app.list_index = 0;
                    app.refresh_data().await;
                } else {
                    app.check_area_access(AreaCode::Hr);
                }
            }
            3 => {
                let user_area = app.session.as_ref().map(|s| s.user.area);
                if user_area == Some(AreaCode::Marketing) || user_area == Some(AreaCode::Production) {
                    app.current_screen = CurrentScreen::Events;
                    app.sub_view = SubView::List1;
                    app.list_index = 0;
                    app.refresh_data().await;
                } else {
                    app.check_area_access(AreaCode::Marketing);
                }
            }
            4 => {
                app.logout();
            }
            _ => {}
        },
        KeyCode::Esc => {
            app.logout();
        }
        _ => {}
    }
}

async fn handle_attention_keys(app: &mut App, code: KeyCode) {
    match app.sub_view {
        SubView::List1 => match code {
            KeyCode::Up => {
                if app.list_index > 0 {
                    app.list_index -= 1;
                }
            }
            KeyCode::Down => {
                if !app.cases.is_empty() && app.list_index < app.cases.len() - 1 {
                    app.list_index += 1;
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                if !app.cases.is_empty() && app.list_index < app.cases.len() {
                    let case_id = app.cases[app.list_index].id;
                    let next_status = app.cases[app.list_index].status.next();
                    match app.api.update_case_status(case_id, next_status).await {
                        Ok(upd) => {
                            app.set_popup(
                                PopupKind::Success,
                                format!("Estado actualizado a: {}", upd.status.display_name()),
                            );
                            app.refresh_data().await;
                        }
                        Err(e) => app.handle_api_error(e),
                    }
                }
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                app.sub_view = SubView::List2;
                app.list_index = 0;
                app.refresh_data().await;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.sub_view = SubView::Form2;
                app.clear_inputs();
            }
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::MainMenu;
            }
            _ => {}
        },
        SubView::List2 => match code {
            KeyCode::Up => {
                if app.list_index > 0 {
                    app.list_index -= 1;
                }
            }
            KeyCode::Down => {
                if !app.clients.is_empty() && app.list_index < app.clients.len() - 1 {
                    app.list_index += 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                app.sub_view = SubView::Form1;
                app.clear_inputs();
            }
            KeyCode::Char('v') | KeyCode::Char('V') | KeyCode::Esc => {
                app.sub_view = SubView::List1;
                app.list_index = 0;
            }
            _ => {}
        },
        SubView::Form1 => match code {
            KeyCode::Tab => {
                app.active_field = (app.active_field + 1) % 3;
            }
            KeyCode::Char(c) => match app.active_field {
                0 => app.input_1.push(c),
                1 => app.input_2.push(c),
                _ => app.input_3.push(c),
            },
            KeyCode::Backspace => match app.active_field {
                0 => { app.input_1.pop(); }
                1 => { app.input_2.pop(); }
                _ => { app.input_3.pop(); }
            },
            KeyCode::Enter => {
                let input = CreateClientInput {
                    full_name: app.input_1.clone(),
                    email: if app.input_2.is_empty() { None } else { Some(app.input_2.clone()) },
                    phone: if app.input_3.is_empty() { None } else { Some(app.input_3.clone()) },
                };
                match app.api.create_client(input).await {
                    Ok(cl) => {
                        app.set_popup(PopupKind::Success, format!("Cliente '{}' registrado.", cl.full_name));
                        app.sub_view = SubView::List2;
                        app.clear_inputs();
                        app.refresh_data().await;
                    }
                    Err(e) => app.handle_api_error(e),
                }
            }
            KeyCode::Esc => {
                app.sub_view = SubView::List2;
                app.clear_inputs();
            }
            _ => {}
        },
        SubView::Form2 => match code {
            KeyCode::Tab => {
                app.active_field = (app.active_field + 1) % 2;
            }
            KeyCode::Char(c) => match app.active_field {
                0 => app.input_1.push(c),
                _ => app.input_2.push(c),
            },
            KeyCode::Backspace => match app.active_field {
                0 => { app.input_1.pop(); }
                _ => { app.input_2.pop(); }
            },
            KeyCode::Enter => {
                let client_id = if !app.clients.is_empty() {
                    app.clients[0].id
                } else {
                    uuid::Uuid::new_v4()
                };
                let input = CreateCaseInput {
                    client_id,
                    subject: app.input_1.clone(),
                    description: app.input_2.clone(),
                };
                match app.api.create_case(input).await {
                    Ok(cs) => {
                        app.set_popup(PopupKind::Success, format!("Caso '{}' creado exitosamente.", cs.subject));
                        app.sub_view = SubView::List1;
                        app.clear_inputs();
                        app.refresh_data().await;
                    }
                    Err(e) => app.handle_api_error(e),
                }
            }
            KeyCode::Esc => {
                app.sub_view = SubView::List1;
                app.clear_inputs();
            }
            _ => {}
        },
        _ => {}
    }
}

async fn handle_content_keys(app: &mut App, code: KeyCode) {
    match app.sub_view {
        SubView::List1 => match code {
            KeyCode::Up => {
                if app.list_index > 0 {
                    app.list_index -= 1;
                }
            }
            KeyCode::Down => {
                if !app.articles.is_empty() && app.list_index < app.articles.len() - 1 {
                    app.list_index += 1;
                }
            }
            KeyCode::Enter => {
                if !app.articles.is_empty() && app.list_index < app.articles.len() {
                    let article_id = app.articles[app.list_index].id;
                    match app.api.get_article(article_id).await {
                        Ok(art) => {
                            app.selected_article = Some(art);
                            app.sub_view = SubView::Detail;
                        }
                        Err(e) => app.handle_api_error(e),
                    }
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.sub_view = SubView::Form1;
                app.clear_inputs();
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                if !app.articles.is_empty() && app.list_index < app.articles.len() {
                    let article_id = app.articles[app.list_index].id;
                    match app.api.update_article_status(article_id, ArticleStatus::Review).await {
                        Ok(_) => {
                            app.set_popup(PopupKind::Success, "Artículo enviado a Revisión Editorial.".to_string());
                            app.refresh_data().await;
                        }
                        Err(e) => app.handle_api_error(e),
                    }
                }
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                if !app.articles.is_empty() && app.list_index < app.articles.len() {
                    let article_id = app.articles[app.list_index].id;
                    match app.api.update_article_status(article_id, ArticleStatus::Published).await {
                        Ok(_) => {
                            app.set_popup(PopupKind::Success, "Artículo publicado exitosamente al Noticiero web.".to_string());
                            app.refresh_data().await;
                        }
                        Err(e) => app.handle_api_error(e),
                    }
                }
            }
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::MainMenu;
            }
            _ => {}
        },
        SubView::Detail => {
            if matches!(code, KeyCode::Esc | KeyCode::Enter) {
                app.sub_view = SubView::List1;
            }
        }
        SubView::Form1 => match code {
            KeyCode::Tab => {
                app.active_field = (app.active_field + 1) % 3;
            }
            KeyCode::Char(c) => match app.active_field {
                0 => app.input_1.push(c),
                1 => app.input_2.push(c),
                _ => app.input_3.push(c),
            },
            KeyCode::Backspace => match app.active_field {
                0 => { app.input_1.pop(); }
                1 => { app.input_2.pop(); }
                _ => { app.input_3.pop(); }
            },
            KeyCode::Enter => {
                let input = CreateArticleInput {
                    title: app.input_1.clone(),
                    summary: if app.input_2.is_empty() { None } else { Some(app.input_2.clone()) },
                    body: app.input_3.clone(),
                };
                match app.api.create_article(input).await {
                    Ok(art) => {
                        app.set_popup(PopupKind::Success, format!("Artículo '{}' redactado en Borrador.", art.title));
                        app.sub_view = SubView::List1;
                        app.clear_inputs();
                        app.refresh_data().await;
                    }
                    Err(e) => app.handle_api_error(e),
                }
            }
            KeyCode::Esc => {
                app.sub_view = SubView::List1;
                app.clear_inputs();
            }
            _ => {}
        },
        _ => {}
    }
}

async fn handle_hr_keys(app: &mut App, code: KeyCode) {
    match app.sub_view {
        SubView::List1 => match code {
            KeyCode::Up => {
                if app.list_index > 0 { app.list_index -= 1; }
            }
            KeyCode::Down => {
                if !app.employees.is_empty() && app.list_index < app.employees.len() - 1 {
                    app.list_index += 1;
                }
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                app.sub_view = SubView::List2;
                app.list_index = 0;
                app.refresh_data().await;
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                app.sub_view = SubView::List3;
                app.list_index = 0;
                app.refresh_data().await;
            }
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::MainMenu;
            }
            _ => {}
        },
        SubView::List2 => match code {
            KeyCode::Up => {
                if app.list_index > 0 { app.list_index -= 1; }
            }
            KeyCode::Down => {
                if !app.openings.is_empty() && app.list_index < app.openings.len() - 1 {
                    app.list_index += 1;
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.sub_view = SubView::Form1;
                app.clear_inputs();
            }
            KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Esc => {
                app.sub_view = SubView::List1;
                app.list_index = 0;
            }
            _ => {}
        },
        SubView::List3 => match code {
            KeyCode::Up => {
                if app.list_index > 0 { app.list_index -= 1; }
            }
            KeyCode::Down => {
                if !app.candidates.is_empty() && app.list_index < app.candidates.len() - 1 {
                    app.list_index += 1;
                }
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                app.sub_view = SubView::Form2;
                app.clear_inputs();
            }
            KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Esc => {
                app.sub_view = SubView::List1;
                app.list_index = 0;
            }
            _ => {}
        },
        SubView::Form1 => match code {
            KeyCode::Tab => {
                app.active_field = (app.active_field + 1) % 2;
            }
            KeyCode::Char(c) => match app.active_field {
                0 => app.input_1.push(c),
                _ => app.input_2.push(c),
            },
            KeyCode::Backspace => match app.active_field {
                0 => { app.input_1.pop(); }
                _ => { app.input_2.pop(); }
            },
            KeyCode::Enter => {
                let input = CreateJobOpeningInput {
                    title: app.input_1.clone(),
                    area: AreaCode::Production,
                    description: app.input_2.clone(),
                };
                match app.api.create_job_opening(input).await {
                    Ok(jo) => {
                        app.set_popup(PopupKind::Success, format!("Vacante '{}' publicada.", jo.title));
                        app.sub_view = SubView::List2;
                        app.clear_inputs();
                        app.refresh_data().await;
                    }
                    Err(e) => app.handle_api_error(e),
                }
            }
            KeyCode::Esc => {
                app.sub_view = SubView::List2;
                app.clear_inputs();
            }
            _ => {}
        },
        SubView::Form2 => match code {
            KeyCode::Tab => {
                app.active_field = (app.active_field + 1) % 2;
            }
            KeyCode::Char(c) => match app.active_field {
                0 => app.input_1.push(c),
                _ => app.input_2.push(c),
            },
            KeyCode::Backspace => match app.active_field {
                0 => { app.input_1.pop(); }
                _ => { app.input_2.pop(); }
            },
            KeyCode::Enter => {
                let opening_id = if !app.openings.is_empty() {
                    app.openings[0].id
                } else {
                    uuid::Uuid::new_v4()
                };
                let input = CreateCandidateInput {
                    job_opening_id: opening_id,
                    full_name: app.input_1.clone(),
                    email: app.input_2.clone(),
                    resume_summary: None,
                };
                match app.api.create_candidate(input).await {
                    Ok(cand) => {
                        app.set_popup(PopupKind::Success, format!("Candidato '{}' registrado.", cand.full_name));
                        app.sub_view = SubView::List3;
                        app.clear_inputs();
                        app.refresh_data().await;
                    }
                    Err(e) => app.handle_api_error(e),
                }
            }
            KeyCode::Esc => {
                app.sub_view = SubView::List3;
                app.clear_inputs();
            }
            _ => {}
        },
        _ => {}
    }
}

async fn handle_events_keys(app: &mut App, code: KeyCode) {
    match app.sub_view {
        SubView::List1 => match code {
            KeyCode::Up => {
                if app.list_index > 0 { app.list_index -= 1; }
            }
            KeyCode::Down => {
                if !app.events.is_empty() && app.list_index < app.events.len() - 1 {
                    app.list_index += 1;
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.sub_view = SubView::Form1;
                app.clear_inputs();
            }
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::MainMenu;
            }
            _ => {}
        },
        SubView::Form1 => match code {
            KeyCode::Tab => {
                app.active_field = (app.active_field + 1) % 3;
            }
            KeyCode::Char(c) => match app.active_field {
                0 => app.input_1.push(c),
                1 => app.input_2.push(c),
                _ => app.input_3.push(c),
            },
            KeyCode::Backspace => match app.active_field {
                0 => { app.input_1.pop(); }
                1 => { app.input_2.pop(); }
                _ => { app.input_3.pop(); }
            },
            KeyCode::Enter => {
                let starts = chrono::Utc::now() + chrono::Duration::days(5);
                let ends = Some(starts + chrono::Duration::days(2));
                let input = CreateEventInput {
                    name: app.input_1.clone(),
                    location: if app.input_2.is_empty() { None } else { Some(app.input_2.clone()) },
                    description: if app.input_3.is_empty() { None } else { Some(app.input_3.clone()) },
                    starts_at: starts,
                    ends_at: ends,
                };
                match app.api.create_event(input).await {
                    Ok(ev) => {
                        app.set_popup(PopupKind::Success, format!("Evento '{}' programado exitosamente.", ev.name));
                        app.sub_view = SubView::List1;
                        app.clear_inputs();
                        app.refresh_data().await;
                    }
                    Err(e) => app.handle_api_error(e),
                }
            }
            KeyCode::Esc => {
                app.sub_view = SubView::List1;
                app.clear_inputs();
            }
            _ => {}
        },
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::MockApiClient;
    use validator::Validate;

    #[tokio::test]
    async fn test_mock_api_login_and_areas() {
        let api = MockApiClient::new();
        let session = api.login("prod.editor", "secret").await.unwrap();
        assert_eq!(session.user.area, AreaCode::Production);
        assert!(session.token.is_some());
    }

    #[tokio::test]
    async fn test_validation_rejects_invalid_inputs() {
        let invalid_client = CreateClientInput {
            full_name: "AB".to_string(),
            email: Some("correo_invalido_sin_arroba".to_string()),
            phone: None,
        };
        assert!(invalid_client.validate().is_err());

        let valid_client = CreateClientInput {
            full_name: "Librería Bolívar C.A.".to_string(),
            email: Some("ventas@libreriabolivar.com".to_string()),
            phone: Some("+58-285-6321144".to_string()),
        };
        assert!(valid_client.validate().is_ok());
    }

    #[tokio::test]
    async fn test_case_status_transition() {
        let api = MockApiClient::new();
        let cases = api.list_cases(0, 10).await.unwrap();
        assert!(!cases.content.is_empty());
        let first_case = &cases.content[0];
        let updated = api.update_case_status(first_case.id, CaseStatus::Closed).await.unwrap();
        assert_eq!(updated.status, CaseStatus::Closed);
    }

    #[tokio::test]
    async fn test_event_date_validation() {
        let api = MockApiClient::new();
        let now = chrono::Utc::now();
        let invalid_event = CreateEventInput {
            name: "Feria del Libro".to_string(),
            description: None,
            location: Some("Maracaibo".to_string()),
            starts_at: now,
            ends_at: Some(now - chrono::Duration::hours(2)),
        };
        let res = api.create_event(invalid_event).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_app_rbac_enforcement() {
        let config = AppConfig {
            core_api_url: "http://localhost:8080".to_string(),
            terminal_area: None,
            mock_mode: true,
        };
        let mut app = App::new(config);
        login_quick(&mut app, "atencion.agente").await;
        assert_eq!(app.current_screen, CurrentScreen::MainMenu);

        let allowed = app.check_area_access(AreaCode::Attention);
        assert!(allowed);

        let denied = app.check_area_access(AreaCode::Production);
        assert!(!denied);
        assert!(app.popup.is_some());
        let (kind, msg) = app.popup.as_ref().unwrap();
        assert_eq!(*kind, PopupKind::Error);
        assert!(msg.contains("ACCESO DENEGADO (403)"));
    }

    #[tokio::test]
    async fn test_app_login_and_logout() {
        let config = AppConfig {
            core_api_url: "http://localhost:8080".to_string(),
            terminal_area: None,
            mock_mode: true,
        };
        let mut app = App::new(config);
        assert!(app.session.is_none());

        login_quick(&mut app, "prod.editor").await;
        assert!(app.session.is_some());
        assert_eq!(app.session.as_ref().unwrap().user.area, AreaCode::Production);

        app.logout();
        assert!(app.session.is_none());
        assert_eq!(app.current_screen, CurrentScreen::Login);
    }

    #[tokio::test]
    async fn test_app_article_workflow() {
        let config = AppConfig {
            core_api_url: "http://localhost:8080".to_string(),
            terminal_area: None,
            mock_mode: true,
        };
        let mut app = App::new(config);
        login_quick(&mut app, "prod.editor").await;
        app.current_screen = CurrentScreen::Content;
        app.refresh_data().await;
        let initial_count = app.articles.len();

        let new_art = app.api.create_article(CreateArticleInput {
            title: "Nueva edición especial 2026".to_string(),
            summary: Some("Lanzamiento editorial lacustre".to_string()),
            body: "Cuerpo detallado de la nueva edición especial para circulación regional.".to_string(),
        }).await.unwrap();

        assert_eq!(new_art.status, ArticleStatus::Draft);
        let reviewed = app.api.update_article_status(new_art.id, ArticleStatus::Review).await.unwrap();
        assert_eq!(reviewed.status, ArticleStatus::Review);

        let published = app.api.update_article_status(new_art.id, ArticleStatus::Published).await.unwrap();
        assert_eq!(published.status, ArticleStatus::Published);
        assert!(published.published_at.is_some());

        app.refresh_data().await;
        assert_eq!(app.articles.len(), initial_count + 1);
    }
}
