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
import { getTeacherStudentResults, TeacherStudentResultApi } from '@/lib/api';
import { useParams } from 'next/navigation';
import { useEffect, useState } from 'react';
import { Student } from './student';

export default function ExamsTable({}: {}) {
  const { id } = useParams();

  const [students, setStudents] = useState<TeacherStudentResultApi[]>([]);

  useEffect(() => {
    getTeacherStudentResults(Number(id)).then((data) => {
      setStudents(data);
    });
  }, []);
  return (
    <div className="flex flex-col space-y-4">
      <Card>
        <CardHeader>
          <CardTitle>Students</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Answers Count</TableHead>
                <TableHead>Correct Answers</TableHead>
                <TableHead>Performance %</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {students.map((e) => (
                <Student key={e.id} student={e} />
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
