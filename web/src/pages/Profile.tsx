import { useState } from 'react'
import { Navigate } from 'react-router-dom'
import { useAuth, type User } from '../hooks/useAuth'

export default function Profile() {
  const { isLoggedIn, user, updateUser } = useAuth()

  // フックはすべてここで呼ぶ（条件分岐より前に置くのが React のルール）
  const [isEditing, setIsEditing] = useState(false)
  const [editData, setEditData] = useState<User>({ displayName: '', bio: '', department: '', grade: '' })

  // 未ログインならログインページへ
  if (!isLoggedIn || !user) {
    return <Navigate to="/login" />
  }

  const handleEdit = () => {
    setEditData(user) // 最新データを編集フォームにセット
    setIsEditing(true)
  }

  const handleSave = () => {
    updateUser(editData) // authContext に保存
    setIsEditing(false)
  }

  const handleCancel = () => {
    setIsEditing(false) // 変更を破棄して表示モードに戻る
  }

  const initial = user.displayName.charAt(0).toUpperCase()

  return (
    <div className="max-w-lg mx-auto mt-10 p-6">
      <h1 className="text-2xl font-bold mb-6">プロフィール</h1>

      {/* アバター + 名前 */}
      <div className="flex items-center gap-4 mb-6">
        <div className="w-16 h-16 rounded-full bg-blue-600 flex items-center justify-center text-white text-2xl font-bold">
          {initial}
        </div>
        <div>
          <p className="text-xl font-semibold">{user.displayName}</p>
          <p className="text-gray-500 text-sm">{user.department} / {user.grade}</p>
        </div>
      </div>

      {/* 表示モード */}
      {!isEditing && (
        <>
          <div className="border border-gray-200 rounded-lg divide-y divide-gray-200 mb-4">
            <ProfileRow label="表示名"   value={user.displayName} />
            <ProfileRow label="自己紹介" value={user.bio} />
            <ProfileRow label="学科"     value={user.department} />
            <ProfileRow label="学年"     value={user.grade} />
          </div>
          <button
            onClick={handleEdit}
            className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
          >
            編集
          </button>
        </>
      )}

      {/* 編集モード */}
      {isEditing && (
        <>
          <div className="flex flex-col gap-3 mb-4">
            <EditRow
              label="表示名"
              value={editData.displayName}
              onChange={(v) => setEditData({ ...editData, displayName: v })}
            />
            <EditRow
              label="自己紹介"
              value={editData.bio}
              onChange={(v) => setEditData({ ...editData, bio: v })}
            />
            <EditRow
              label="学科"
              value={editData.department}
              onChange={(v) => setEditData({ ...editData, department: v })}
            />
            <EditRow
              label="学年"
              value={editData.grade}
              onChange={(v) => setEditData({ ...editData, grade: v })}
            />
          </div>
          <div className="flex gap-3">
            <button
              onClick={handleSave}
              className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
            >
              保存
            </button>
            <button
              onClick={handleCancel}
              className="border border-gray-300 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-50"
            >
              キャンセル
            </button>
          </div>
        </>
      )}
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

// 編集モード用の1行
function EditRow({
  label,
  value,
  onChange,
}: {
  label: string
  value: string
  onChange: (v: string) => void
}) {
  return (
    <div className="flex items-center gap-4">
      <label className="w-32 text-gray-500 text-sm shrink-0">{label}</label>
      <input
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="flex-1 border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
  )
}
