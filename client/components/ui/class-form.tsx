import { createClassByTeacher } from '@/lib/api';
import React, { useState } from 'react';
import { Button } from './button';
import { Input } from './input';

const ClassForm = () => {
  const [name, setName] = useState('');
  const [code, setCode] = useState('');
  const [description, setDescription] = useState('');

  const handleSubmit: React.FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault();

    if (name === '') return alert('Name is required');
    if (code === '') return alert('Code is required');
    if (description === '') return alert('Description is required');

    createClassByTeacher({
      name,
      code,
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

export default ClassForm;
