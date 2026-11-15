@"
# Decisiones de diseño — Infraestructura

## Topología
FW-E es gateway de todas las VLANs. SW-Core-E es L2 puro. En Docker: un contenedor firewall con una interfaz por red y ip_forward=1. NGINX en DMZ, el firewall hace DNAT 443->NGINX.

## Firewall
nftables sobre iptables. Política DROP por defecto. Sin puertos expuestos (expose). Log limitado a 5/min.

## TLS / NGINX
mkcert para desarrollo. TLS 1.2+/1.3. limit_req en lugar de fail2ban. Failover con upstream backup.

## Respaldo
age sobre GPG. Retención con find -mtime.
"@ | Out-File -FilePath deploy/docs/decisiones.md -Encoding utf8

**Ruta:** `deploy/docs/endurecimiento.md`
@"
# Endurecimiento de contenedores

- read_only: true en lo posible.
- cap_drop: [ALL] y solo NET_ADMIN/NET_RAW al firewall.
- Trivy para escaneo de imágenes.
- limit_req vs fail2ban (nativo y suficiente).
"@ | Out-File -FilePath deploy/docs/endurecimiento.md -Encoding utf8