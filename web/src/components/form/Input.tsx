import type { InputHTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'

type InputProps = InputHTMLAttributes<HTMLInputElement>

export default function Input({ className, ...props }: InputProps) {
  return (
    <input
      className={twMerge(
        'w-full border rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500',
        className,
      )}
      {...props}
    />
  )
}
