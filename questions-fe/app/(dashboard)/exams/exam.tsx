import EditExamForm from '@/components/ui/edit-exam-form';
import { TableCell, TableRow } from '@/components/ui/table';
import { deleteExam, ExamApi } from '@/lib/api';
import { Pencil, Trash2, X } from 'lucide-react';
import { useState } from 'react';

export function Exam({ exam }: { exam: ExamApi }) {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const toggleModal = () => setIsModalOpen(!isModalOpen);

  const handleDelete = async () => {
    const res = await deleteExam(exam.id);

    if (res) {
      window.location.reload();
    } else {
      alert('Failed to delete question');
    }
  };

  return (
    <TableRow>
      <TableCell className="font-medium">{exam.id}</TableCell>
      <TableCell>{exam.name}</TableCell>
      <TableCell>{exam.question_count}</TableCell>
      <TableCell className="hidden md:table-cell">
        {new Date(exam.created_at).toLocaleDateString()}
      </TableCell>
      <TableCell className="flex flex-row items-center justify-center space-x-2">
        <Pencil className="cursor-pointer max-w-fit" onClick={toggleModal} />
        <Trash2 className="cursor-pointer max-w-fit" onClick={handleDelete} />
      </TableCell>
      {isModalOpen && (
        <div className="fixed top-16 left-1/2 transform -translate-x-1/2 w-fit h-fit z-50 flex items-start justify-start border border-gray-200 bg-white rounded-lg shadow-lg p-9 overflow-y-auto">
          <div
            className="absolute top-0 right-0 p-4 cursor-pointer"
            onClick={toggleModal}
          >
            <X />
          </div>
          <div className="mt-5">
            <EditExamForm id={exam.id} />
          </div>
        </div>
      )}
    </TableRow>
  );
}
