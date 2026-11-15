import { useState } from 'react'

function formatearFecha(texto) {
  if (!texto) return ''
  return new Date(texto).toLocaleDateString('es-VE', {
    year: 'numeric', month: 'long', day: 'numeric'
  })
}

export default function ArticleCard({ articulo }) {
  const [abierto, setAbierto] = useState(false)

  return (
    <article className="tarjeta">
      <p className="fecha">{formatearFecha(articulo.publishedAt)}</p>
      <h2>{articulo.title}</h2>
      <p>{articulo.summary}</p>

      {abierto && <p className="cuerpo">{articulo.body}</p>}

      <button className="boton" onClick={() => setAbierto(!abierto)}>
        {abierto ? 'Leer menos' : 'Leer más'}
      </button>
    </article>
  )
}