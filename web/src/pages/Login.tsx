import { Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

export default function Login() {
  const navigate = useNavigate()
  const { login } = useAuth()

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="bg-white p-8 rounded-lg shadow w-full max-w-sm">
        <h1 className="text-2xl font-bold mb-6 text-center">ログイン</h1>

        <form
          onSubmit={(e) => { e.preventDefault(); login(); navigate('/dashboard') }}
          className="flex flex-col gap-4"
        >
          <div>
            <label className="block text-sm font-medium mb-1">ユーザー名</label>
            <input
              type="text"
              className="w-full border rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="username"
            />
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">パスワード</label>
            <input
              type="password"
              className="w-full border rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="••••••••"
            />
          </div>

          <button
            type="submit"
            className="bg-blue-600 text-white rounded-md py-2 hover:bg-blue-700 transition-colors"
          >
            ログイン
          </button>
        </form>

        <p className="mt-4 text-sm text-center text-gray-600">
          アカウントをお持ちでない方は{' '}
          <Link to="/signup" className="text-blue-600 hover:underline">
            新規登録
          </Link>
        </p>
      </div>
    </div>
  )
}
