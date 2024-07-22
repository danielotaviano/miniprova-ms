'use server';

import { auth } from './auth';
import { Role } from './utils';

export interface UserApi {
  id: number;
  name: string;
  email: string;
  roles: Role[];
  avatar: string;
}

export const getUsers = async (): Promise<UserApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/auth/users`, {
    headers: {
      Authorization: `Bearer ${session.user.jwt}`
    }
  });
  return res.json();
};

export const setUserRoles = async (
  id: number,
  roles: Role[]
): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/auth/users/${id}/roles`, {
    method: 'PATCH',
    body: JSON.stringify(roles),
    headers: {
      Authorization: `Bearer ${session.user.jwt}`,
      'Content-Type': 'application/json'
    }
  });

  console.log('res', res);

  return res.ok;
};
