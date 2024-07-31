'use client';

import { NextApiRequest, NextApiResponse } from 'next';
import { useParams } from 'next/navigation';

export default function ExamPage() {
  const { id } = useParams();

  return <>{id}</>;
}
