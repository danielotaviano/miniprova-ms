import AddClassExamForm from '@/components/ui/add-class-exam';
import EditClassForm from '@/components/ui/edit-class-form';
import { TableCell, TableRow } from '@/components/ui/table';
import { deleteClassByTeacher, enrollClass, StudentClassApi } from '@/lib/api';
import { UserPlus } from 'lucide-react';
import { useState } from 'react';

export function UnenrolledClass({
  classByTeacher
}: {
  classByTeacher: StudentClassApi;
}) {
  const enrollHandle = () => {
    enrollClass(classByTeacher.id)
      .then(() => {
        window.location.reload();
      })
      .catch(() => {
        alert('Failed to enroll class');
      });
  };

  return (
    <TableRow>
      <TableCell className="font-medium">{classByTeacher.id}</TableCell>
      <TableCell>{classByTeacher.name}</TableCell>
      <TableCell>{classByTeacher.code}</TableCell>
      <TableCell>{classByTeacher.description}</TableCell>
      <TableCell className="flex flex-row items-center justify-center space-x-2">
        <UserPlus className="cursor-pointer max-w-fit" onClick={enrollHandle} />
      </TableCell>
    </TableRow>
  );
}
