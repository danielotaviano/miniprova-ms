import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export enum Role {
  TEACHER = 'TEACHER',
  STUDENT = 'STUDENT',
  ADMIN = 'ADMIN'
}

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
