'use client';

import { auth } from '@/lib/auth';
import { useRouter } from 'next/navigation';

export default async function Page() {
  const session = await auth();

  return (
    <div className="flex flex-col items-center justify-center ">
      <p className="text-4xl font-bold text-center text-primary">
        Welcome, {session?.user.name}!
      </p>
    </div>
  );
}
