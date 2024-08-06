import { getClassById, updateClassByTeacher } from '@/lib/api';
import React, { useEffect, useState } from 'react';
import { Button } from './button';
import { Input } from './input';

const EditClassForm = ({ id }: { id: number }) => {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [code, setCode] = useState('');

  useEffect(() => {
    getClassById(id).then((c) => {
      setName(c.name);
      setDescription(c.description);
      setCode(c.code);
    });
  }, []);

  const handleSubmit: React.FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault();

    if (name === '') return alert('Name is required');
    if (description === '') return alert('Description is required');

    updateClassByTeacher(id, {
      name,
      description
    })
      .then((r) => {
        if (r) return window.location.reload();
        alert('Error creating class');
      })
      .catch(() => {
        alert('Error creating class');
      });
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="flex items-start justify-start w-full flex-col space-y-4"
    >
      <div>
        <label htmlFor="name">Name</label>
        <Input
          id="name"
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="focus-visible:outline-none focus-visible:ring-0"
        />
      </div>
      <div>
        <label htmlFor="name">Code</label>
        <Input
          id="name"
          type="text"
          disabled
          value={code}
          onChange={(e) => setCode(e.target.value)}
          className="focus-visible:outline-none focus-visible:ring-0"
        />
      </div>
      <div>
        <label htmlFor="name">Description</label>
        <Input
          id="name"
          type="text"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          className="focus-visible:outline-none focus-visible:ring-0"
        />
      </div>

      <Button type="submit">Submit</Button>
    </form>
  );
};

export default EditClassForm;
