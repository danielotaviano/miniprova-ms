import { TableCell, TableRow } from '@/components/ui/table';
import { TeacherStudentResultApi } from '@/lib/api';

export function Student({ student }: { student: TeacherStudentResultApi }) {
  return (
    <TableRow>
      <TableCell>{student.name}</TableCell>
      <TableCell>
        {student.answered_questions} / {student.total_questions}
      </TableCell>
      <TableCell>{student.score}</TableCell>
      <TableCell>
        {((student.score / student.total_questions) * 100).toFixed(2)}%
      </TableCell>
    </TableRow>
  );
}
