import { Link } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

// ダッシュボードページ（ログイン後に表示するページ）
export default function Dashboard() {
  const { user } = useAuth()

  if (!user) return null

  return (
    <div className="max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold text-gray-800 mb-6">Dashboard</h1>

      {/* プロフィールカード：クリックするとプロフィールページへ */}
      <Link
        to="/profile"
        className="inline-flex items-center gap-4 p-4 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors"
      >
        {/* アバター（名前の頭文字を丸いアイコンで表示） */}
        <div className="w-14 h-14 rounded-full bg-blue-600 flex items-center justify-center text-white text-xl font-bold shrink-0">
          {user.displayName.charAt(0).toUpperCase()}
        </div>
        <div>
          <p className="font-semibold text-gray-800">{user.displayName}</p>
          <p className="text-sm text-gray-500">{user.department} / {user.grade}</p>
        </div>
      </Link>
    </div>
  )
}
