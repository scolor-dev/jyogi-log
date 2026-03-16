import { Routes, Route } from 'react-router-dom'
import RootLayout from './components/layouts/RootLayout'
import Home from './pages/Home'
import About from './pages/About'
import Info from './pages/Info/Info'

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<RootLayout />}>
        <Route index element={<Home />} />
        <Route path="about" element={<About />} />
        <Route path="info" element={<Info />} />
      </Route>
    </Routes>
  )
}
