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
  QuestionApi,
  UserApi
} from '@/lib/api';
import { useEffect, useState } from 'react';
import { OpenExam } from './open-exam';
import { DoneExam } from './done-exam';

export function ExamsTable({}: {}) {
  const [exams, setExams] = useState<ExamToDoApi[]>([]);
  const [finishedExams, setFinishedExams] = useState<ExamToDoApi[]>([]);

  useEffect(() => {
    getExamsToDo().then((res) => setExams(res));
    getExamsDone().then((res) => setFinishedExams(res));
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

      <Card>
        <CardHeader>
          <CardTitle>Results</CardTitle>
          <CardDescription>
            View your result from the past exams.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Class</TableHead>
                <TableHead>Started At</TableHead>
                <TableHead>Ended at</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {finishedExams.map((e) => (
                <DoneExam key={e.exam_name} exam={e} />
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
