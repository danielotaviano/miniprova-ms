import React, { useCallback, useEffect, useState } from 'react';
import { Input } from './input';
import { ArrowDown, ArrowUp } from 'lucide-react';
import { Button } from './button';
import {
  createExam,
  createQuestion,
  getExamById,
  getQuestionById,
  getQuestions,
  getQuestionsByExamId,
  QuestionApi,
  updateExam
} from '@/lib/api';

const EditExamForm = ({ id }: { id: number }) => {
  const [dbQuestions, setDbQuestions] = useState<QuestionApi[]>([]);
  const [examName, setExamName] = useState('');
  const [selectedQuestions, setSelectedQuestions] = useState<Set<number>>(
    new Set()
  );

  useEffect(() => {
    getQuestions().then((questions) => {
      setDbQuestions(questions);
    });

    getExamById(id).then((exam) => {
      if (!exam) {
        alert('Exam not found');
        return;
      }
      setExamName(exam.name);
    });

    getQuestionsByExamId(id).then((questions) => {
      setSelectedQuestions(new Set(questions.map((q) => q.id)));
    });
  }, []);

  const [visibleAnswers, setVisibleAnswers] = useState<{
    [id: number]: boolean;
  }>({});

  const [answersByQuestion, setAnswersByQuestion] = useState<{
    [id: number]: {
      answer: string;
      is_correct: boolean;
    }[];
  }>({});

  const toggleAnswersVisibility = useCallback(
    async (id: number) => {
      if (!answersByQuestion[id]) {
        const question = await getQuestionById(id);

        if (!question) throw new Error('unable to get answers');
        answersByQuestion[id] = question.answers;
      }

      setVisibleAnswers((prevVisibleAnswers) => ({
        ...prevVisibleAnswers,
        [id]: !prevVisibleAnswers[id]
      }));
    },
    [answersByQuestion]
  );

  const handleSubmit: React.FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault();

    const hasMoreThanOneQuestion = selectedQuestions.size > 0;

    if (!hasMoreThanOneQuestion) {
      alert('You must have at least one question');
      return;
    }

    updateExam(id, {
      name: examName,
      questions: Array.from(selectedQuestions)
    })
      .then((r) => {
        if (r) return window.location.reload();
        alert('Error updating exam');
      })
      .catch(() => {
        alert('Error updating exam');
      });
  };

  const handleExamChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setExamName(e.target.value);
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
          value={examName}
          onChange={handleExamChange}
          className="focus-visible:outline-none focus-visible:ring-0"
        />
      </div>
      <span>Questions</span>
      {dbQuestions.map((dbQuestion, index) => (
        <div className="border p-2 w-full flex flex-col">
          <div
            key={dbQuestion.id}
            className="flex flex-row items-center space-x-2 "
          >
            <Input
              type="checkbox"
              className="focus-visible:outline-none focus-visible:ring-0 w-4"
              checked={selectedQuestions.has(dbQuestion.id)}
              onChange={(e) => {
                if (e.target.checked) {
                  setSelectedQuestions(
                    (prevSelectedQuestions) =>
                      new Set(prevSelectedQuestions.add(dbQuestion.id))
                  );
                } else {
                  setSelectedQuestions((prevSelectedQuestions) => {
                    prevSelectedQuestions.delete(dbQuestion.id);
                    return new Set(prevSelectedQuestions);
                  });
                }
              }}
            />
            <span>{dbQuestion.id}</span>
            <span>{dbQuestion.question}</span>
            <button
              onClick={() => toggleAnswersVisibility(dbQuestion.id)}
              type="button"
              className="transition-all"
            >
              {visibleAnswers[dbQuestion.id] ? (
                <ArrowUp size={15} className="ml-9" />
              ) : (
                <ArrowDown size={15} className="ml-9" />
              )}
            </button>
          </div>
          {visibleAnswers[dbQuestion.id] && (
            <div className="mt-2">
              {answersByQuestion[dbQuestion.id].map((answer, index) => (
                <div
                  key={index}
                  className={'p-1 ' + (answer.is_correct ? 'bg-green-300' : '')}
                >
                  <span className="min-w-3 max-w-3 mr-2">
                    {String.fromCharCode(97 + index)})
                  </span>
                  {answer.answer}
                </div>
              ))}
            </div>
          )}
        </div>
      ))}

      <Button type="submit">Submit</Button>
    </form>
  );
};

export default EditExamForm;
