'use client';

import {
  getExamQuestions,
  GetExamQuestionsApi,
  getStudentExamResults,
  StudentExamResultApi,
  submitAnswer
} from '@/lib/api';
import { NextApiRequest, NextApiResponse } from 'next';
import { useParams } from 'next/navigation';
import { useCallback, useEffect, useState } from 'react';

export default function ExamPage() {
  const { id } = useParams();
  const [results, setResults] = useState<StudentExamResultApi[]>([]);

  const getResultsSummary = useCallback(() => {
    const totalQuestions = results.length;
    const correctAnswers = results.filter((r) =>
      r.answers.find((a) => a.correct && a.marked)
    ).length;

    return {
      totalQuestions,
      correctAnswers
    };
  }, [results]);

  useEffect(() => {
    getStudentExamResults(Number(id)).then((data) => {
      setResults(data);
    });
  }, []);

  return (
    <div className="p-6">
      <h1 className={`text-base mb-6 `}>
        Your Summary Results:{' '}
        {results.length > 0 && getResultsSummary().correctAnswers} out of{' '}
        {results.length}
      </h1>

      {results.map((question, index) => (
        <div key={question.id} className="mb-6  border rounded p-4">
          <div className="flex flex-row">
            <h1 className="text-2xl font-bold mr-4">{index + 1}.</h1>
            <h1 className="text-2xl font-bold mb-4">{question.question}</h1>
          </div>
          <ul className="list-none p-0">
            {question.answers.map((answer) => (
              <li
                key={answer.id}
                className={`p-4 mb-2 border rounded transition-all shadow-sm 
                  ${answer.correct ? 'bg-green-300' : answer.marked ? 'bg-red-300' : ''}
                `}
              >
                {answer.answer}
              </li>
            ))}
          </ul>
        </div>
      ))}
    </div>
  );
}
