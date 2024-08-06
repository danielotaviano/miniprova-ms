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

import {
  ExamApi,
  ExamToDoApi,
  getExams,
  getExamsDone,
  getExamsToDo,
  getQuestions,
  getTeacherExams,
  QuestionApi,
  TeacherExamApi,
  UserApi
} from '@/lib/api';
import { useEffect, useState } from 'react';
import { OpenExam } from './open-exam';

export function ExamsTable({}: {}) {
  const [exams, setExams] = useState<TeacherExamApi[]>([]);
  const [finishedExams, setFinishedExams] = useState<ExamToDoApi[]>([]);

  useEffect(() => {
    getTeacherExams().then((res) => setExams(res));
  }, []);

  return (
    <div className="flex flex-col space-y-4">
      <Card>
        <CardHeader>
          <CardTitle>Exams</CardTitle>
          <CardDescription>Exams to do</CardDescription>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Class</TableHead>
                <TableHead>Start At</TableHead>
                <TableHead>End At</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {exams.map((e) => (
                <OpenExam key={e.exam_name} exam={e} />
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
