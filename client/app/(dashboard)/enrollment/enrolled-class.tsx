import { TableCell, TableRow } from '@/components/ui/table';
import { StudentClassApi } from '@/lib/api';

export function EnrolledClass({
  classByTeacher
}: {
  classByTeacher: StudentClassApi;
}) {
  return (
    <TableRow>
      <TableCell className="font-medium">{classByTeacher.id}</TableCell>
      <TableCell>{classByTeacher.name}</TableCell>
      <TableCell>{classByTeacher.code}</TableCell>
      <TableCell>{classByTeacher.description}</TableCell>
    </TableRow>
  );
}
