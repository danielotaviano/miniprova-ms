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
  getExams,
  getQuestions,
  QuestionApi,
  UserApi
} from '@/lib/api';
import { useEffect, useState } from 'react';
import { Exam } from './exam';

export function ExamsTable({}: {}) {
  const [exams, setExams] = useState<ExamApi[]>([]);

  useEffect(() => {
    (async () => {
      getExams().then((exams) => {
        setExams(exams);
      });
    })();
  }, []);

  return (
    <Card>
      <CardHeader>
        <CardTitle>Exams</CardTitle>
        <CardDescription>Manage your exams.</CardDescription>
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
              <Exam key={e.id} exam={e} />
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
