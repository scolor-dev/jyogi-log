import { Link } from 'react-router-dom'

// サービス名（ロゴ）を表示するヘッダー
export default function Header() {
  return (
    <header className="bg-blue-600 text-white p-4">
      <Link to="/" className="font-bold text-xl hover:opacity-90">
        jyogi-OAuth
      </Link>
    </header>
  )
}
