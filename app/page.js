"use client";

// tauri imports
import { invoke } from "@tauri-apps/api/tauri";

import React, { useEffect, useState } from "react";

import Image from "next/image";

import Greet from "./greet.jsx";
import ListDrives from "./components/listDrives.jsx";
import ListFiles from "./components/listFiles.jsx";

import ThemeButton from "@/app/components/ui/Theme/ThemeButton";

export default function Home() {
  const [selectedDiskLetter, setSelectedDiskLetter] = useState(null);
  console.log(selectedDiskLetter);

  const [files, setFiles] = useState([]);
  const [directories, setDirectories] = useState([]);

  useEffect(() => {
    if (selectedDiskLetter !== null) {
      invoke("list_files", { drive_letter: selectedDiskLetter })
        .then((files) => {
          setFiles(files.files);
          setDirectories(files.dirs);
        })
        .catch(console.error);
    }
  }, [selectedDiskLetter]);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24 bg-primary">
      <Greet />
      {selectedDiskLetter ? (
        <div>
          <p className="text-primary">Selected Drive: {selectedDiskLetter}</p>

          <ListFiles files={files} directories={directories} />
        </div>
      ) : (
        <ListDrives setDriveLetter={setSelectedDiskLetter} />
      )}
    </main>
  );
}
