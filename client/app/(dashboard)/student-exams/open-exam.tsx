import { Button } from '@/components/ui/button';
import { TableCell, TableRow } from '@/components/ui/table';
import { ExamToDoApi } from '@/lib/api';
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';

const adjustToUTCMinus3 = (date: Date) => {
  const adjustedDate = new Date(date);
  adjustedDate.setHours(adjustedDate.getHours() - 3);
  return adjustedDate;
};

export function OpenExam({ exam }: { exam: ExamToDoApi }) {
  const router = useRouter();

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [countdown, setCountdown] = useState(0);
  const toggleModal = () => setIsModalOpen(!isModalOpen);

  useEffect(() => {
    const interval = setInterval(() => {
      const now = new Date();
      const start = adjustToUTCMinus3(new Date(exam.start_time));
      const end = adjustToUTCMinus3(new Date(exam.end_time));

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
        {adjustToUTCMinus3(new Date(exam.start_time)).toLocaleString('en-US', {
          timeZone: 'America/Sao_Paulo',
          year: 'numeric',
          month: '2-digit',
          day: '2-digit',
          hour: '2-digit',
          minute: '2-digit',
          second: '2-digit'
        })}
      </TableCell>
      <TableCell className="hidden md:table-cell">
        {adjustToUTCMinus3(new Date(exam.end_time)).toLocaleString('en-US', {
          timeZone: 'America/Sao_Paulo',
          year: 'numeric',
          month: '2-digit',
          day: '2-digit',
          hour: '2-digit',
          minute: '2-digit',
          second: '2-digit'
        })}
      </TableCell>
      <TableCell>
        <Button
          disabled={
            adjustToUTCMinus3(new Date(exam.start_time)) > new Date() ||
            adjustToUTCMinus3(new Date(exam.end_time)) < new Date()
          }
          onClick={() => {
            router.push(`/exam/${exam.id}`);
          }}
          className="min-w-28"
        >
          {adjustToUTCMinus3(new Date(exam.start_time)) > new Date()
            ? `Start in ${new Date(countdown).toISOString().substr(11, 8)}`
            : 'Start'}
        </Button>
      </TableCell>
    </TableRow>
  );
}
