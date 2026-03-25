import type { ComponentProps } from 'react'
import { twMerge } from 'tailwind-merge'
import Button from './Button'

type SubmitButtonProps = Omit<ComponentProps<typeof Button>, 'type'>

export default function SubmitButton({ className, ...props }: SubmitButtonProps) {
  return <Button type="submit" className={twMerge('w-full', className)} {...props} />
}
