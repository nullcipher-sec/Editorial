const URL_API = '/api/public/articles';

const mensaje = document.getElementById('mensaje');
const lista = document.getElementById('lista');

function formatearFecha(texto) {
  if (!texto) return '';
  const fecha = new Date(texto);
  return fecha.toLocaleDateString('es-VE');
}

function crearArticulo(a) {
  const div = document.createElement('div');
  div.className = 'articulo';

  const titulo = document.createElement('h2');
  titulo.textContent = a.title;

  const info = document.createElement('p');
  info.className = 'info';
  info.textContent = formatearFecha(a.publishedAt || a.published_at);

  const resumen = document.createElement('p');
  resumen.textContent = a.summary;

  div.append(titulo, info, resumen);
  return div;
}

async function cargarArticulos() {
  try {
    const respuesta = await fetch(URL_API);
    if (!respuesta.ok) {
      throw new Error('Error ' + respuesta.status);
    }

    const datos = await respuesta.json();
    // la API devuelve una página: los artículos vienen en "content"
    const articulos = Array.isArray(datos) ? datos : datos.content;

    if (!articulos || articulos.length === 0) {
      mensaje.textContent = 'No hay artículos publicados todavía.';
      return;
    }

    mensaje.textContent = '';
    articulos.forEach(a => lista.appendChild(crearArticulo(a)));
  } catch (error) {
    mensaje.textContent = 'No se pudieron cargar los artículos.';
  }
}

cargarArticulos();