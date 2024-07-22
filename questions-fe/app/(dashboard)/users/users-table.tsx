'use client';

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card';
import {
  Table,
  TableBody,
  TableHead,
  TableHeader,
  TableRow
} from '@/components/ui/table';

import { getUsers, UserApi } from '@/lib/api';
import { User } from './user';
import { useEffect, useState } from 'react';
import { auth } from '@/lib/auth';
import { Role } from '@/lib/utils';

export function UsersTable({}: {}) {
  const [users, setUsers] = useState<UserApi[]>([]);

  useEffect(() => {
    (async () => {
      const session = await auth();
      if (!session?.user.roles.includes(Role.ADMIN)) window.location.href = '/';
      getUsers().then((users) => {
        console.log(users);
        setUsers(users);
      });
    })();
  }, []);

  return (
    <Card>
      <CardHeader>
        <CardTitle>Users</CardTitle>
        <CardDescription>Manage your users.</CardDescription>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[100px] sm:table-cell">
                <span className="sr-only">Image</span>
              </TableHead>
              <TableHead>Id</TableHead>
              <TableHead>Name</TableHead>
              <TableHead>Email</TableHead>
              <TableHead>Roles</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {users.map((user) => (
              <User key={user.id} user={user} />
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
