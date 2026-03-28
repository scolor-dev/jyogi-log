// ユーザープロフィールの型
// バックエンドの MeResponse { user_uuid, display_name, identifier } に対応
export type User = {
  uuid: string
  identifier: string // メールアドレス等の識別子（バックエンドの identifier フィールド）
  display_name: string
}
