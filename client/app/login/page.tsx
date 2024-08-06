'use client';

import React, { useEffect, useState } from 'react';
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from '@/components/ui/card';
import { signIn, auth } from '@/lib/auth';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import router from 'next/router';

export default function LoginPage() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');

  useEffect(() => {
    const checkAuthStatus = async () => {
      const response = await auth();

      if (response?.user) {
        router.push('/');
      }
    };

    checkAuthStatus();
  }, []);

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await signIn('credentials', {
        redirect: false,
        email,
        password
      });

      window.location.href = '/';
    } catch (error) {
      setError('Not able to login. Please try again.');
    }
  };

  return (
    <div className="min-h-screen flex justify-center items-start md:items-center p-8">
      <Card className="w-full max-w-sm">
        <CardHeader>
          <CardTitle className="text-2xl">Login</CardTitle>
          <CardDescription>
            Please login using your email and password.
          </CardDescription>
        </CardHeader>
        <CardFooter>
          <form onSubmit={handleLogin} className="w-full">
            <Input
              type="email"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full mb-2"
            />
            <Input
              type="password"
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full mb-2"
            />
            <Button type="submit" className="w-full">
              Login
            </Button>

            {error && <p className="text-red-500 text-sm mt-2">{error}</p>}
          </form>
        </CardFooter>
      </Card>
    </div>
  );
}
