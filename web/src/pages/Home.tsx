import { Link } from 'react-router-dom'

export default function Home() {
  return (
    <div className="max-w-lg mx-auto mt-16 text-center">
      <h1 className="text-4xl font-bold mb-4">jyogi-OAuth</h1>
      <div className="flex justify-center gap-4">
        <Link
          to="/login"
          className="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700"
        >
          ログイン
        </Link>
        <Link
          to="/signup"
          className="border border-blue-600 text-blue-600 px-6 py-2 rounded hover:bg-blue-50"
        >
          新規登録
        </Link>
      </div>
    </div>
  )
}
