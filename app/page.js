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
    <main className="flex min-h-screen w-[100vw] flex-col items-center justify-between bg-primary ">

      <div className=" flex overflow-x-hidden ">
        {selectedDiskLetter ? (
          <div className="w-[100vw] px-[25px]">
            <ListFiles files={files} directories={directories} />
          </div>
        ) : (
          <div className="mt-[75px]">
            <ListDrives setDriveLetter={setSelectedDiskLetter} />
            
          </div>
        )}
      </div>

      <div className="fixed bottom-0 right-0 flex p-2">
        <ThemeButton />
      </div>
       
    </main>
  );
}
