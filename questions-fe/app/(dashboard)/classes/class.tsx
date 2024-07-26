import EditClassForm from '@/components/ui/edit-class-form';
import { TableCell, TableRow } from '@/components/ui/table';
import { ClassByTeacherApi, deleteClassByTeacher } from '@/lib/api';
import { Pencil, Trash2, X } from 'lucide-react';
import { useState } from 'react';

export function Class({
  classByTeacher
}: {
  classByTeacher: ClassByTeacherApi;
}) {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const toggleModal = () => setIsModalOpen(!isModalOpen);

  const handleDelete = async () => {
    const res = await deleteClassByTeacher(classByTeacher.id);

    if (res) {
      window.location.reload();
    } else {
      alert('Failed to delete class');
    }
  };

  return (
    <TableRow>
      <TableCell className="font-medium">{classByTeacher.id}</TableCell>
      <TableCell>{classByTeacher.name}</TableCell>
      <TableCell>{classByTeacher.code}</TableCell>
      <TableCell>{classByTeacher.description}</TableCell>
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
            <EditClassForm id={classByTeacher.id} />
          </div>
        </div>
      )}
    </TableRow>
  );
}
