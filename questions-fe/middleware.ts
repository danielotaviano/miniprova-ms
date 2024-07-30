import { auth } from '@/lib/auth';
import type { NextRequest } from 'next/server';
import { NextResponse } from 'next/server';

export async function middleware(request: NextRequest) {
  try {
    const session = await auth();
    if (!session) {
      return NextResponse.redirect(new URL('/login', request.url));
    }

    const expiresAt = new Date(session.expires);

    if (expiresAt < new Date()) {
      return NextResponse.redirect(new URL('/login', request.url));
    }

    return NextResponse.next();
  } catch (error) {
    return NextResponse.redirect(new URL('/login', request.url));
  }
}

export const config = {
  matcher: ['/']
};
