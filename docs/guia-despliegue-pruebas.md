# Guía de despliegue y pruebas (entorno de desarrollo)

Guía paso a paso para levantar el Core, PostgreSQL, OpenLDAP (y opcionalmente las TUI) y verificar la seguridad de la API. Los comandos son para macOS/Linux (bash/zsh). Para PowerShell ver `docs/pruebas-integrales.md`.

> Esta composición (`deploy/docker-compose.dev.yml`) simula la separación lógica en redes Docker; no prueba VLAN 802.1Q, firewall ni TLS. Esos se verifican con `deploy/docker-compose.yml` (topología completa, @luiscab19).

## 1. Requisitos

- Docker Desktop (o Docker Engine) en ejecución y Docker Compose v2.
- Puertos libres: `8080` (Core) y, si lo expones, `5432`.
- `curl` y `python3` (solo para extraer el token en los ejemplos).
- Opcional para tests unitarios: JDK 21 y Maven.

Comprueba:

```bash
docker info > /dev/null && docker compose version
```

## 2. Configuración (`deploy/.env`)

```bash
cd deploy
cp .env.dev.example .env
```

`.env.dev.example` trae valores de desarrollo ya consistentes con el compose (en especial `LDAP_ADMIN_PASSWORD=admin_dev_only`, que el healthcheck de LDAP usa fijo). `.env` está en `.gitignore`: nunca se sube.

> Si partes de `.env.example` (plantilla de producción) el LDAP quedará `unhealthy` por contraseña distinta. Usa `.env.dev.example` para pruebas.

## 3. Levantar el stack

Mínimo (Core + datos + LDAP):

```bash
docker compose -f docker-compose.dev.yml up -d --build postgres ldap editorial_core
docker compose -f docker-compose.dev.yml ps
```

Con las 5 TUI (build más largo):

```bash
docker compose -f docker-compose.dev.yml up -d --build
```

Espera a que `ldap` y `editorial_core` estén `healthy`:

```bash
until curl -sf http://localhost:8080/actuator/health; do sleep 5; done; echo
# {"status":"UP"}
```

Si algo falla: `docker compose -f docker-compose.dev.yml logs --tail 100 postgres ldap editorial_core`.

**Reinicio limpio** (datos y usuarios nuevos de `db.sql`, `seed.sql` o `bootstrap.ldif`; borra los volúmenes de desarrollo):

```bash
docker compose -f docker-compose.dev.yml down -v
docker compose -f docker-compose.dev.yml up -d --build postgres ldap editorial_core
```

## 4. Documentación interactiva

`http://localhost:8080/scalar` (solo perfil `dev`). Contrato: `docs/api/openapi.yaml`.

## 5. Usuarios de prueba (LDAP)

Contraseña para todos: `EditorialDev!2026`.

| Usuario | Área / rol |
|---|---|
| `atencion.agente` | Atención (`ROLE_ATTENTION`) |
| `prod.editor` | Producción (`ROLE_PRODUCTION`) |
| `rrhh.gerente` | RRHH (`ROLE_HR`) |
| `recluta.especialista` | Reclutamiento (`ROLE_RECRUITMENT`) |
| `marketing.coord` | Marketing (`ROLE_MARKETING`) |

### Crear un usuario nuevo

Solo se administra **LDAP**; el Core crea o actualiza el registro interno (`app_user`) en el primer login
a partir del `cn` (nombre) y del grupo del usuario (`gidNumber` → grupo en `ou=groups`).

Grupos/áreas: `ATTENTION` 2100, `PRODUCTION` 2200, `RECRUITMENT` 2300, `HR` 2400, `MARKETING` 2500.

```bash
cat > nuevo.ldif <<'EOF'
dn: uid=nuevo.usuario,ou=people,dc=editorial,dc=local
objectClass: inetOrgPerson
objectClass: posixAccount
uid: nuevo.usuario
cn: Nombre Apellido
sn: Apellido
uidNumber: 10100
gidNumber: 2500
homeDirectory: /home/nuevo.usuario
mail: nuevo.usuario@editorial.local
userPassword: MiClave!2026
EOF
docker compose cp nuevo.ldif ldap:/tmp/nuevo.ldif
docker compose exec -T ldap ldapadd -x -D "cn=admin,dc=editorial,dc=local" -w admin_dev_only -f /tmp/nuevo.ldif
```

Cada usuario necesita un `uidNumber` distinto (los de prueba usan 10001–10005). Para cambiarlo de área, modifica su
`gidNumber` en LDAP (`ldapmodify`); se aplica en su siguiente login. Sin grupo válido el login responde 403.
Para dejarlo como usuario de ejemplo permanente, añade el bloque a `deploy/ldap/bootstrap.ldif` y reinicia con `docker compose down -v`.

## 6. Pruebas manuales de la API

Define un ayudante para obtener tokens:

```bash
B=http://localhost:8080
token() { curl -s -X POST $B/api/auth/login -H 'Content-Type: application/json' \
  -d "{\"username\":\"$1\",\"password\":\"${2:-EditorialDev!2026}\"}" \
  | python3 -c "import sys,json;print(json.load(sys.stdin)['token']['value'])"; }
```

| # | Prueba | Comando | Esperado |
|---|---|---|---|
| 1 | Salud | `curl -s $B/actuator/health` | `{"status":"UP"}` |
| 2 | API pública sin token | `curl -s $B/api/public/articles` | 200, solo artículos `PUBLISHED` |
| 3 | Artículo no público por id | `curl -s -o /dev/null -w '%{http_code}' $B/api/public/articles/<id-de-un-DRAFT>` | 404 |
| 4 | Ruta interna sin token | `curl -s -o /dev/null -w '%{http_code}' $B/api/internal/hr/employees` | 401 |
| 5 | Login con contraseña incorrecta | `curl -s -o /dev/null -w '%{http_code}' -X POST $B/api/auth/login -H 'Content-Type: application/json' -d '{"username":"atencion.agente","password":"mala"}'` | 401 |
| 6 | Área correcta | `curl -s -H "Authorization: Bearer $(token atencion.agente)" $B/api/internal/attention/clients` | 200 |
| 7 | Otra área (403) | `curl -s -o /dev/null -w '%{http_code}' -H "Authorization: Bearer $(token atencion.agente)" $B/api/internal/hr/employees` | 403 |

### Alta de empleados (RRHH)

```bash
H=$(token rrhh.gerente)
curl -s -X POST $B/api/internal/hr/employees -H "Authorization: Bearer $H" -H 'Content-Type: application/json' \
  -d '{"employeeCode":"E-100","fullName":"Ana Pérez","email":"ana@editorial.local","department":"MARKETING","hiredAt":"2026-01-10"}'
```

| Caso | Esperado |
|---|---|
| Datos válidos | 200 con `id`, `employeeCode`, `fullName`, `department`, `active` |
| Mismo `employeeCode` otra vez | 409 `Resource already exists` |
| Email inválido, campo vacío o `department` inexistente | 400 `Invalid request` |
| Token de otra área | 403 |
| Sin token | 401 |

Limpieza opcional del empleado de prueba:

```bash
docker compose -f docker-compose.dev.yml exec -T postgres psql -U editorial -d editorial -c "delete from employee where employee_code='E-100'"
```

(el `docker compose` anterior se ejecuta desde `deploy/`).

## 7. Tests automáticos

```bash
cd services/editorial_core
mvn test
```

## 8. TUI

Con el stack completo arriba, cada área tiene su contenedor (`tui_atencion`, `tui_produccion`, `tui_reclutamiento`, `tui_rrhh`, `tui_eventos`) con `MOCK_MODE=false`, es decir, contra el Core real. Son interactivos (`stdin_open` y `tty`): conéctate con

```bash
docker compose -f docker-compose.dev.yml attach tui_atencion   # salir sin cerrarla: Ctrl+P, Ctrl+Q
```

e inicia sesión con el usuario del área (sección 5). Detalle de teclas y pantallas en `services/editorial_tui/README.md`.

## 9. Detener

```bash
docker compose -f docker-compose.dev.yml down        # conserva datos
docker compose -f docker-compose.dev.yml down -v     # borra volúmenes
```

## 10. Problemas frecuentes

| Síntoma | Causa / solución |
|---|---|
| `ldap` queda `unhealthy` (`err=49` en logs) | `LDAP_ADMIN_PASSWORD` distinta de `admin_dev_only` o volumen viejo: usar `.env.dev.example` y `down -v`. |
| Login da 401 con usuario correcto | Volumen de LDAP anterior sin usuarios nuevos: `down -v`. |
| Core no arranca por dependencia | Esperar a que `ldap` esté `healthy`; ver logs. |
| Puerto 8080 ocupado | Cambiar `CORE_PORT` en `.env`. |
