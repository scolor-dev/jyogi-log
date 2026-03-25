import { Outlet } from 'react-router-dom'
import Header from '../components/Header'
import Navigation from '../components/Navigation'
import Footer from '../components/Footer'

export default function RootLayout() {
  return (
    <div className="min-h-screen flex flex-col">
      <Header />
      <Navigation />
      <main className="flex-1 p-6">
        <Outlet />
      </main>
      <Footer />
    </div>
  )
}
