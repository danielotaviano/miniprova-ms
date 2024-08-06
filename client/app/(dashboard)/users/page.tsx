'use client';

import { CardContent } from '@/components/ui/card';
import { auth } from '@/lib/auth';
import { Role } from '@/lib/utils';
import { useEffect } from 'react';
import { UsersTable } from './users-table';

export default function UsersPage() {
  useEffect(() => {
    async () => {
      const session = await auth();
      if (!session?.user.roles.includes(Role.ADMIN)) window.location.href = '/';
    };
  }, []);

  return (
    <CardContent>
      <UsersTable />
    </CardContent>
  );
}
