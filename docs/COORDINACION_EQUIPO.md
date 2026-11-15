# Coordinación — Core API, auth y base de datos

Documento para @arrias24 (TUI), @JhonmarMorales17 (noticiero) y @luiscab19 (infra).

## URLs base

| Entorno | Base URL |
|---|---|
| Desarrollo local | `http://localhost:8080` |
| Topología Docker (fase 1) | `http://10.20.30.10:8080` |
| DMZ → Apps (futuro) | El noticiero llama al Core vía red de aplicaciones, no a Postgres |

## Contrato HTTP

- **Canónico:** [`docs/api/openapi.yaml`](api/openapi.yaml)
- **Interactivo (solo `dev`, Scalar con marca Sultana del Lago):** `http://localhost:8080/scalar` · JSON: `/v3/api-docs`

### Autenticación (fase 2 — activa en `dev`)

```http
POST /api/auth/login
Content-Type: application/json

{"username":"<ldap_uid>","password":"<contraseña>"}
```

Respuesta: `Session` con `user` (`ldap_uid`, `display_name`, `area`) y `token.value` (JWT).

| Header en rutas internas | Valor |
|---|---|
| `Authorization` | `Bearer <jwt>` |

Roles por área (Spring): `ROLE_ATTENTION`, `ROLE_PRODUCTION`, `ROLE_RECRUITMENT`, `ROLE_HR`, `ROLE_MARKETING`.

**Credenciales de desarrollo (LDAP + TUI quick-login):**

| `ldap_uid` | Área | Contraseña |
|---|---|---|
| `atencion.agente` | ATTENTION | `EditorialDev!2026` |
| `prod.editor` | PRODUCTION | `EditorialDev!2026` |
| `rrhh.gerente` | HR | `EditorialDev!2026` |
| `recluta.especialista` | RECRUITMENT | `EditorialDev!2026` |
| `marketing.coord` | MARKETING | `EditorialDev!2026` |

Usuarios definidos en `deploy/ldap/bootstrap.ldif` y filas en `app_user` (`deploy/db_init/db.sql`). Si Postgres ya existía antes de actualizar el seed, recrear volumen: `docker compose -f deploy/docker-compose.dev.yml down -v`.

### Reglas actuales

1. Las rutas y los JSON de respuesta **no cambiarán de nombre** sin aviso en el issue #1.
2. `GET` internos devuelven listas paginadas (`page`, `size`, `totalElements`…).
3. `POST`/`PATCH` internos están protegidos por JWT + `@PreAuthorize` por área (excepto stubs pendientes, p. ej. `POST /api/internal/hr/employees`).
4. La API pública **nunca** expone artículos con `status != PUBLISHED`.

## Para el noticiero (@JhonmarMorales17)

```http
GET /api/public/articles?page=0&size=20
GET /api/public/articles/{uuid}
```

Artículos de muestra en `db.sql` (IDs fijos en desarrollo). El borrador `bbbbbbbb-…` no debe aparecer en la API pública.

## Para las TUI (@arrias24)

Prefijo común: `/api/internal/...` (Bearer obligatorio salvo login).

| Área | Rutas |
|---|---|
| Atención | `/attention/clients`, `/attention/cases` |
| Contenido | `/content/articles`, `PATCH …/publish` |
| RRHH | `/hr/employees`, `/hr/job-openings` |
| Reclutamiento | `/recruitment/candidates` |
| Eventos | `/events` |
| Marketing | `/marketing/campaigns` |

`MOCK_MODE=false` y `CORE_API_URL=http://editorial_core:8080` en `docker-compose.dev.yml`.

## Para infra (@luiscab19)

| Servicio | Red | IP sugerida | Puertos |
|---|---|---|---|
| PostgreSQL | VLAN 40 `10.20.40.0/24` | `10.20.40.10` | 5432 (solo desde Apps) |
| editorial_core | VLAN 30 `10.20.30.0/24` | `10.20.30.10` | 8080 |
| LDAP | VLAN 30 | `10.20.30.20` | 389 |

Variables: ver `deploy/.env.example`. Init SQL: montar `deploy/db_init/db.sql` en Postgres.

LDAP en dev: imagen `deploy/ldap/Dockerfile` (bootstrap incluido; evita bind-mount roto en Docker Desktop). El Core usa `LDAP_MANAGER_DN` / `LDAP_MANAGER_PASSWORD` para validar usuarios (búsqueda anónima deshabilitada en OpenLDAP).

Compose de referencia local: `deploy/docker-compose.dev.yml` (no reemplaza el compose definitivo).

## Diagramas

- ER: `docs/diagrams/er-model.drawio` (+ `er-model.png`)