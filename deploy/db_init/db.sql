-- Esquema inicial de Sultana del Lago Editores (fase 1).
-- Se aplica al arrancar PostgreSQL (docker-entrypoint-initdb.d).

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE app_user (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ldap_uid        VARCHAR(128) NOT NULL UNIQUE,
    display_name    VARCHAR(255) NOT NULL,
    area            VARCHAR(32)  NOT NULL CHECK (area IN ('ATTENTION', 'PRODUCTION', 'RECRUITMENT', 'HR', 'MARKETING')),
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE TABLE client (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name       VARCHAR(255) NOT NULL,
    email           VARCHAR(255),
    phone           VARCHAR(64),
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE TABLE support_case (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id       UUID         NOT NULL REFERENCES client (id) ON DELETE RESTRICT,
    subject         VARCHAR(500) NOT NULL,
    description     TEXT,
    status          VARCHAR(32)  NOT NULL DEFAULT 'OPEN'
                    CHECK (status IN ('OPEN', 'IN_PROGRESS', 'CLOSED')),
    assigned_to     UUID         REFERENCES app_user (id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE TABLE article (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title           VARCHAR(500) NOT NULL,
    summary         TEXT,
    body            TEXT         NOT NULL,
    status          VARCHAR(32)  NOT NULL DEFAULT 'DRAFT'
                    CHECK (status IN ('DRAFT', 'REVIEW', 'PUBLISHED')),
    author_id       UUID         REFERENCES app_user (id) ON DELETE SET NULL,
    published_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT now(),
    CONSTRAINT article_published_at_check CHECK (
        (status = 'PUBLISHED' AND published_at IS NOT NULL)
        OR (status <> 'PUBLISHED')
    )
);

CREATE TABLE employee (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_code   VARCHAR(32)  NOT NULL UNIQUE,
    full_name       VARCHAR(255) NOT NULL,
    email           VARCHAR(255) NOT NULL,
    department      VARCHAR(32)  NOT NULL CHECK (department IN ('ATTENTION', 'PRODUCTION', 'RECRUITMENT', 'HR', 'MARKETING')),
    hired_at        DATE         NOT NULL,
    active          BOOLEAN      NOT NULL DEFAULT true
);

CREATE TABLE job_opening (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title           VARCHAR(255) NOT NULL,
    description     TEXT,
    area            VARCHAR(32)  NOT NULL CHECK (area IN ('ATTENTION', 'PRODUCTION', 'RECRUITMENT', 'HR', 'MARKETING')),
    open            BOOLEAN      NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE TABLE candidate (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_opening_id  UUID         NOT NULL REFERENCES job_opening (id) ON DELETE CASCADE,
    full_name       VARCHAR(255) NOT NULL,
    email           VARCHAR(255) NOT NULL,
    resume_summary  TEXT,
    stage           VARCHAR(64)  NOT NULL DEFAULT 'APPLIED',
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE TABLE event (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(255) NOT NULL,
    description     TEXT,
    location        VARCHAR(255),
    starts_at       TIMESTAMPTZ  NOT NULL,
    ends_at         TIMESTAMPTZ,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE TABLE campaign (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(255) NOT NULL,
    channel         VARCHAR(128),
    budget_cents    BIGINT,
    starts_on       DATE,
    ends_on         DATE,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX idx_support_case_client ON support_case (client_id);
CREATE INDEX idx_article_status_published ON article (status) WHERE status = 'PUBLISHED';
CREATE INDEX idx_candidate_job_opening ON candidate (job_opening_id);

-- Datos de muestra para desarrollo local y pruebas del noticiero (@JhonmarMorales17).
INSERT INTO app_user (id, ldap_uid, display_name, area) VALUES
    ('11111111-1111-1111-1111-111111111101', 'prod.editor', 'Editor de Producción', 'PRODUCTION');

INSERT INTO article (id, title, summary, body, status, author_id, published_at) VALUES
    (
        'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1',
        'Sultana del Lago inaugura nueva sala de prensa',
        'La editorial estrena instalaciones para cubrir la región lacustre.',
        'Sultana del Lago Editores presentó hoy su nueva sala de prensa con equipos de transmisión y estudios de audio.',
        'PUBLISHED',
        '11111111-1111-1111-1111-111111111101',
        now() - interval '2 days'
    ),
    (
        'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2',
        'Convocatoria abierta: reporteros de campo',
        'Buscamos talento para refuerzo en la cuenca del lago.',
        'El área de Reclutamiento abrió vacantes para reporteros con experiencia en cobertura in situ.',
        'PUBLISHED',
        '11111111-1111-1111-1111-111111111101',
        now() - interval '1 day'
    ),
    (
        'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbb3',
        'Borrador interno — no publicar',
        'Este artículo está en borrador.',
        'Contenido confidencial en revisión editorial.',
        'DRAFT',
        '11111111-1111-1111-1111-111111111101',
        NULL
    );
