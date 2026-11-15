# Sultana del Lago Editores — Terminal User Interface (TUI)

Aplicación interactiva por terminal para las áreas internas de Sultana del Lago Editores:
* **Atención al Cliente** (VLAN 210)
* **Producción y Contenido** (VLAN 220)
* **Recursos Humanos y Reclutamiento** (VLAN 240 / 230)
* **Eventos y Lanzamientos** (VLAN 250)

---

## 1. Justificación de Seguridad y Tecnología

| Componente | Elección | Justificación de Ciberseguridad |
|---|---|---|
| **Lenguaje** | Rust (Edición 2021) | *Memory Safety* garantizado en compilación: erradica desbordamientos de búfer (*buffer overflow*), punteros colgantes (*use-after-free*) y condiciones de carrera. |
| **Motor TUI** | Ratatui + Crossterm | Renderizado reactivo de alto rendimiento en consola; arquitectura declarativa basada en componentes. |
| **Cliente HTTP** | Reqwest con backend `rustls-tls` | TLS 1.3 implementado 100% en Rust seguro, eliminando vulnerabilidades nativas de OpenSSL en C. |
| **Gestión de Memoria** | Crate `zeroize` | Limpieza física de memoria RAM (*memory wiping*) para contraseñas y tokens en el evento `Drop`. Cero persistencia de credenciales en disco. |
| **Validación de Datos** | Crate `validator` | *Validación defensiva en el cliente*: sanitización estricta de longitudes, formatos de correo y coherencia temporal en fechas. |
| **Arquitectura de Red** | Aislamiento Zero Trust | La terminal **nunca** se conecta a PostgreSQL (`SRV-Data-E`). Toda comunicación viaja a través del firewall central `FW-E` hacia `SRV-Apps-E` (`10.20.30.10:8080`). |

---

## 2. Requisitos y Compilación Local

### Requisitos:
* Rust 1.75+ con Cargo.

### Comandos de desarrollo:
```bash
# Ejecutar la suite completa de pruebas unitarias
cargo test

# Verificación estricta de calidad y buenas prácticas (linter)
cargo clippy -- -D warnings

# Compilar y ejecutar la aplicación interactivamente
cargo run
```

---

## 3. Navegación y Atajos de Teclado

* **Globales:**
  * `[Q]`: Salir de la aplicación (disponible en Login y Menú Principal).
  * `[Esc]`: Volver a la pantalla o listado anterior / Cerrar sesión.
  * `[Enter]` / `[Espacio]`: Cerrar ventanas emergentes de error o notificación.
* **Pantalla de Login:**
  * `[1]` a `[4]`: Selección instantánea de perfiles de prueba de área (`atencion.agente`, `prod.editor`, `rrhh.gerente`, `recluta.especialista`).
  * `[Tab]`: Alternar entre campo de usuario y contraseña.
  * `[Enter]`: Iniciar sesión con las credenciales escritas.
* **Menú Principal (Control de Acceso RBAC):**
  * `[↑ / ↓]`: Seleccionar módulo.
  * `[Enter]`: Acceder al módulo (valida permisos de área; si no corresponde, emite rechazo **403 Forbidden**).
* **Módulos:**
  * **Atención:** `[S]` Avanzar estado de caso (`OPEN` -> `IN_PROGRESS` -> `CLOSED`), `[C]` Directorio de clientes, `[N]` Nuevo caso, `[R]` Nuevo cliente.
  * **Contenido:** `[Enter]` Leer detalle de artículo, `[N]` Redactar artículo, `[R]` Enviar a revisión, `[P]` Publicar al noticiero.
  * **RRHH / Reclutamiento:** `[E]` Empleados, `[V]` Vacantes, `[C]` Candidatos, `[N]` Nueva vacante, `[P]` Postular candidato.
  * **Eventos:** `[N]` Programar nuevo evento.

---

## 4. Despliegue en Contenedores (Simulación VLANs 210–250)

### Construcción de imagen Docker:
```bash
docker build -t editorial-tui:latest .
```

### Ejecución interactiva:
```bash
docker run -it --rm \
  -e CORE_API_URL=http://10.20.30.10:8080 \
  -e MOCK_MODE=true \
  editorial-tui:latest
```

Para la integración en `docker-compose.yml`, cada terminal de área puede instanciarse con un contenedor propio conectado a la red correspondiente coordinada con `@luiscab19`.

La composición de desarrollo del repositorio (`deploy/docker-compose.dev.yml`) incluye una terminal
por área (`tui_atencion`, `tui_produccion`, `tui_reclutamiento`, `tui_rrhh` y `tui_eventos`).
Estas redes Docker reproducen la separación lógica de las VLAN, pero no crean VLAN 802.1Q ni
sustituyen el enrutamiento y las reglas del firewall físico o virtual.

`MOCK_MODE=true` es una simulación local en memoria: no autentica contra LDAP, no persiste en
PostgreSQL y no representa una prueba de conectividad entre VLAN. En despliegues integrados debe
usarse `MOCK_MODE=false`; los errores del Core deben corregirse en el servicio responsable y no
interpretarse como datos reales de la terminal.
