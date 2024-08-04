'use server';
import NextAuth from 'next-auth';
import CredentialsProvider from 'next-auth/providers/credentials';
import { Role } from './utils';

interface MeResponse {
  id: string;
  roles: Role[];
  name: string;
}

export const { signIn, auth, signOut } = NextAuth({
  providers: [
    CredentialsProvider({
      name: 'Credentials',
      credentials: {
        email: { label: 'Email', type: 'text' },
        password: { label: 'Password', type: 'password' }
      },
      async authorize(credentials) {
        try {
          const gateway = process.env.GATEWAY_URL;
          const res = await fetch(`${gateway}/auth/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              email: credentials.email,
              password: credentials.password
            })
          });
          if (!res.ok) return null;
          const { token } = await res.json();

          if (res.ok && token) {
            return { jwt: token, roles: [] };
          }

          return null;
        } catch (error) {
          return null;
        }
      }
    })
  ],
  callbacks: {
    async jwt({ token, user }) {
      // If user just logged in, save their token to the token object
      if (user) {
        token.jwt = user.jwt;
      }

      return token;
    },
    async session({ session, token, user }) {
      const gateway = process.env.GATEWAY_URL;
      const res = await fetch(`${gateway}/auth/me`, {
        method: 'GET',
        headers: {
          Authorization: `Bearer ${token.jwt}`,
          'Content-Type': 'application/json'
        }
      });

      if (!res.ok) throw new Error('Failed to fetch user information');
      const userInfo: MeResponse = await res.json();

      console.log('session.user', session.user);
      console.log('userInfo', userInfo);

      session.user = {
        ...session.user,
        ...userInfo,
        jwt: token.jwt as string
      };

      return session;
    }
  },
  pages: {
    signIn: '/login'
    // error: '/auth/error', // Error code passed in query string as ?error=
    // signOut: '/signout',
    // verifyRequest: '/auth/verify-request', // (used for check email message)
    // newUser: null // If set, new users will be directed here on first sign in
  }
});
