// フッター: コピーライトを表示する
export default function Footer() {
  return (
    <footer className="bg-gray-100 border-t border-gray-200 py-4 text-center text-xs text-gray-500">
      © {new Date().getFullYear()} jyogi-OAuth. All rights reserved.
    </footer>
  )
}
