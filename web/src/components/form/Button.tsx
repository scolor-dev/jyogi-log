import type { ButtonHTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement>

export default function Button({ className, children, ...props }: ButtonProps) {
  return (
    <button
      className={twMerge(
        'rounded-md py-2 font-medium transition-colors bg-blue-600 text-white hover:bg-blue-700',
        className,
      )}
      {...props}
    >
      {children}
    </button>
  )
}
