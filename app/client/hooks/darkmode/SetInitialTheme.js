// components/SetInitialTheme.js
'use client';

import { useEffect } from 'react';

export default function SetInitialTheme() {
  useEffect(() => {
    const theme = localStorage.getItem('theme') || 'light';
    document.documentElement.className = theme === 'dark' ? 'dark-theme-class' : 'light-theme-class';
  }, []);

  return null;
}
