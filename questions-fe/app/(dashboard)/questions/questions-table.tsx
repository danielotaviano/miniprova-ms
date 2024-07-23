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

import { getQuestions, QuestionApi, UserApi } from '@/lib/api';
import { useEffect, useState } from 'react';
import { Question } from './question';

export function QuestionsTable({}: {}) {
  const [questions, setQuestions] = useState<QuestionApi[]>([]);

  useEffect(() => {
    (async () => {
      getQuestions().then((questions) => {
        console.log(questions);
        setQuestions(questions);
      });
    })();
  }, []);

  return (
    <Card>
      <CardHeader>
        <CardTitle>Questions</CardTitle>
        <CardDescription>Manage your questions.</CardDescription>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Id</TableHead>
              <TableHead>Question</TableHead>
              <TableHead>Created At</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {questions.map((question) => (
              <Question key={question.id} question={question} />
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
