'use server';

import { auth } from './auth';
import { Role } from './utils';

export interface GetExamQuestionsApi {
  id: number;
  question: string;
  answers: {
    id: number;
    answer: String;
    marked: boolean;
  }[];
}

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

export interface CreateClassApi {
  name: string;
  description: string;
  code: string;
}

export interface UpdateClassApi {
  name: string;
  description: string;
}

export interface ClassByTeacherApi {
  id: number;
  name: string;
  code: string;
  description: string;
}

export interface StudentClassApi {
  id: number;
  name: String;
  code: String;
  description: String;
  user_id: number;
}

export interface ExamToDoApi {
  id: number;
  exam_name: string;
  start_time: string;
  end_time: string;
  class_name: string;
}

export interface StudentExamResultApi {
  id: number;
  question: string;
  answers: {
    id: number;
    answer: string;
    correct: boolean;
    marked: boolean;
  }[];
}

export const getStudentExamResults = async (
  examId: number
): Promise<StudentExamResultApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/exam/exams/student/${examId}/results`,
    {
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );
  return res.json();
};

export const submitAnswer = async (
  examId: number,
  questionId: number,
  answerId: number
): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/exam/exams/student/${examId}/question/${questionId}/submit`,
    {
      method: 'POST',
      body: JSON.stringify({ answer_id: answerId }),
      headers: {
        Authorization: `Bearer ${session.user.jwt}`,
        'Content-Type': 'application/json'
      }
    }
  );

  return res.ok;
};

export const getExamQuestions = async (
  id: number
): Promise<GetExamQuestionsApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/exam/exams/student/exam/${id}/questions`,
    {
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );
  return res.json();
};

export const getExamsToDo = async (): Promise<ExamToDoApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/exam/exams/student/open`,
    {
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );
  return res.json();
};

export const getExamsDone = async (): Promise<ExamToDoApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/exam/exams/student/finished`,
    {
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );
  return res.json();
};

export const getUnrolledClasses = async (): Promise<StudentClassApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/exam/classes/students/unenrolled`,
    {
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );
  return res.json();
};

export const getEnrolledClasses = async (): Promise<StudentClassApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/exam/classes/students/enrolled`,
    {
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );
  return res.json();
};

export const enrollClass = async (id: number): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(
    `${process.env.GATEWAY_URL}/exam/classes/${id}/enroll`,
    {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${session.user.jwt}`
      }
    }
  );

  return res.ok;
};

export const getClassesByTeacher = async (): Promise<ClassByTeacherApi[]> => {
  const session = await auth();

  if (!session) {
    return [];
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/exam/classes/teachers`, {
    headers: {
      Authorization: `Bearer ${session.user.jwt}`
    }
  });
  return res.json();
};

export const updateClassByTeacher = async (
  id: number,
  classByTeacher: UpdateClassApi
): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/exam/classes/${id}`, {
    method: 'PATCH',
    body: JSON.stringify(classByTeacher),
    headers: {
      Authorization: `Bearer ${session.user.jwt}`,
      'Content-Type': 'application/json'
    }
  });

  return res.ok;
};

export const getClassById = async (id: number): Promise<ClassByTeacherApi> => {
  const session = await auth();

  if (!session) {
    throw new Error('dont found a session');
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/exam/classes/${id}`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${session.user.jwt}`,
      'Content-Type': 'application/json'
    }
  });

  return res.json();
};

export const createClassByTeacher = async (
  classByTeacher: CreateClassApi
): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/exam/classes`, {
    method: 'POST',
    body: JSON.stringify(classByTeacher),
    headers: {
      Authorization: `Bearer ${session.user.jwt}`,
      'Content-Type': 'application/json'
    }
  });

  return res.ok;
};

export const deleteClassByTeacher = async (id: number): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  const res = await fetch(`${process.env.GATEWAY_URL}/exam/classes/${id}`, {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${session.user.jwt}`
    }
  });

  return res.ok;
};

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

export const addExamToClass = async (
  classId: number,
  examId: number,
  startDate: string,
  endDate: string
): Promise<boolean> => {
  const session = await auth();

  if (!session) {
    return false;
  }

  console.log(
    JSON.stringify({
      exam_id: examId,
      start_date: startDate,
      end_date: endDate,
      class_id: classId
    })
  );

  const res = await fetch(
    `${process.env.GATEWAY_URL}/exam/classes/${classId}/exams`,
    {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${session.user.jwt}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        exam_id: examId,
        start_date: startDate,
        end_date: endDate
      })
    }
  );

  console.log(res);

  return res.ok;
};
