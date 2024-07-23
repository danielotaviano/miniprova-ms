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

export interface QuestionApi {
  id: number;
  question: string;
  created_at: string;
}

export interface CreateQuestionApi {
  question: string;
  answers: {
    answer: string;
    is_correct: boolean;
  }[];
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

  return res.ok;
};

export const getQuestions = async (): Promise<QuestionApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/question/questions`, {
    headers: {
      Authorization: `Bearer ${session.user.jwt}`
    }
  });
  return res.json();
};

export const createQuestion = async (
  question: CreateQuestionApi
): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/question/questions`, {
    method: 'POST',
    body: JSON.stringify(question),
    headers: {
      Authorization: `Bearer ${session.user.jwt}`,
      'Content-Type': 'application/json'
    }
  });

  return res.ok;
};
