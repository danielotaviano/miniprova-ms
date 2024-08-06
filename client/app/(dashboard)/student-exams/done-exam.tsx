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

export function DoneExam({ exam }: { exam: ExamToDoApi }) {
  const router = useRouter();

  return (
    <TableRow>
      <TableCell className="font-medium">{exam.exam_name}</TableCell>
      <TableCell>{exam.class_name}</TableCell>
      <TableCell className="hidden md:table-cell">
        {adjustToUTCMinus3(new Date(exam.start_time)).toLocaleDateString()}{' '}
        {adjustToUTCMinus3(new Date(exam.start_time)).toLocaleTimeString()}
      </TableCell>
      <TableCell className="hidden md:table-cell">
        {adjustToUTCMinus3(new Date(exam.end_time)).toLocaleDateString()}{' '}
        {adjustToUTCMinus3(new Date(exam.end_time)).toLocaleTimeString()}
      </TableCell>
      <TableCell>
        <Button
          disabled={adjustToUTCMinus3(new Date(exam.start_time)) > new Date()}
          onClick={() => {
            // want to redirect to /exam/[id]
            router.push(`/exam-result/${exam.id}`);
          }}
          className="min-w-28"
        >
          View Results
        </Button>
      </TableCell>
    </TableRow>
  );
}
