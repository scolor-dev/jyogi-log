import { useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import { FormField, Input, PasswordInput, SubmitButton, ErrorMessage } from '../components/form'
import { loginApi, ApiError } from '../api/authApi'

export default function Login() {
  const navigate = useNavigate()
  const { login } = useAuth()

  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [error, setError] = useState('')
  const [isLoading, setIsLoading] = useState(false)

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>): Promise<void> => {
    e.preventDefault()
    setError('')
    setIsLoading(true)

    try {
      // POST /api/v1/auth/login を呼び出す（AC-6）
      const { user, access_token } = await loginApi(email, password)
      // authContext にセット → /dashboard 遷移（AC-7）
      login(user, access_token)
      navigate('/dashboard')
    } catch (err) {
      // ログイン失敗時にエラーメッセージ表示（AC-8）
      if (err instanceof ApiError) {
        setError(err.message)
      } else {
        setError('ログインに失敗しました。しばらく時間をおいて再度お試しください。')
      }
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="bg-white p-8 rounded-lg shadow w-full max-w-sm">
        <h1 className="text-2xl font-bold mb-6 text-center">ログイン</h1>

        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          {/* AC-5: username から email フィールドへ変更 */}
          <FormField label="メールアドレス" htmlFor="email">
            <Input
              id="email"
              type="email"
              placeholder="example@example.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
          </FormField>

          <FormField label="パスワード" htmlFor="password">
            <PasswordInput
              id="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
          </FormField>

          <ErrorMessage message={error} />

          <SubmitButton disabled={isLoading}>
            {isLoading ? 'ログイン中...' : 'ログイン'}
          </SubmitButton>
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
