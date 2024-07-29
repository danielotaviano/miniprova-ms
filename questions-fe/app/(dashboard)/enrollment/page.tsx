'use client';

import { Button } from '@/components/ui/button';
import { CardContent } from '@/components/ui/card';
import ClassForm from '@/components/ui/class-form';
import { auth } from '@/lib/auth';
import { Role } from '@/lib/utils';
import { PlusCircle, X } from 'lucide-react';
import { useEffect, useState } from 'react';
import { ClassesTable } from './classes-table';

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
      {isModalOpen && (
        <div className="absolute top-16 left-1/2 transform -translate-x-1/2 w-fit z-50 flex items-start justify-start border border-gray-200 bg-white rounded-lg shadow-lg p-4 overflow-y-auto max-h-[calc(100%-4rem)]">
          <div
            className="absolute top-0 right-0 p-4 cursor-pointer"
            onClick={toggleModal}
          >
            <X />
          </div>
          <div className="flex items-center justify-center w-full">
            <ClassForm />
          </div>
        </div>
      )}
      <CardContent>
        <ClassesTable />
      </CardContent>
    </>
  );
}
