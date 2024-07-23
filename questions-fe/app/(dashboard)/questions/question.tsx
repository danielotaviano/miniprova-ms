import { TableCell, TableRow } from '@/components/ui/table';
import { QuestionApi } from '@/lib/api';

export function Question({ question }: { question: QuestionApi }) {
  return (
    <TableRow>
      <TableCell className="font-medium">{question.id}</TableCell>
      <TableCell>{question.question}</TableCell>
      <TableCell className="hidden md:table-cell">
        {new Date(question.created_at).toLocaleDateString()}
      </TableCell>
    </TableRow>
  );
}
