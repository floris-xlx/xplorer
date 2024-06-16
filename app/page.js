"use client";

// tauri imports
import { invoke } from "@tauri-apps/api/tauri";

import React, { useEffect, useState } from "react";

import Image from "next/image";

import Greet from "./greet.jsx";
import ListDrives from "./components/listDrives.jsx";
import ListFiles from "./components/listFiles.jsx";
import FolderView from "./components/ui/Views/FolderView";
import Header  from "./components/ui/Header";

import ThemeButton from "@/app/components/ui/Theme/ThemeButton";

import { GetKeyLocalStorage, SetKeyLocalStorage } from "@/app/client/caching/LocalStorageRouter";


export default function Home() {
  const [selectedDiskLetter, setSelectedDiskLetter] = useState(null);

  const [path, setPath] = useState(null);
  const [relativePath, setRelativePath] = useState(null);
  console.log("path: ", path);

  const [files, setFiles] = useState([]);
  const [directories, setDirectories] = useState([]);
  const [loadingTime, setLoadingTime] = useState(0);


  useEffect(() => {
    // cache the selected disk letter
    SetKeyLocalStorage("currentDiskLetter", selectedDiskLetter);
   

    if (selectedDiskLetter) {
      const pathFromDiskLetter = `${selectedDiskLetter}:/`
      SetKeyLocalStorage("currentPath", pathFromDiskLetter);
    }

  }, [selectedDiskLetter]);

  useEffect(() => {
    // cache the path
    SetKeyLocalStorage("currentPath", path);
  } , [path]);

  // we manage selected files from here to aid in the cache removal of selected files
  const [selectedFiles, setSelectedFiles] = useState([]);

  const triggerReload = () => {
    // construct path
    const cachedPath = GetKeyLocalStorage("currentPath");

    invoke("list_files", { path: cachedPath })
      .then((files) => {
        setFiles(files.files);
        setDirectories(files.dirs);
        setLoadingTime(files.loading_time);
        console.log("reloaded", files);
      })
      .catch(console.error);
  } 


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
      <div className="fixed top-0 mx-auto font-semibold text-sm  text-primary w-full items-center z-10">
        < Header 
          path={path}
          setPath={setPath}
          setSelectedFiles={setSelectedFiles}
          selectedFiles={selectedFiles}
          triggerReload={triggerReload}
          setSelectedDiskLetter={setSelectedDiskLetter}
        />
      </div>
      
      
      <p className="fixed bottom-0 left-0 text-red-primary font-semibold text-md p-1 select-none bg-secondary border border-red-400 rounded-md z-100 opacity-40 hover:opacity-100 transition z-100"> 
        Loaded in {loadingTime}ms
      </p>


      <div className=" flex overflow-x-hidden ">


        {selectedDiskLetter ? (
          <div className="w-[100vw] px-[25px] mt-[50px]">
            <FolderView
              files={files}
              directories={directories}
              setPath={setPath}
              path={path}
              selectedFiles={selectedFiles}
              setSelectedFiles={setSelectedFiles}
      
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
