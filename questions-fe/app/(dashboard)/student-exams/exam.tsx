import EditExamForm from '@/components/ui/edit-exam-form';
import { TableCell, TableRow } from '@/components/ui/table';
import { deleteExam, ExamApi, ExamToDoApi } from '@/lib/api';
import { Pencil, Trash2, X } from 'lucide-react';
import { useState } from 'react';

export function Exam({ exam }: { exam: ExamToDoApi }) {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const toggleModal = () => setIsModalOpen(!isModalOpen);

  return (
    <TableRow>
      <TableCell className="font-medium">{exam.exam_name}</TableCell>
      <TableCell>{exam.class_name}</TableCell>
      <TableCell className="hidden md:table-cell">
        {new Date(exam.start_time).toLocaleDateString()}{' '}
        {new Date(exam.start_time).toLocaleTimeString()}
      </TableCell>
      <TableCell className="hidden md:table-cell">
        {new Date(exam.end_time).toLocaleDateString()}{' '}
        {new Date(exam.end_time).toLocaleTimeString()}
      </TableCell>
    </TableRow>
  );
}
