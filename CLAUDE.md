# Sultana del Lago Editores — reglas para Claude

Proyecto 2 de Seguridad Informática (Sultana del Lago Editores). Arquitectura y flujo de equipo: ver `README.md`.

## Commits y Pull Requests
- Claude no aparece como autor ni colaborador: nunca añadir `Co-Authored-By: Claude` ni atribución.
- Conventional Commits en inglés (`feat:`, `fix:`, `docs:`, `chore:`...). Commits pequeños.
- Solo hacer commit o push cuando el usuario lo pida.

## Ramas
- Cada integrante trabaja en `dev/<usuario>`; nunca push directo a `main`. Integración vía PR.

## Seguridad
- Nunca subir `.env`, `deploy/secrets/` ni claves privadas. Solo `.env.example`.
- Seguridad con librerías probadas (Spring Security, Spring LDAP, Bean Validation, mkcert/certbot, age/GPG): no implementar criptografía propia.
- No desactivar validaciones ni debilitar el cifrado para "hacer que funcione".

## Idioma y estilo
- Responder en español. Código, commits y logs en inglés. Comentarios en español.
- `editorial_core`: Java 21 + Spring Boot 3, `application.properties`, perfiles `dev`/`prod`,
  estructura en paquetes `controller`, `service`, `repository`, `entity`, `model`, `exception`, `config`.

## Tareas
- Las tareas de cada integrante están en los issues del repo. Al terminar, marcar la casilla con `gh issue edit`.
