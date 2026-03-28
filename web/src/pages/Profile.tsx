import { useAuth } from '../hooks/useAuth'

export default function Profile() {
  const { user } = useAuth()

  // PrivateRoute でガード済みだが TypeScript の型保証のため
  if (!user) return null

  const initial = user.display_name.charAt(0).toUpperCase()

  return (
    <div className="max-w-lg mx-auto mt-10 p-6">
      <h1 className="text-2xl font-bold mb-6">プロフィール</h1>

      {/* アバター + 表示名 */}
      <div className="flex items-center gap-4 mb-6">
        <div className="w-16 h-16 rounded-full bg-blue-600 flex items-center justify-center text-white text-2xl font-bold">
          {initial}
        </div>
        <div>
          <p className="text-xl font-semibold">{user.display_name}</p>
          <p className="text-gray-500 text-sm">{user.identifier}</p>
        </div>
      </div>

      {/* 表示モード（AC-4: 新 User 型フィールドのみ表示、編集機能は一時削除） */}
      <div className="border border-gray-200 rounded-lg divide-y divide-gray-200 mb-4">
        <ProfileRow label="表示名"           value={user.display_name} />
        <ProfileRow label="メールアドレス"   value={user.identifier} />
      </div>
    </div>
  )
}

// 表示モード用の1行
function ProfileRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex px-4 py-3 gap-4">
      <span className="w-32 text-gray-500 text-sm shrink-0">{label}</span>
      <span className="text-gray-800 text-sm">{value}</span>
    </div>
  )
}
