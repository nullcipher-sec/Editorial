# editorial-core

Backend (Core) de Sultana del Lago Editores: API interna para las TUI y API pública para el noticiero.

## Alcance

- Esquema PostgreSQL en `deploy/db_init/db.sql`
- Contrato HTTP estable (`docs/api/openapi.yaml`, Scalar en perfil `dev`)
- `GET /api/public/articles` operativo (solo `PUBLISHED`)
- Login LDAP + JWT en `POST /api/auth/login`
- Rutas `/api/internal/**` protegidas por rol (`ROLE_<AREA>`): 401 sin token, 403 con rol equivocado

## Desarrollo local

```bash
cd deploy
cp .env.example .env   # ajustar contraseñas
docker compose -f docker-compose.dev.yml up --build
```

- Salud: `curl -s http://localhost:8080/actuator/health`
- Artículos públicos: `curl -s http://localhost:8080/api/public/articles`
- Scalar (solo `dev`): http://localhost:8080/scalar · OpenAPI: `/v3/api-docs`

Sin Docker:

```bash
# Postgres con db.sql cargado en localhost:5432
export POSTGRES_USER=editorial POSTGRES_PASSWORD=... POSTGRES_DB=editorial
export SPRING_PROFILES_ACTIVE=dev
./mvnw spring-boot:run
```

## Integración en topología

| Variable | Producción (referencia) |
|---|---|
| `POSTGRES_HOST` | `10.20.40.10` |
| `CORE_PORT` | `8080` (VLAN 30) |
| `SPRING_PROFILES_ACTIVE` | `prod` |

@luiscab19: el servicio expone **8080/tcp** en la red de aplicaciones y necesita salida TCP a Postgres en `10.20.40.10:5432` (solo el Core).
