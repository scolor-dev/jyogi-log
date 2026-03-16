import { Link, Outlet } from 'react-router-dom'

export default function RootLayout() {
  return (
    <div className="min-h-screen flex flex-col">
      <header className="bg-blue-600 text-white p-4">
        <nav className="flex gap-6 items-center">
          <span className="font-bold text-lg">jyogi-OAuth</span>
          <Link to="/" className="hover:underline">Home</Link>
          <Link to="/about" className="hover:underline">About</Link>
          <Link to="/info" className="hover:underline">Info</Link>
        </nav>
      </header>

      <main className="flex-1 p-6">
        <Outlet />
      </main>

      <footer className="bg-gray-100 text-center p-4 text-sm text-gray-500">
        © 2025 jyogi-OAuth
      </footer>
    </div>
  )
}
