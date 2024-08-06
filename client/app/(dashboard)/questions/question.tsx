import EditQuestionForm from '@/components/ui/edit-question-form';
import { TableCell, TableRow } from '@/components/ui/table';
import { deleteQuestion, QuestionApi } from '@/lib/api';
import { Pencil, X, Trash2 } from 'lucide-react';
import { useState } from 'react';

export function Question({ question }: { question: QuestionApi }) {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const toggleModal = () => setIsModalOpen(!isModalOpen);

  const handleDelete = async () => {
    const res = await deleteQuestion(question.id);

    if (res) {
      alert('Question deleted successfully');
      window.location.reload();
    } else {
      alert('Failed to delete question');
    }
  };

  return (
    <TableRow>
      <TableCell className="font-medium">{question.id}</TableCell>
      <TableCell>{question.question}</TableCell>
      <TableCell className="hidden md:table-cell">
        {new Date(question.created_at).toLocaleDateString()}
      </TableCell>
      <TableCell className="flex flex-row items-center justify-center space-x-2">
        <Pencil className="cursor-pointer max-w-fit" onClick={toggleModal} />
        <Trash2 className="cursor-pointer max-w-fit" onClick={handleDelete} />
      </TableCell>

      {isModalOpen && (
        <div className="fixed top-16 left-1/2 transform -translate-x-1/2 w-fit max-h-fit z-50 flex items-start justify-start border border-gray-200 bg-white rounded-lg shadow-lg p-9 overflow-y-auto">
          <div
            className="absolute top-0 right-0 p-4 cursor-pointer"
            onClick={toggleModal}
          >
            <X />
          </div>
          <div className="">
            <EditQuestionForm questionId={question.id} />
          </div>
        </div>
      )}
    </TableRow>
  );
}
