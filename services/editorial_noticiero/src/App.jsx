import { useEffect, useState } from 'react'
import ArticleCard from './ArticleCard'
import logo from './assets/logo-sultana-del-lago.jpg'

export default function App() {
  const [articulos, setArticulos] = useState([])
  const [estado, setEstado] = useState('cargando')

  useEffect(() => {
    fetch('/api/public/articles')
      .then(res => {
        if (!res.ok) throw new Error('Error ' + res.status)
        return res.json()
      })
      .then(datos => {
        setArticulos(Array.isArray(datos) ? datos : datos.content || [])
        setEstado('listo')
      })
      .catch(() => setEstado('error'))
  }, [])

  return (
    <>
      <header className="cabecera">
  <img className="logo" src={logo} alt="Sultana del Lago Editores" />
  <h1>El Noticiero</h1>
       </header>

      <main className="contenido">
        {estado === 'cargando' && <p className="mensaje">Cargando artículos...</p>}
        {estado === 'error' && (
          <p className="mensaje error">No se pudieron cargar los artículos.</p>
        )}
        {estado === 'listo' && articulos.length === 0 && (
          <p className="mensaje">No hay artículos publicados todavía.</p>
        )}

        <section className="lista">
          {articulos.map(a => (
            <ArticleCard key={a.id} articulo={a} />
          ))}
        </section>
      </main>

      <footer className="pie">Sultana del Lago 2026</footer>
    </>
  )
}