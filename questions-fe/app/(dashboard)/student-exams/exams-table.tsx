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
  getExamsToDo,
  getQuestions,
  QuestionApi,
  UserApi
} from '@/lib/api';
import { useEffect, useState } from 'react';
import { Exam } from './exam';

export function ExamsTable({}: {}) {
  const [exams, setExams] = useState<ExamToDoApi[]>([]);

  useEffect(() => {
    getExamsToDo().then((res) => setExams(res));
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
                <TableHead>Id</TableHead>
                <TableHead>Name</TableHead>
                <TableHead>Questions Count</TableHead>
                <TableHead>Created At</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {exams.map((e) => (
                <Exam key={e.exam_name} exam={e} />
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
                <TableHead>Id</TableHead>
                <TableHead>Name</TableHead>
                <TableHead>Questions Count</TableHead>
                <TableHead>Created At</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {/* {exams.map((e) => (
                <Exam key={e.id} exam={e} />
              ))} */}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
