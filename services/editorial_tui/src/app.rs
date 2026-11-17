use crate::api::{ApiBackend, ApiError, EditorialApi, HttpApiClient};
use crate::auth::Session;
use crate::config::AppConfig;
use crate::domain::*;
use crate::ui::components::PopupKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    Login,
    MainMenu,
    Attention,
    Content,
    Recruitment,
    Hr,
    Events,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubView {
    List1,
    List2,
    List3,
    Form1,
    Form2,
    Detail,
}

pub struct App {
    pub config: AppConfig,
    pub api: ApiBackend,
    pub session: Option<Session>,
    pub current_screen: CurrentScreen,
    pub sub_view: SubView,
    pub popup: Option<(PopupKind, String)>,
    pub should_quit: bool,

    pub menu_index: usize,
    pub list_index: usize,
    pub active_field: usize,

    pub input_1: String,
    pub input_2: String,
    pub input_3: String,
    pub input_4: String,

    pub clients: Vec<Client>,
    pub cases: Vec<SupportCase>,
    pub articles: Vec<ArticleSummary>,
    pub selected_article: Option<Article>,
    pub selected_opening: Option<JobOpening>,
    pub employees: Vec<Employee>,
    pub openings: Vec<JobOpening>,
    pub candidates: Vec<Candidate>,
    pub events: Vec<Event>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let api = ApiBackend::Http(HttpApiClient::new(config.core_api_url.clone()));

        Self {
            config,
            api,
            session: None,
            current_screen: CurrentScreen::Login,
            sub_view: SubView::List1,
            popup: None,
            should_quit: false,

            menu_index: 0,
            list_index: 0,
            active_field: 0,

            input_1: String::new(),
            input_2: String::new(),
            input_3: String::new(),
            input_4: String::new(),

            clients: Vec::new(),
            cases: Vec::new(),
            articles: Vec::new(),
            selected_article: None,
            selected_opening: None,
            employees: Vec::new(),
            openings: Vec::new(),
            candidates: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn set_popup(&mut self, kind: PopupKind, message: String) {
        self.popup = Some((kind, message));
    }

    pub fn clear_popup(&mut self) {
        self.popup = None;
    }

    pub fn clear_inputs(&mut self) {
        self.input_1.clear();
        self.input_2.clear();
        self.input_3.clear();
        self.input_4.clear();
        self.active_field = 0;
    }

    pub fn logout(&mut self) {
        self.session = None;
        self.current_screen = CurrentScreen::Login;
        self.sub_view = SubView::List1;
        self.clear_inputs();
        self.set_popup(
            PopupKind::Info,
            "Sesión cerrada de forma segura. Credenciales eliminadas de memoria.".to_string(),
        );
    }

    pub async fn refresh_data(&mut self) {
        match self.current_screen {
            CurrentScreen::Attention => {
                if let Ok(c) = self.api.list_clients(0, 50).await {
                    self.clients = c.content;
                }
                if let Ok(cases) = self.api.list_cases(0, 50).await {
                    self.cases = cases.content;
                }
            }
            CurrentScreen::Content => {
                if let Ok(a) = self.api.list_articles(0, 50).await {
                    self.articles = a.content;
                }
            }
            CurrentScreen::Recruitment => {
                if let Ok(cand) = self.api.list_candidates(0, 50).await {
                    self.candidates = cand.content;
                }
                if let Ok(o) = self.api.list_job_openings(0, 50).await {
                    self.openings = o.content;
                }
            }
            CurrentScreen::Hr => {
                if let Ok(e) = self.api.list_employees(0, 50).await {
                    self.employees = e.content;
                }
                if let Ok(o) = self.api.list_job_openings(0, 50).await {
                    self.openings = o.content;
                }
                if let Ok(cand) = self.api.list_candidates(0, 50).await {
                    self.candidates = cand.content;
                }
            }
            CurrentScreen::Events => {
                if let Ok(ev) = self.api.list_events(0, 50).await {
                    self.events = ev.content;
                }
            }
            _ => {}
        }
    }

    pub fn check_area_access(&mut self, target_area: AreaCode) -> bool {
        if let Some(session) = &self.session {
            if session.user.area == target_area {
                true
            } else {
                self.set_popup(
                    PopupKind::Error,
                    format!(
                        "ACCESO DENEGADO (403): Su usuario pertenece al área {:?}. No tiene permisos para acceder a {:?}.",
                        session.user.area.display_name(),
                        target_area.display_name()
                    ),
                );
                false
            }
        } else {
            self.set_popup(
                PopupKind::Error,
                "Debe iniciar sesión primero (401).".to_string(),
            );
            false
        }
    }

    pub fn handle_api_error(&mut self, err: ApiError) {
        match err {
            ApiError::Unauthorized => {
                self.set_popup(
                    PopupKind::Error,
                    "Sesión expirada o no autorizada (401).".to_string(),
                );
                self.current_screen = CurrentScreen::Login;
            }
            ApiError::Forbidden => {
                self.set_popup(
                    PopupKind::Error,
                    "Permiso denegado (403): Su rol no permite esta operación.".to_string(),
                );
            }
            ApiError::NotFound => {
                self.set_popup(
                    PopupKind::Warning,
                    "El recurso solicitado no fue encontrado (404).".to_string(),
                );
            }
            ApiError::NotImplemented(msg) => {
                self.set_popup(PopupKind::Info, format!("Fase 1: {}", msg));
            }
            ApiError::Validation(msg) => {
                self.set_popup(PopupKind::Warning, format!("Error de validación: {}", msg));
            }
            ApiError::Network(msg) => {
                self.set_popup(PopupKind::Error, format!("Error de conexión: {}", msg));
            }
            ApiError::Serialization(msg) => {
                self.set_popup(
                    PopupKind::Error,
                    format!("Error de formato de datos: {}", msg),
                );
            }
        }
    }
}
