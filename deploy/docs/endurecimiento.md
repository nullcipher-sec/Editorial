@"
# Endurecimiento de contenedores

## read_only y cap_drop
En docker-compose final:
- read_only: true en lo posible.
- cap_drop: [ALL], añadir solo NET_ADMIN y NET_RAW al firewall.

## Trivy
trivy image --severity HIGH,CRITICAL <imagen> antes de entregar. Integrar en CI si se puede.

## limit_req vs fail2ban
fail2ban requiere logs del host y permisos elevados; frágil en contenedor. limit_req de NGINX es nativo y suficiente.

## Secretos
deploy/secrets/ y .env fuera del repo. Producción: Docker secrets o Vault.
"@ | Out-File -FilePath deploy/docs/endurecimiento.md -Encoding utf8