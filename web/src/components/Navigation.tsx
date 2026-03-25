import { Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

export default function Navigation() {
  const { isLoggedIn, logout } = useAuth()
  const navigate = useNavigate()

  return (
    <nav className="bg-white border-b border-gray-200 px-4 py-2 flex items-center gap-6">
      <Link to="/" className="text-gray-700 hover:text-blue-600 text-sm font-medium">
        Home
      </Link>

      {isLoggedIn ? (
        <>
          <Link
            to="/dashboard"
            className="text-gray-700 hover:text-blue-600 text-sm font-medium"
          >
            Dashboard
          </Link>
          <button
            onClick={() => { logout(); navigate('/') }}
            className="ml-auto text-sm font-medium text-red-500 hover:text-red-700"
          >
            Logout
          </button>
        </>
      ) : (
        <div className="ml-auto flex gap-4">
          <Link
            to="/login"
            className="text-sm font-medium text-gray-700 hover:text-blue-600"
          >
            Login
          </Link>
          <Link
            to="/signup"
            className="text-sm font-medium bg-blue-600 text-white px-3 py-1 rounded hover:bg-blue-700"
          >
            Signup
          </Link>
        </div>
      )}
    </nav>
  )
}
