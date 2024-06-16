"use client";

// tauri imports
import { invoke } from "@tauri-apps/api/tauri";

import React, { useEffect, useState } from "react";

import Image from "next/image";

import Greet from "./greet.jsx";
import ListDrives from "./components/listDrives.jsx";
import ListFiles from "./components/listFiles.jsx";
import FolderView from "./components/ui/Views/FolderView";

import ThemeButton from "@/app/components/ui/Theme/ThemeButton";

export default function Home() {
  const [selectedDiskLetter, setSelectedDiskLetter] = useState(null);

  const [path, setPath] = useState(null);
  const [relativePath, setRelativePath] = useState(null);
  console.log("path: ", path);

  const [files, setFiles] = useState([]);
  const [directories, setDirectories] = useState([]);
  const [loadingTime, setLoadingTime] = useState(0);

  useEffect(() => {
    // construct path
    const newPath = path ? `${selectedDiskLetter}:${path}` : selectedDiskLetter;

    

    invoke("list_files", { path: newPath })
      .then((files) => {
        setFiles(files.files);
        setDirectories(files.dirs);
        setLoadingTime(files.loading_time);
      })
      .catch(console.error);
  }, [selectedDiskLetter]);


  useEffect (() => {
    if (path) {
      // handle the diskletter alrdy in the path
      const newPath = path.startsWith(`${selectedDiskLetter}:`) ? path : `${selectedDiskLetter}:${path}`;

      invoke("list_files_from_root", { path: newPath })
        .then((files) => {
          setFiles(files.files);
          setDirectories(files.dirs);
          setLoadingTime(files.loading_time);

        })
        .catch(console.error);
    }
  }, [path]);

  
  return (
    <main className="flex min-h-screen w-[100vw] flex-col items-center justify-between bg-primary ">
      <p className="fixed top-0 left-0 text-red-primary font-semibold text-md p-1 select-none bg-secondary border border-red-400 rounded-md "> 
        Loaded in {loadingTime}ms
      </p>


      <div className=" flex overflow-x-hidden ">


        {selectedDiskLetter ? (
          <div className="w-[100vw] px-[25px]">
            <FolderView
              files={files}
              directories={directories}
              setPath={setPath}
              path={path}
      
            />

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
