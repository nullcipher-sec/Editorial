-- Datos de prueba para Sultana del Lago Editores.
-- Se suman a los que ya trae db.sql (no repite sus ids).

-- Usuarios: uno por área (el de PRODUCTION ya existe en db.sql)
INSERT INTO app_user (id, ldap_uid, display_name, area) VALUES
  ('11111111-1111-1111-1111-111111111102', 'att.agente',   'Agente de Atención',  'ATTENTION'),
  ('11111111-1111-1111-1111-111111111103', 'rec.analista', 'Analista de Reclutamiento', 'RECRUITMENT'),
  ('11111111-1111-1111-1111-111111111104', 'hr.analista',  'Analista de RRHH',    'HR'),
  ('11111111-1111-1111-1111-111111111105', 'mkt.gestor',   'Gestor de Marketing', 'MARKETING');

-- Artículos: dos públicos más, uno en revisión y uno en borrador
INSERT INTO article (id, title, summary, body, status, author_id, published_at) VALUES
  ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa3',
   'Feria del libro del lago abre inscripciones',
   'Editoriales y autores locales podrán exhibir sus obras.',
   'La feria del libro reunirá a escritores, ilustradores y lectores de toda la región lacustre.',
   'PUBLISHED', '11111111-1111-1111-1111-111111111101', now() - interval '3 days'),
  ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa4',
   'Entrevista con autores de la región',
   'Tres escritores hablan de su proceso creativo.',
   'Conversamos con tres autores sobre cómo nacen sus historias y qué leen.',
   'PUBLISHED', '11111111-1111-1111-1111-111111111101', now() - interval '5 days'),
  ('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbb4',
   'Resumen anual (en revisión)',
   'Balance del año, pendiente de aprobación.',
   'Texto en revisión editorial, aún no aprobado.',
   'REVIEW', '11111111-1111-1111-1111-111111111101', NULL),
  ('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbb5',
   'Plan de la colección de otoño',
   'Borrador de la planificación de lanzamientos.',
   'Notas internas sobre fechas y títulos, no publicar.',
   'DRAFT', '11111111-1111-1111-1111-111111111101', NULL);

-- Eventos
INSERT INTO event (id, name, description, location, starts_at, ends_at) VALUES
  ('55555555-5555-5555-5555-555555555501', 'Presentación de libro',
   'Lanzamiento de una novela de autor local.', 'Sala principal',
   now() + interval '10 days', now() + interval '10 days 2 hours'),
  ('55555555-5555-5555-5555-555555555502', 'Taller de redacción',
   'Taller abierto para nuevos reporteros.', 'Auditorio',
   now() + interval '20 days', now() + interval '20 days 3 hours');

-- Clientes y casos de atención
INSERT INTO client (id, full_name, email, phone) VALUES
  ('33333333-3333-3333-3333-333333333301', 'Librería El Faro',  'contacto@elfaro.example.com', '0414-0000001'),
  ('33333333-3333-3333-3333-333333333302', 'María Pérez',       'maria.perez@example.com',     '0424-0000002');

INSERT INTO support_case (id, client_id, subject, description, status, assigned_to) VALUES
  ('44444444-4444-4444-4444-444444444401', '33333333-3333-3333-3333-333333333301',
   'Pedido de ejemplares retrasado', 'El cliente no ha recibido su pedido.', 'OPEN',
   '11111111-1111-1111-1111-111111111102'),
  ('44444444-4444-4444-4444-444444444402', '33333333-3333-3333-3333-333333333302',
   'Solicitud de suscripción', 'Quiere suscribirse al boletín mensual.', 'IN_PROGRESS',
   '11111111-1111-1111-1111-111111111102'),
  ('44444444-4444-4444-4444-444444444403', '33333333-3333-3333-3333-333333333301',
   'Cambio de dirección de envío', 'Actualización de datos ya realizada.', 'CLOSED',
   '11111111-1111-1111-1111-111111111102');