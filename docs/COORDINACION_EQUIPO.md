# Coordinación fase 1 — Core API y base de datos

Documento para @arrias24 (TUI), @JhonmarMorales17 (noticiero) y @luiscab19 (infra).

## URLs base

| Entorno | Base URL |
|---|---|
| Desarrollo local | `http://localhost:8080` |
| Topología Docker (fase 1) | `http://10.20.30.10:8080` |
| DMZ → Apps (futuro) | El noticiero llama al Core vía red de aplicaciones, no a Postgres |

## Contrato HTTP

- **Canónico:** [`docs/api/openapi.yaml`](api/openapi.yaml)
- **Interactivo (solo `dev`, Scalar):** `http://localhost:8080/scalar` · JSON: `/v3/api-docs`

### Reglas fase 1

1. Las rutas y los JSON de respuesta **no cambiarán de nombre** sin aviso en el issue #1.
2. `GET` internos devuelven listas paginadas (`page`, `size`, `totalElements`…).
3. `POST`/`PATCH` internos responden **501** hasta integrar LDAP + JWT + `@PreAuthorize`.
4. La API pública **nunca** expone artículos con `status != PUBLISHED`.

### Autenticación (próxima fase)

| Header | Valor |
|---|---|
| `Authorization` | `Bearer <jwt>` (pendiente) |

Roles por área: `ATTENTION`, `PRODUCTION`, `RECRUITMENT`, `HR`, `MARKETING`.

## Para el noticiero (@JhonmarMorales17)

```http
GET /api/public/articles?page=0&size=20
GET /api/public/articles/{uuid}
```

Artículos de muestra en `db.sql` (IDs fijos en desarrollo). El borrador `bbbbbbbb-…` no debe aparecer en la API pública.

## Para las TUI (@arrias24)

Prefijo común: `/api/internal/...`

| Área | Rutas |
|---|---|
| Atención | `/attention/clients`, `/attention/cases` |
| Contenido | `/content/articles`, `PATCH …/publish` (501) |
| RRHH | `/hr/employees`, `/hr/job-openings` |
| Reclutamiento | `/recruitment/candidates` |
| Eventos | `/events` |
| Marketing | `/marketing/campaigns` |

Pueden implementar clientes HTTP contra `GET` ya; mutaciones cuando se anuncie auth en el issue.

## Para infra (@luiscab19)

| Servicio | Red | IP sugerida | Puertos |
|---|---|---|---|
| PostgreSQL | VLAN 40 `10.20.40.0/24` | `10.20.40.10` | 5432 (solo desde Apps) |
| editorial_core | VLAN 30 `10.20.30.0/24` | `10.20.30.10` | 8080 |

Variables: ver `deploy/.env.example`. Init SQL: montar `deploy/db_init/db.sql` en Postgres.

Compose de referencia local: `deploy/docker-compose.dev.yml` (no reemplaza el compose definitivo).

## Diagramas

- ER: `docs/diagrams/er-model.drawio` (+ `er-model.png`)
