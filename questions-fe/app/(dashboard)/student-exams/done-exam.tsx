import { Button } from '@/components/ui/button';
import { TableCell, TableRow } from '@/components/ui/table';
import { ExamToDoApi } from '@/lib/api';
import { useEffect, useState } from 'react';

export function DoneExam({ exam }: { exam: ExamToDoApi }) {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [countdown, setCountdown] = useState(0);
  const toggleModal = () => setIsModalOpen(!isModalOpen);

  useEffect(() => {
    const interval = setInterval(() => {
      const now = new Date();
      const start = new Date(exam.start_time);
      const end = new Date(exam.end_time);

      if (now < start) {
        setCountdown(start.getTime() - now.getTime());
      } else if (now < end) {
        setCountdown(end.getTime() - now.getTime());
      } else {
        setCountdown(0);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [exam]);

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
      <TableCell>
        <Button
          disabled={new Date(exam.start_time) > new Date()}
          onClick={() => {}}
          className="min-w-28"
        >
          View Results
        </Button>
      </TableCell>
    </TableRow>
  );
}
