import { Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import { FormField, Input, PasswordInput, SubmitButton } from '../components/form'

export default function Login() {
  const navigate = useNavigate()
  const { login } = useAuth()

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="bg-white p-8 rounded-lg shadow w-full max-w-sm">
        <h1 className="text-2xl font-bold mb-6 text-center">ログイン</h1>

        {/* TODO: 認証ロジック実装時に入力値をフォームに接続する */}
        <form
          onSubmit={(e) => { e.preventDefault(); login(); navigate('/dashboard') }}
          className="flex flex-col gap-4"
        >
          <FormField label="ユーザー名" htmlFor="username">
            <Input id="username" type="text" placeholder="username" />
          </FormField>

          <FormField label="パスワード" htmlFor="password">
            <PasswordInput id="password" />
          </FormField>

          <SubmitButton>ログイン</SubmitButton>
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
