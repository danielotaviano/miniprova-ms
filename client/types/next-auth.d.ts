// types/next-auth.d.ts
import NextAuth from 'next-auth';

declare module 'next-auth' {
  interface User {
    jwt: string;
    avatar?: string;
    roles: Role[];
  }

  interface AdapterSession {
    jwt: string;
  }

  interface JWT {
    jwt: string;
  }

  interface Session {
    user: User;
  }
}
