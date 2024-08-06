'use client';

import { Button } from '@/components/ui/button';
import { CardContent } from '@/components/ui/card';
import QuestionForm from '@/components/ui/question-form';
import { auth } from '@/lib/auth';
import { Role } from '@/lib/utils';
import { PlusCircle, X } from 'lucide-react';
import { useEffect, useState } from 'react';
import { ExamsTable } from './exams-table';
import ExamForm from '@/components/ui/exam-form';

export default function ExamsPage() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const toggleModal = () => setIsModalOpen(!isModalOpen);
  useEffect(() => {
    async () => {
      const rolesWithAccess = [Role.ADMIN, Role.TEACHER];
      const session = await auth();

      if (!session?.user.roles.some((role) => rolesWithAccess.includes(role))) {
        window.location.href = '/';
      }
    };
  }, []);

  return (
    <>
      <CardContent>
        <ExamsTable />
      </CardContent>
    </>
  );
}
