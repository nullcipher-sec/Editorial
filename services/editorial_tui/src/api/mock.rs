use super::{ApiError, EditorialApi};
use crate::auth::Session;
use crate::domain::*;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use validator::Validate;

struct MockState {
    users: Vec<AppUser>,
    clients: Vec<Client>,
    cases: Vec<SupportCase>,
    articles: Vec<Article>,
    employees: Vec<Employee>,
    openings: Vec<JobOpening>,
    candidates: Vec<Candidate>,
    events: Vec<Event>,
}

#[derive(Clone)]
pub struct MockApiClient {
    state: Arc<Mutex<MockState>>,
}

impl Default for MockApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockApiClient {
    pub fn new() -> Self {
        let editor_id = Uuid::parse_str("11111111-1111-1111-1111-111111111101").unwrap();
        let attention_id = Uuid::new_v4();
        let hr_id = Uuid::new_v4();
        let recruit_id = Uuid::new_v4();

        let users = vec![
            AppUser {
                id: editor_id,
                ldap_uid: "prod.editor".to_string(),
                display_name: "Editor de Producción".to_string(),
                area: AreaCode::Production,
            },
            AppUser {
                id: attention_id,
                ldap_uid: "atencion.agente".to_string(),
                display_name: "Agente de Atención".to_string(),
                area: AreaCode::Attention,
            },
            AppUser {
                id: hr_id,
                ldap_uid: "rrhh.gerente".to_string(),
                display_name: "Gerente de RRHH".to_string(),
                area: AreaCode::Hr,
            },
            AppUser {
                id: recruit_id,
                ldap_uid: "recluta.especialista".to_string(),
                display_name: "Especialista de Reclutamiento".to_string(),
                area: AreaCode::Recruitment,
            },
        ];

        let client1_id = Uuid::new_v4();
        let client2_id = Uuid::new_v4();
        let clients = vec![
            Client {
                id: client1_id,
                full_name: "Librería del Lago C.A.".to_string(),
                email: Some("contacto@librerialago.com".to_string()),
                phone: Some("+58-261-7001122".to_string()),
            },
            Client {
                id: client2_id,
                full_name: "Distribuidora Zuliana de Libros".to_string(),
                email: Some("ventas@distrizulia.com".to_string()),
                phone: Some("+58-261-7003344".to_string()),
            },
        ];

        let cases = vec![
            SupportCase {
                id: Uuid::new_v4(),
                client_id: client1_id,
                subject: "Retraso en entrega de tiraje de noviembre".to_string(),
                description: Some("El pedido de 500 ejemplares no ha llegado a la sede central.".to_string()),
                status: CaseStatus::Open,
                assigned_to: Some(attention_id),
            },
            SupportCase {
                id: Uuid::new_v4(),
                client_id: client2_id,
                subject: "Solicitud de reimpresión catálogo 2026".to_string(),
                description: Some("Se requiere presupuesto para reimpresión de 200 catálogos.".to_string()),
                status: CaseStatus::InProgress,
                assigned_to: Some(attention_id),
            },
        ];

        let article1_id = Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1").unwrap();
        let article2_id = Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2").unwrap();
        let article3_id = Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbb3").unwrap();

        let articles = vec![
            Article {
                id: article1_id,
                title: "Sultana del Lago inaugura nueva sala de prensa".to_string(),
                summary: Some("La editorial estrena instalaciones para cubrir la región lacustre.".to_string()),
                body: "Sultana del Lago Editores presentó hoy su nueva sala de prensa con equipos de transmisión y estudios de audio.".to_string(),
                status: ArticleStatus::Published,
                author_id: Some(editor_id),
                published_at: Some(Utc::now()),
            },
            Article {
                id: article2_id,
                title: "Convocatoria abierta: reporteros de campo".to_string(),
                summary: Some("Buscamos talento para refuerzo en la cuenca del lago.".to_string()),
                body: "El área de Reclutamiento abrió vacantes para reporteros con experiencia en cobertura in situ.".to_string(),
                status: ArticleStatus::Published,
                author_id: Some(editor_id),
                published_at: Some(Utc::now()),
            },
            Article {
                id: article3_id,
                title: "Borrador interno — Análisis de ventas Q3".to_string(),
                summary: Some("Informe confidencial de ventas y distribución regional.".to_string()),
                body: "Contenido confidencial en revisión editorial de finanzas y producción.".to_string(),
                status: ArticleStatus::Draft,
                author_id: Some(editor_id),
                published_at: None,
            },
        ];

        let employees = vec![
            Employee {
                id: Uuid::new_v4(),
                employee_code: "EMP-001".to_string(),
                full_name: "Carlos Mendoza".to_string(),
                email: "carlos.mendoza@editorial.local".to_string(),
                department: AreaCode::Production,
            },
            Employee {
                id: Uuid::new_v4(),
                employee_code: "EMP-002".to_string(),
                full_name: "Elena Rincón".to_string(),
                email: "elena.rincon@editorial.local".to_string(),
                department: AreaCode::Attention,
            },
            Employee {
                id: Uuid::new_v4(),
                employee_code: "EMP-003".to_string(),
                full_name: "Marcos Urdaneta".to_string(),
                email: "marcos.urdaneta@editorial.local".to_string(),
                department: AreaCode::Hr,
            },
        ];

        let opening1_id = Uuid::new_v4();
        let openings = vec![
            JobOpening {
                id: opening1_id,
                title: "Corrector de Estilo Senior".to_string(),
                area: AreaCode::Production,
                open: true,
            },
            JobOpening {
                id: Uuid::new_v4(),
                title: "Analista de Atención al Cliente".to_string(),
                area: AreaCode::Attention,
                open: true,
            },
        ];

        let candidates = vec![
            Candidate {
                id: Uuid::new_v4(),
                job_opening_id: opening1_id,
                full_name: "Lucía Fernández".to_string(),
                stage: "APPLIED".to_string(),
            },
            Candidate {
                id: Uuid::new_v4(),
                job_opening_id: opening1_id,
                full_name: "Javier Soto".to_string(),
                stage: "INTERVIEW".to_string(),
            },
        ];

        let events = vec![
            Event {
                id: Uuid::new_v4(),
                name: "Feria Internacional del Libro de Maracaibo".to_string(),
                description: Some("Presentación de las novedades editoriales 2026.".to_string()),
                location: Some("Centro de Arte de Maracaibo Lía Bermúdez".to_string()),
                starts_at: Utc::now() + chrono::Duration::days(10),
                ends_at: Some(Utc::now() + chrono::Duration::days(14)),
            },
            Event {
                id: Uuid::new_v4(),
                name: "Taller de Poesía y Narrativa Breve".to_string(),
                description: Some("Encuentro literario con autores regionales.".to_string()),
                location: Some("Sala de Conferencias Sultana del Lago".to_string()),
                starts_at: Utc::now() + chrono::Duration::days(20),
                ends_at: Some(Utc::now() + chrono::Duration::days(21)),
            },
        ];

        Self {
            state: Arc::new(Mutex::new(MockState {
                users,
                clients,
                cases,
                articles,
                employees,
                openings,
                candidates,
                events,
            })),
        }
    }
}

impl EditorialApi for MockApiClient {
    async fn login(&self, username: &str, _password: &str) -> Result<Session, ApiError> {
        let lock = self.state.lock().await;
        let user = lock
            .users
            .iter()
            .find(|u| u.ldap_uid.eq_ignore_ascii_case(username))
            .cloned()
            .unwrap_or_else(|| AppUser {
                id: Uuid::new_v4(),
                ldap_uid: username.to_string(),
                display_name: format!("Usuario {}", username),
                area: AreaCode::Attention,
            });

        let fake_jwt = format!("simulated_jwt_token_for_{}", user.ldap_uid);
        Ok(Session::new(user, Some(fake_jwt)))
    }

    async fn list_clients(&self, page: u32, size: u32) -> Result<PageResponse<Client>, ApiError> {
        let lock = self.state.lock().await;
        let total = lock.clients.len() as u64;
        let start = (page * size) as usize;
        let content = if start >= lock.clients.len() {
            Vec::new()
        } else {
            lock.clients
                .iter()
                .skip(start)
                .take(size as usize)
                .cloned()
                .collect()
        };
        let total_pages = if total == 0 { 1 } else { (total as f64 / size as f64).ceil() as u32 };

        Ok(PageResponse {
            content,
            page,
            size,
            total_elements: total,
            total_pages,
        })
    }

    async fn create_client(&self, input: CreateClientInput) -> Result<Client, ApiError> {
        input.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
        let mut lock = self.state.lock().await;
        let client = Client {
            id: Uuid::new_v4(),
            full_name: input.full_name,
            email: input.email,
            phone: input.phone,
        };
        lock.clients.push(client.clone());
        Ok(client)
    }

    async fn list_cases(&self, page: u32, size: u32) -> Result<PageResponse<SupportCase>, ApiError> {
        let lock = self.state.lock().await;
        let total = lock.cases.len() as u64;
        let start = (page * size) as usize;
        let content = if start >= lock.cases.len() {
            Vec::new()
        } else {
            lock.cases
                .iter()
                .skip(start)
                .take(size as usize)
                .cloned()
                .collect()
        };
        let total_pages = if total == 0 { 1 } else { (total as f64 / size as f64).ceil() as u32 };

        Ok(PageResponse {
            content,
            page,
            size,
            total_elements: total,
            total_pages,
        })
    }

    async fn create_case(&self, input: CreateCaseInput) -> Result<SupportCase, ApiError> {
        input.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
        let mut lock = self.state.lock().await;
        let new_case = SupportCase {
            id: Uuid::new_v4(),
            client_id: input.client_id,
            subject: input.subject,
            description: Some(input.description),
            status: CaseStatus::Open,
            assigned_to: None,
        };
        lock.cases.push(new_case.clone());
        Ok(new_case)
    }

    async fn update_case_status(&self, id: Uuid, status: CaseStatus) -> Result<SupportCase, ApiError> {
        let mut lock = self.state.lock().await;
        if let Some(c) = lock.cases.iter_mut().find(|c| c.id == id) {
            c.status = status;
            Ok(c.clone())
        } else {
            Err(ApiError::NotFound)
        }
    }

    async fn list_articles(&self, page: u32, size: u32) -> Result<PageResponse<ArticleSummary>, ApiError> {
        let lock = self.state.lock().await;
        let total = lock.articles.len() as u64;
        let start = (page * size) as usize;
        let content = if start >= lock.articles.len() {
            Vec::new()
        } else {
            lock.articles
                .iter()
                .skip(start)
                .take(size as usize)
                .map(|a| ArticleSummary {
                    id: a.id,
                    title: a.title.clone(),
                    status: a.status,
                    published_at: a.published_at,
                })
                .collect()
        };
        let total_pages = if total == 0 { 1 } else { (total as f64 / size as f64).ceil() as u32 };

        Ok(PageResponse {
            content,
            page,
            size,
            total_elements: total,
            total_pages,
        })
    }

    async fn get_article(&self, id: Uuid) -> Result<Article, ApiError> {
        let lock = self.state.lock().await;
        lock.articles
            .iter()
            .find(|a| a.id == id)
            .cloned()
            .ok_or(ApiError::NotFound)
    }

    async fn create_article(&self, input: CreateArticleInput) -> Result<Article, ApiError> {
        input.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
        let mut lock = self.state.lock().await;
        let article = Article {
            id: Uuid::new_v4(),
            title: input.title,
            summary: input.summary,
            body: input.body,
            status: ArticleStatus::Draft,
            author_id: None,
            published_at: None,
        };
        lock.articles.push(article.clone());
        Ok(article)
    }

    async fn update_article_status(&self, id: Uuid, status: ArticleStatus) -> Result<Article, ApiError> {
        let mut lock = self.state.lock().await;
        if let Some(a) = lock.articles.iter_mut().find(|a| a.id == id) {
            a.status = status;
            if status == ArticleStatus::Published {
                a.published_at = Some(Utc::now());
            } else {
                a.published_at = None;
            }
            Ok(a.clone())
        } else {
            Err(ApiError::NotFound)
        }
    }

    async fn list_employees(&self, page: u32, size: u32) -> Result<PageResponse<Employee>, ApiError> {
        let lock = self.state.lock().await;
        let total = lock.employees.len() as u64;
        let start = (page * size) as usize;
        let content = if start >= lock.employees.len() {
            Vec::new()
        } else {
            lock.employees
                .iter()
                .skip(start)
                .take(size as usize)
                .cloned()
                .collect()
        };
        let total_pages = if total == 0 { 1 } else { (total as f64 / size as f64).ceil() as u32 };

        Ok(PageResponse {
            content,
            page,
            size,
            total_elements: total,
            total_pages,
        })
    }

    async fn list_job_openings(&self, page: u32, size: u32) -> Result<PageResponse<JobOpening>, ApiError> {
        let lock = self.state.lock().await;
        let total = lock.openings.len() as u64;
        let start = (page * size) as usize;
        let content = if start >= lock.openings.len() {
            Vec::new()
        } else {
            lock.openings
                .iter()
                .skip(start)
                .take(size as usize)
                .cloned()
                .collect()
        };
        let total_pages = if total == 0 { 1 } else { (total as f64 / size as f64).ceil() as u32 };

        Ok(PageResponse {
            content,
            page,
            size,
            total_elements: total,
            total_pages,
        })
    }

    async fn create_job_opening(&self, input: CreateJobOpeningInput) -> Result<JobOpening, ApiError> {
        input.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
        let mut lock = self.state.lock().await;
        let opening = JobOpening {
            id: Uuid::new_v4(),
            title: input.title,
            area: input.area,
            open: true,
        };
        lock.openings.push(opening.clone());
        Ok(opening)
    }

    async fn list_candidates(&self, page: u32, size: u32) -> Result<PageResponse<Candidate>, ApiError> {
        let lock = self.state.lock().await;
        let total = lock.candidates.len() as u64;
        let start = (page * size) as usize;
        let content = if start >= lock.candidates.len() {
            Vec::new()
        } else {
            lock.candidates
                .iter()
                .skip(start)
                .take(size as usize)
                .cloned()
                .collect()
        };
        let total_pages = if total == 0 { 1 } else { (total as f64 / size as f64).ceil() as u32 };

        Ok(PageResponse {
            content,
            page,
            size,
            total_elements: total,
            total_pages,
        })
    }

    async fn create_candidate(&self, input: CreateCandidateInput) -> Result<Candidate, ApiError> {
        input.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
        let mut lock = self.state.lock().await;
        let candidate = Candidate {
            id: Uuid::new_v4(),
            job_opening_id: input.job_opening_id,
            full_name: input.full_name,
            stage: "APPLIED".to_string(),
        };
        lock.candidates.push(candidate.clone());
        Ok(candidate)
    }

    async fn list_events(&self, page: u32, size: u32) -> Result<PageResponse<Event>, ApiError> {
        let lock = self.state.lock().await;
        let total = lock.events.len() as u64;
        let start = (page * size) as usize;
        let content = if start >= lock.events.len() {
            Vec::new()
        } else {
            lock.events
                .iter()
                .skip(start)
                .take(size as usize)
                .cloned()
                .collect()
        };
        let total_pages = if total == 0 { 1 } else { (total as f64 / size as f64).ceil() as u32 };

        Ok(PageResponse {
            content,
            page,
            size,
            total_elements: total,
            total_pages,
        })
    }

    async fn create_event(&self, input: CreateEventInput) -> Result<Event, ApiError> {
        input.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
        if let Some(ends) = input.ends_at {
            if ends <= input.starts_at {
                return Err(ApiError::Validation(
                    "La fecha de finalización debe ser posterior a la fecha de inicio".to_string(),
                ));
            }
        }
        let mut lock = self.state.lock().await;
        let event = Event {
            id: Uuid::new_v4(),
            name: input.name,
            description: input.description,
            location: input.location,
            starts_at: input.starts_at,
            ends_at: input.ends_at,
        };
        lock.events.push(event.clone());
        Ok(event)
    }
}
