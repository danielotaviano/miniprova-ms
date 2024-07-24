'use server';

import { auth } from './auth';
import { Role } from './utils';

export interface CreateExamApi {
  name: string;
  questions: number[];
}
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

export interface ExamApi {
  id: number;
  name: string;
  created_at: string;
  question_count: number;
}
export interface GetQuestionApi {
  id: string;
  question: string;
  answers: {
    answer: string;
    is_correct: boolean;
  }[];
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

export const createExam = async (exam: CreateExamApi): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/question/exams`, {
    method: 'POST',
    body: JSON.stringify(exam),
    headers: {
      Authorization: `Bearer ${session.user.jwt}`,
      'Content-Type': 'application/json'
    }
  });

  return res.ok;
};

export const updateExam = async (
  id: number,
  exam: CreateExamApi
): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/question/exams/${id}`, {
    method: 'PATCH',
    body: JSON.stringify(exam),
    headers: {
      Authorization: `Bearer ${session.user.jwt}`,
      'Content-Type': 'application/json'
    }
  });

  return res.ok;
};

export const getExamById = async (id: number): Promise<ExamApi | null> => {
  const session = await auth();

  if (!session) {
    return null;
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/question/exams/${id}`, {
    headers: {
      Authorization: `Bearer ${session.user.jwt}`
    }
  });
  return res.json();
};

export const getQuestionsByExamId = async (
  id: number
): Promise<QuestionApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/question/exams/${id}/questions`,
    {
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );
  return res.json();
};

export const editQuestion = async (
  id: number,
  question: CreateQuestionApi
): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/question/questions/${id}`,
    {
      method: 'PATCH',
      body: JSON.stringify(question),
      headers: {
        Authorization: `Bearer ${session.user.jwt}`,
        'Content-Type': 'application/json'
      }
    }
  );

  return res.ok;
};

export const getQuestionById = async (
  id: number
): Promise<GetQuestionApi | null> => {
  const session = await auth();

  if (!session) {
    return null;
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/question/questions/${id}`,
    {
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );
  return res.json();
};

export const deleteQuestion = async (id: number): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/question/questions/${id}`,
    {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );

  return res.ok;
};

export const deleteExam = async (id: number): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/question/exams/${id}`, {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${session.user.jwt}`
    }
  });

  return res.ok;
};

export const getExams = async (): Promise<ExamApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/question/exams`, {
    headers: {
      Authorization: `Bearer ${session.user.jwt}`
    }
  });
  return res.json();
};
