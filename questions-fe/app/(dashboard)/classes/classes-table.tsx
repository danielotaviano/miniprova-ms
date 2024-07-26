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
  ClassByTeacherApi,
  getClassesByTeacher
} from '@/lib/api';
import { useEffect, useState } from 'react';
import { Class } from './class';

export function ClassesTable({}: {}) {
  const [classes, setClasses] = useState<ClassByTeacherApi[]>([]);

  useEffect(() => {
    (async () => {
      getClassesByTeacher().then((data) => {
        setClasses(data);
      });
    })();
  }, []);

  return (
    <Card>
      <CardHeader>
        <CardTitle>Classes</CardTitle>
        <CardDescription>Manage your classes.</CardDescription>
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
            {classes.map((e) => (
              <Class key={e.id} classByTeacher={e} />
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
