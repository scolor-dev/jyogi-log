import { Link, Outlet } from 'react-router-dom'

export default function RootLayout() {
  return (
    <div className="min-h-screen flex flex-col">
      <header className="bg-blue-600 text-white p-4">
        <nav className="flex items-center">
          <span className="font-bold text-lg mr-6">jyogi-OAuth</span>
          <div className="flex gap-6">
            <Link to="/" className="hover:underline">Home</Link>
            <Link to="/about" className="hover:underline">About</Link>
            <Link to="/info" className="hover:underline">Info</Link>
          </div>
          <div className="ml-auto flex gap-4">
            <Link to="/login" className="hover:underline">ログイン</Link>
            <Link to="/signup" className="hover:underline">新規登録</Link>
          </div>
        </nav>
      </header>

      <main className="flex-1 p-6">
        <Outlet />
      </main>
    </div>
  )
}
