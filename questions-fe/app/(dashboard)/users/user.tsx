import { Badge } from '@/components/ui/badge';
import { TableCell, TableRow } from '@/components/ui/table';
import { setUserRoles, UserApi } from '@/lib/api';
import { Role } from '@/lib/utils';
import Image from 'next/image';
import { useCallback, useState } from 'react';

export function User({ user }: { user: UserApi }) {
  const existingRoles = Object.values(Role);
  const [userCurrentRoles, setUserCurrentRoles] = useState(user.roles);
  const [isLoadingRolesPatch, setIsLoadingRolesPatch] = useState(false);

  const patchRoles = useCallback((id: number, roles: Role[]) => {
    if (isLoadingRolesPatch) return;
    setIsLoadingRolesPatch(true);

    setUserRoles(id, roles)
      .then((result) => {
        if (result) setUserCurrentRoles(roles);
        setIsLoadingRolesPatch(false);
      })
      .finally(() => setIsLoadingRolesPatch(false));
  }, []);

  return (
    <TableRow>
      <TableCell className="hidden sm:table-cell">
        <Image
          alt="Product image"
          className="aspect-square rounded-md object-cover"
          height="64"
          src={user.avatar ?? '/placeholder-user.jpg'}
          width="64"
        />
      </TableCell>
      <TableCell className="font-medium">{user.id}</TableCell>
      <TableCell>{user.name}</TableCell>
      <TableCell className="hidden md:table-cell">{user.email}</TableCell>
      <TableCell className="hidden md:table-cell space-x-1">
        {existingRoles.map((role) => {
          if (!userCurrentRoles.includes(role)) {
            return (
              <span
                key={`${user.id}${role}`}
                onClick={() => {
                  patchRoles(user.id, [...userCurrentRoles, role]);
                }}
                className={
                  isLoadingRolesPatch ? 'cursor-progress' : 'cursor-pointer'
                }
              >
                <Badge
                  key={`${user.id}${role}`}
                  className="capitalize bg-slate-100 text-black"
                >
                  {role}
                </Badge>
              </span>
            );
          }

          return (
            <span
              key={`${user.id}${role}`}
              onClick={() => {
                patchRoles(
                  user.id,
                  userCurrentRoles.filter((r) => r !== role)
                );
              }}
              className={
                isLoadingRolesPatch ? 'cursor-progress' : 'cursor-pointer'
              }
            >
              <Badge key={`${user.id}${role}`} className="capitalize">
                {role}
              </Badge>
            </span>
          );
        })}
      </TableCell>
    </TableRow>
  );
}
