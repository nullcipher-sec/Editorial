# Bootstrap LDIF baked into the image (bind-mounting custom/ breaks osixia startup on Docker Desktop).
FROM osixia/openldap:1.5.0

COPY bootstrap.ldif /container/service/slapd/assets/config/bootstrap/ldif/custom/50-bootstrap.ldif