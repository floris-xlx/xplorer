'use client';

import Image from "next/image";

import Greet from './greet.jsx';
import ListDrives from './components/listDrives.jsx';

import ThemeButton from '@/app/components/ui/Theme/ThemeButton';

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24 bg-primary">
      <Greet />
      <ListDrives />

    
    
    </main>
  );
}
