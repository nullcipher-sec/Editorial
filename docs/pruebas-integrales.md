# Guía de pruebas integrales en desarrollo

Esta guía levanta el Core, PostgreSQL, LDAP y las cinco terminales TUI en una misma composición local. Usa `deploy/docker-compose.dev.yml`; no uses `deploy/docker-compose.yml` para probar las aplicaciones, porque esa composición representa la topología y aún tiene servicios de aplicación simulados.

La composición de desarrollo no incluye el noticiero web ni el firewall. Sus redes simulan separación lógica, pero no prueban VLAN 802.1Q ni reglas reales de enrutamiento.

## Requisitos

- Docker Desktop instalado y en ejecución.
- Docker Compose v2 disponible.
- Puertos locales `8080` y `5432` libres.

En PowerShell, desde la raíz del repositorio:

```powershell
docker info
docker compose version
Set-Location .\deploy
```

`docker info` debe terminar correctamente; `docker --version` por sí solo no confirma que el daemon esté disponible.

## Preparar el entorno

Crear `.env` si todavía no existe y abrirlo:

```powershell
if (-not (Test-Path .env)) { Copy-Item .env.example .env }
notepad .env
```

Para seguir esta guía, configura los valores locales de desarrollo así:

```dotenv
POSTGRES_USER=editorial
POSTGRES_PASSWORD=editorial_dev_only
POSTGRES_DB=editorial
LDAP_ADMIN_PASSWORD=admin_dev_only
LDAP_DOMAIN=editorial.local
```

La comprobación de salud de LDAP usa `admin_dev_only`; si cambias `LDAP_ADMIN_PASSWORD`, también tendrás que actualizar el usuario y la contraseña de esa comprobación en `docker-compose.dev.yml`. No reutilices estas credenciales de desarrollo fuera de tu máquina local.

Valida la composición antes de construir:

```powershell
docker compose -f docker-compose.dev.yml config -q
```

## Levantar todos los servicios

```powershell
docker compose -f docker-compose.dev.yml up -d --build
docker compose -f docker-compose.dev.yml ps
```

La primera construcción puede tardar varios minutos. Espera a que `ldap` y `editorial_core` aparezcan como `healthy`; PostgreSQL y las TUI deben estar en ejecución.

Si un servicio no inicia, revisa sus registros:

```powershell
docker compose -f docker-compose.dev.yml logs --tail 100 postgres ldap editorial_core
```

Para seguir los registros en vivo, añade `-f`; `Ctrl+C` termina el seguimiento, no detiene los contenedores.

## Comprobar infraestructura y API

### Salud del Core

```powershell
Invoke-RestMethod http://localhost:8080/actuator/health
```

Resultado esperado: `status` igual a `UP`.

### Artículos públicos

```powershell
$articles = Invoke-RestMethod http://localhost:8080/api/public/articles
$articles.content | Format-Table title
```

Debe responder correctamente y mostrar los artículos publicados de prueba. Esta ruta es pública y no requiere token.

### Autenticación, autorización y acceso a PostgreSQL

Las cuentas de prueba están definidas en `deploy/ldap/bootstrap.ldif`. Por ejemplo, inicia sesión como Atención:

```powershell
$body = @{
    username = 'atencion.agente'
    password = 'EditorialDev!2026'
} | ConvertTo-Json -Compress

$session = Invoke-RestMethod `
    -Method Post `
    -Uri http://localhost:8080/api/auth/login `
    -ContentType 'application/json' `
    -Body $body

$token = $session.token.value
$headers = @{ Authorization = "Bearer $token" }
```

Comprueba que una ruta interna exige autenticación y que el token del área Atención funciona:

```powershell
curl.exe -s -o NUL -w "Sin token: HTTP %{http_code}`n" `
    http://localhost:8080/api/internal/attention/clients

Invoke-RestMethod `
    -Uri http://localhost:8080/api/internal/attention/clients `
    -Headers $headers
```

Esperado: sin token, `401`; con el token válido, respuesta correcta. Para comprobar el control por área, intenta consultar Contenido con el mismo token:

```powershell
curl.exe -s -o NUL -w "Token de Atención en Contenido: HTTP %{http_code}`n" `
    -H "Authorization: Bearer $token" `
    http://localhost:8080/api/internal/content/articles
```

Esperado: `403`.

Credenciales disponibles para las TUI:

| Usuario | Área |
|---|---|
| `atencion.agente` | Atención |
| `prod.editor` | Producción |
| `rrhh.gerente` | RRHH |
| `recluta.especialista` | Reclutamiento |
| `marketing.coord` | Eventos/Marketing |

La contraseña de las cuentas es `EditorialDev!2026`.

## Probar las terminales TUI

Cada terminal corre en su propio contenedor. Conéctate a una desde PowerShell:

```powershell
docker compose -f docker-compose.dev.yml attach tui_atencion
```

Usa las credenciales de su área e intenta iniciar sesión y abrir una sección. En el menú, el acceso a funciones de otra área debe ser rechazado. Para probar otra terminal, sal de la sesión actual o desconéctate y cambia el nombre del servicio:

```powershell
docker compose -f docker-compose.dev.yml attach tui_produccion
docker compose -f docker-compose.dev.yml attach tui_reclutamiento
docker compose -f docker-compose.dev.yml attach tui_rrhh
docker compose -f docker-compose.dev.yml attach tui_eventos
```

Para desconectarte de una terminal sin detener su contenedor, usa `Ctrl+P` y luego `Ctrl+Q`.

## Ejecutar las pruebas automatizadas

Estas pruebas complementan la comprobación integrada anterior:

```powershell
Push-Location ..\services\editorial_core
.\mvnw.cmd test
Pop-Location

Push-Location ..\services\editorial_tui
cargo test
Pop-Location
```

La prueba del Core usa su configuración de test; Cargo requiere Rust instalado localmente. Un resultado exitoso termina con código `0`.

## Detener o reiniciar

Para detener los servicios conservando los contenedores y los datos de PostgreSQL:

```powershell
docker compose -f docker-compose.dev.yml stop
```

Para volver a iniciarlos:

```powershell
docker compose -f docker-compose.dev.yml start
```

`docker compose down` elimina los contenedores; como esta composición no monta un volumen persistente para PostgreSQL, al recrearlo se pierden los datos y se vuelve a cargar `db.sql`. Usa `down -v` solo si quieres borrar también los volúmenes anónimos asociados.

## Problemas frecuentes

- **LDAP no queda `healthy`:** comprueba que `.env` tenga `LDAP_ADMIN_PASSWORD=admin_dev_only` y revisa `docker compose -f docker-compose.dev.yml logs ldap`.
- **El puerto ya está ocupado:** libera `8080` o `5432`, o ajusta los puertos publicados en `.env`/`docker-compose.dev.yml`.
- **Login falla:** confirma que `ldap` esté saludable, que `editorial_core` esté saludable y que el usuario y la contraseña coincidan con la tabla de arriba.
- **Un cambio de datos no aparece:** revisa los registros del Core y la respuesta HTTP. Las operaciones pueden persistir en PostgreSQL mientras el contenedor siga existiendo.
