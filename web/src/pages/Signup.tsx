import { useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import { FormField, Input, PasswordInput, SubmitButton, ErrorMessage } from '../components/form'
import { signupApi, ApiError } from '../api/authApi'

export default function Signup() {
  const navigate = useNavigate()
  const { login } = useAuth()

  const [email, setEmail] = useState('')
  const [displayName, setDisplayName] = useState('')
  const [password, setPassword] = useState('')
  const [error, setError] = useState('')
  const [isLoading, setIsLoading] = useState(false)

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>): Promise<void> => {
    e.preventDefault()
    setError('')
    setIsLoading(true)

    try {
      // POST /api/v1/auth/signup を呼び出す（AC-10）
      const { user, access_token } = await signupApi(email, displayName, password)
      // 新規登録成功時に authContext にセット → /dashboard 遷移（AC-11）
      login(user, access_token)
      navigate('/dashboard')
    } catch (err) {
      // 新規登録失敗時にエラーメッセージ表示（AC-12）
      if (err instanceof ApiError) {
        if (err.status === 409) {
          setError('このメールアドレスはすでに登録されています')
        } else if (err.status === 400) {
          setError(err.message)
        } else {
          setError('新規登録に失敗しました。しばらく時間をおいて再度お試しください。')
        }
      } else {
        setError('新規登録に失敗しました。しばらく時間をおいて再度お試しください。')
      }
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="bg-white p-8 rounded-lg shadow w-full max-w-sm">
        <h1 className="text-2xl font-bold mb-6 text-center">新規登録</h1>

        {/* AC-9: email + display_name + password フィールド */}
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <FormField label="メールアドレス" htmlFor="email">
            <Input
              id="email"
              type="email"
              placeholder="example@example.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
          </FormField>

          <FormField label="表示名" htmlFor="displayName">
            <Input
              id="displayName"
              type="text"
              placeholder="表示名"
              value={displayName}
              onChange={(e) => setDisplayName(e.target.value)}
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
            {isLoading ? '登録中...' : '登録'}
          </SubmitButton>
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
