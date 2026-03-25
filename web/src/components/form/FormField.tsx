import type { ReactNode } from 'react'

type FormFieldProps = {
  label: string
  htmlFor: string
  children: ReactNode
}

export default function FormField({ label, htmlFor, children }: FormFieldProps) {
  return (
    <div>
      <label htmlFor={htmlFor} className="block text-sm font-medium mb-1">{label}</label>
      {children}
    </div>
  )
}
