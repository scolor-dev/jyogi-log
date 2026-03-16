import { Link, useNavigate } from 'react-router-dom'

export default function Signup() {
  const navigate = useNavigate()

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="bg-white p-8 rounded-lg shadow w-full max-w-sm">
        <h1 className="text-2xl font-bold mb-6 text-center">新規登録</h1>

        <form onSubmit={(e) => { e.preventDefault(); navigate('/') }} className="flex flex-col gap-4">
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
            登録
          </button>
        </form>

        <p className="mt-4 text-sm text-center text-gray-600">
          すでにアカウントをお持ちの方は{' '}
          <Link to="/login" className="text-blue-600 hover:underline">
            ログイン
          </Link>
        </p>
      </div>
    </div>
  )
}
