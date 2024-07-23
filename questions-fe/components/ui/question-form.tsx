import React, { useState } from 'react';
import { Input } from './input';
import { Trash2 } from 'lucide-react';
import { Button } from './button';
import { createQuestion } from '@/lib/api';

const QuestionForm = () => {
  const [question, setQuestion] = useState('');
  const [alternatives, setAlternatives] = useState([
    { text: '', isCorrect: false }
  ]);

  const handleQuestionChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setQuestion(e.target.value);
  };

  const handleAlternativeChange = (
    index: number,
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const newAlternatives = alternatives.map((alt, i) => {
      if (i === index) {
        return { ...alt, text: e.target.value };
      }
      return alt;
    });
    setAlternatives(newAlternatives);
  };

  const handleCorrectChange = (index: number) => {
    const newAlternatives = alternatives.map((alt, i) => ({
      ...alt,
      isCorrect: i === index
    }));
    setAlternatives(newAlternatives);
  };

  const addAlternative = () => {
    setAlternatives([...alternatives, { text: '', isCorrect: false }]);
  };

  const removeAlternative = (index: number) => {
    if (alternatives.length > 1) {
      const newAlternatives = alternatives.filter((_, i) => i !== index);
      setAlternatives(newAlternatives);
    }
  };

  const handleSubmit: React.FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault();

    const hasMoreThanOneAlternative = alternatives.length > 1;

    const hasCorrectAlternative = alternatives.some(
      (alternative) => alternative.isCorrect
    );

    if (!hasMoreThanOneAlternative) {
      alert('You must have at least two alternatives');
      return;
    }

    if (!hasCorrectAlternative) {
      alert('You must have at least one correct alternative');
      return;
    }

    createQuestion({
      answers: alternatives.map((alternative) => ({
        answer: alternative.text,
        is_correct: alternative.isCorrect
      })),
      question
    })
      .then(() => {
        window.location.reload();
      })
      .catch(() => {
        alert('An error occurred');
      });
  };
  return (
    <form
      onSubmit={handleSubmit}
      className="flex items-start justify-start w-full flex-col space-y-4"
    >
      <div>
        <label htmlFor="question">Question</label>
        <Input
          id="question"
          type="text"
          value={question}
          onChange={handleQuestionChange}
          className="focus-visible:outline-none focus-visible:ring-0"
        />
      </div>
      <span>Alternatives</span>
      {alternatives.map((alternative, index) => (
        <div key={index} className="flex flex-row items-center space-x-1">
          <span className="min-w-3 max-w-3 mr-2">
            {String.fromCharCode(97 + index)})
          </span>
          <Input
            type="text"
            value={alternative.text}
            onChange={(e) => handleAlternativeChange(index, e)}
            className="h-6 w-28"
          />

          <Input
            type="radio"
            name="correctAlternative"
            checked={alternative.isCorrect}
            onChange={() => handleCorrectChange(index)}
            className="h-4 w-fit"
          />

          <Trash2
            onClick={() => removeAlternative(index)}
            className="h-4 cursor-pointer w-fit !ml-5"
            size={100}
          />
        </div>
      ))}

      <Button
        onClick={addAlternative}
        variant="outline"
        size="sm"
        type="button"
      >
        Add Alternative
      </Button>
      {/* <button type="submit">Submit</button> */}

      <Button type="submit">Submit</Button>
    </form>
  );
};

export default QuestionForm;
