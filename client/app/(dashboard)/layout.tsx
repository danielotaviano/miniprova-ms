'use client';

import {
  BookCheck,
  BookOpenCheck,
  BookPlus,
  FileQuestion,
  GraduationCap,
  LucideProps,
  Package2,
  PanelLeft,
  School,
  Users2
} from 'lucide-react';
import Link from 'next/link';

import { Button } from '@/components/ui/button';
import { Sheet, SheetContent, SheetTrigger } from '@/components/ui/sheet';
import { auth } from '@/lib/auth';
import { Role } from '@/lib/utils';
import { ForwardRefExoticComponent, RefAttributes } from 'react';
import { NavItem } from './nav-item';
import Providers from './providers';
import { User } from './user';

export const menuRoleMap: MenuRoleMap[] = [
  {
    label: 'Gestão de usuários',
    icon: Users2,
    href: '/users',
    roles: [Role.ADMIN]
  },
  {
    label: 'Resultados de Avaliações',
    icon: BookCheck,
    href: '/teacher-exams',
    roles: [Role.ADMIN, Role.TEACHER]
  },
  {
    label: 'Avaliações',
    icon: BookOpenCheck,
    href: '/student-exams',
    roles: [Role.ADMIN, Role.STUDENT]
  },
  {
    label: 'Banco de Questões',
    icon: FileQuestion,
    href: '/questions',
    roles: [Role.ADMIN, Role.TEACHER]
  },
  {
    label: 'Banco de Provas',
    icon: BookPlus,
    href: '/exams',
    roles: [Role.ADMIN, Role.TEACHER]
  },
  {
    label: 'Matriculas',
    icon: GraduationCap,
    href: '/enrollment',
    roles: [Role.ADMIN, Role.STUDENT]
  },
  {
    label: 'Turmas',
    icon: School,
    href: '/classes',
    roles: [Role.ADMIN, Role.TEACHER]
  }
];
interface MenuRoleMap {
  label: string;
  icon: ForwardRefExoticComponent<
    Omit<LucideProps, 'ref'> & RefAttributes<SVGSVGElement>
  >;
  href: string;
  roles: Role[];
}

export default async function DashboardLayout({
  children
}: {
  children: React.ReactNode;
}) {
  const session = await auth();

  return (
    <Providers>
      <main className="flex min-h-screen w-full flex-col bg-muted/40">
        <DesktopNav
          roles={session?.user?.roles ?? []}
          menuRoleMap={menuRoleMap}
        />
        <div className="flex flex-col sm:gap-4 sm:py-4 sm:pl-14">
          <header className="sticky top-0 z-30 flex h-14 items-center gap-4 border-b bg-background px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6">
            <MobileNav
              roles={session?.user?.roles ?? []}
              menuRoleMap={menuRoleMap}
            />
            <User />
          </header>
          <main className="grid flex-1 items-start gap-2 p-4 sm:px-6 sm:py-0 md:gap-4 bg-muted/40">
            {children}
          </main>
        </div>
      </main>
    </Providers>
  );
}

function DesktopNav({
  roles,
  menuRoleMap
}: {
  roles: Role[];
  menuRoleMap: MenuRoleMap[];
}) {
  return (
    <aside className="fixed inset-y-0 left-0 z-10 hidden w-14 flex-col border-r bg-background sm:flex">
      <nav className="flex flex-col items-center gap-4 px-2 sm:py-5">
        {menuRoleMap.map((item) => {
          if (item.roles.some((role) => roles.includes(role))) {
            return (
              <NavItem href={item.href} label={item.label}>
                <item.icon className="h-5 w-5" />
              </NavItem>
            );
          }
        })}
      </nav>
      <nav className="mt-auto flex flex-col items-center gap-4 px-2 sm:py-5"></nav>
    </aside>
  );
}

function MobileNav({
  roles,
  menuRoleMap
}: {
  roles: Role[];
  menuRoleMap: MenuRoleMap[];
}) {
  return (
    <Sheet>
      <SheetTrigger asChild>
        <Button size="icon" variant="outline" className="sm:hidden">
          <PanelLeft className="h-5 w-5" />
          <span className="sr-only">Toggle Menu</span>
        </Button>
      </SheetTrigger>
      <SheetContent side="left" className="sm:max-w-xs">
        <nav className="grid gap-6 text-lg font-medium">
          <Link
            href="#"
            className="group flex h-10 w-10 shrink-0 items-center justify-center gap-2 rounded-full bg-primary text-lg font-semibold text-primary-foreground md:text-base"
          >
            <Package2 className="h-5 w-5 transition-all group-hover:scale-110" />
            <span className="sr-only">Vercel</span>
          </Link>
          {roles.includes(Role.TEACHER) && (
            <Link
              href="#"
              className="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
            >
              <FileQuestion className="h-5 w-5" />
              Banco de Questões
            </Link>
          )}

          {/* {menuRoleMap.map((item) => {
            if (item.roles.some((role) => roles.includes(role))) {
              return (
                <Link
                  href={item.href}
                  className="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
                >
                  <item.icon className="h-5 w-5" />
                  {item.label}
                </Link>
              );
            }
          })} */}
        </nav>
      </SheetContent>
    </Sheet>
  );
}
