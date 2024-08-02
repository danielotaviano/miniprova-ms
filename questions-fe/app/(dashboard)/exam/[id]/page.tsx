'use client';

import { getExamQuestions, GetExamQuestionsApi, submitAnswer } from '@/lib/api';
import { NextApiRequest, NextApiResponse } from 'next';
import { useParams } from 'next/navigation';
import { useEffect, useState } from 'react';

export default function ExamPage() {
  const { id } = useParams();
  const [questions, setQuestions] = useState<GetExamQuestionsApi[]>([]);
  const [selectedAnswers, setSelectedAnswers] = useState<
    Record<number, number>
  >({});
  const [isLoading, setIsLoading] = useState(false);

  const handleAnswerSelect = (questionId: number, answerId: number) => {
    setIsLoading(true);
    submitAnswer(Number(id), questionId, answerId).then(() => {
      setSelectedAnswers((prev) => {
        return {
          ...prev,
          [questionId]: answerId
        };
      });
      setIsLoading(false);
    });
  };

  useEffect(() => {
    getExamQuestions(Number(id)).then((data) => {
      console.log('dataaaa', data);
      setQuestions(data);

      const selectedAnswers: Record<number, number> = {};
      data.forEach((question) => {
        const marked = question.answers.find((answer) => answer.marked);

        if (marked)
          setSelectedAnswers((prev) => ({ ...prev, [question.id]: marked.id }));
      });
    });
  }, []);

  return (
    <div className="p-6">
      <h1
        className={`text-base mb-6 ${
          isLoading ? 'text-yellow-500' : 'text-green-500'
        }`}
      >
        {isLoading ? 'Loading...' : 'Saved!'}
      </h1>

      {questions.map((question, index) => (
        <div key={question.id} className="mb-6  border rounded p-4">
          <div className="flex flex-row">
            <h1 className="text-2xl font-bold mr-4">{index + 1}.</h1>
            <h1 className="text-2xl font-bold mb-4">{question.question}</h1>
          </div>
          <ul className="list-none p-0">
            {question.answers.map((answer) => (
              <li
                key={answer.id}
                className={`p-4 mb-2 border rounded cursor-pointer transition-all shadow-sm 
                  ${isLoading ? 'pointer-events-none ' : 'pointer-events-auto '}
                  ${selectedAnswers[question.id] === answer.id ? 'bg-gray-300' : 'bg-white'}
                `}
                onClick={() => handleAnswerSelect(question.id, answer.id)}
              >
                {answer.answer}
              </li>
            ))}
          </ul>
        </div>
      ))}
      <button
        className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
        // onClick={handleSubmit}
      >
        Submit Answers
      </button>
    </div>
  );
}
