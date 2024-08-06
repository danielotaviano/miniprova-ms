import {
  addExamToClass,
  ExamApi,
  getClassById,
  getExams,
  updateClassByTeacher
} from '@/lib/api';
import React, { useEffect, useState } from 'react';
import { Button } from './button';
import { Input } from './input';

const AddClassExamForm = ({ id }: { id: number }) => {
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [exams, setExams] = useState<ExamApi[]>([]);
  const [selectedExam, setSelectedExam] = useState<number | null>(null);

  useEffect(() => {
    getExams().then((e) => {
      setExams(e);
    });
  }, []);

  const handleSubmit: React.FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault();

    if (!startDate || !endDate) {
      alert('Please fill in all fields');
      return;
    }

    if (!selectedExam) {
      alert('Please select an exam');
      return;
    }

    // verify if end date is greater than start date
    if (new Date(startDate) > new Date(endDate)) {
      alert('End date must be greater than start date');
      return;
    }

    const sDate = new Date(startDate);
    const eDate = new Date(endDate);

    addExamToClass(id, selectedExam, sDate.toISOString(), eDate.toISOString())
      .then((res) => {
        if (res) {
          window.location.reload();
        } else {
          alert('Failed to add exam to class');
        }
      })
      .catch(() => {
        alert('Failed to add exam to class');
      });
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="flex items-start justify-start w-full flex-col space-y-4"
    >
      <div>
        <label htmlFor="start_date">Start date</label>
        <Input
          id="start_date"
          type="datetime-local"
          value={startDate}
          onChange={(e) => setStartDate(e.target.value)}
          className="focus-visible:outline-none focus-visible:ring-0"
        />
      </div>
      <div>
        <label htmlFor="end_date">End date</label>
        <Input
          id="end_date"
          type="datetime-local"
          value={endDate}
          onChange={(e) => setEndDate(e.target.value)}
          className="focus-visible:outline-none focus-visible:ring-0"
        />
      </div>

      <div className="flex flex-col w-full">
        <label htmlFor="exam">Exam</label>
        <select
          name="exam"
          id="exam"
          className="
        border border-gray-200 rounded-md p-2 focus-visible:outline-none focus-visible:ring-0"
          value={selectedExam || ''}
          onChange={(e) => setSelectedExam(Number(e.target.value))}
        >
          <option value="" disabled selected>
            Select an exam
          </option>
          {exams.map((e) => (
            <option key={e.id} value={e.id}>
              {e.name}
            </option>
          ))}
        </select>
      </div>

      <Button type="submit">Submit</Button>
    </form>
  );
};

export default AddClassExamForm;
