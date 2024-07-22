import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card';
import { UsersTable } from './users-table';

export default function UsersPage() {
  return (
    <CardContent>
      <UsersTable />
    </CardContent>
  );
}
