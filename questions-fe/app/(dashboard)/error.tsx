'use client';

import { useEffect } from 'react';

export default function Error({
  error,
  reset
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    // Log the error to an error reporting service
    console.error(error);
  }, [error]);

  return (
    <main className="p-4 md:p-6">
      <h1 className="text-2xl font-semibold">An error occurred</h1>
      <p>
        {error.message ||
          'An unexpected error occurred. Please try again later.'}
      </p>
      <button
        className="mt-4 px-4 py-2 bg-blue-500 text-white rounded"
        onClick={reset}
      >
        Try again
      </button>
    </main>
  );
}
