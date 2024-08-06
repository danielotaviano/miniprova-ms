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
  getEnrolledClasses,
  getUnrolledClasses,
  StudentClassApi
} from '@/lib/api';
import { useEffect, useState } from 'react';
import { UnenrolledClass } from './unenrolled-class';
import { EnrolledClass } from './enrolled-class';

export function ClassesTable({}: {}) {
  const [classesToEnroll, setClassesToEnroll] = useState<StudentClassApi[]>([]);
  const [enrolledClasses, setEnrolledClasses] = useState<StudentClassApi[]>([]);

  useEffect(() => {
    getUnrolledClasses().then((res) => setClassesToEnroll(res));
    getEnrolledClasses().then((res) => setEnrolledClasses(res));
  }, []);

  return (
    <div className="flex flex-col space-y-4">
      <Card>
        <CardHeader>
          <CardTitle>Take a seat!</CardTitle>
          <CardDescription>Classes available to enrollment</CardDescription>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Id</TableHead>
                <TableHead>Name</TableHead>
                <TableHead>Code</TableHead>
                <TableHead>Description</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {classesToEnroll.map((e) => (
                <UnenrolledClass key={e.id} classByTeacher={e} />
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>Your enrollments</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Id</TableHead>
                <TableHead>Name</TableHead>
                <TableHead>Code</TableHead>
                <TableHead>Description</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {enrolledClasses.map((e) => (
                <EnrolledClass key={e.id} classByTeacher={e} />
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
