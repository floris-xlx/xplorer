'use client';

import React, { useEffect, useState, Suspense } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { GetKeyLocalStorage, SetKeyLocalStorage, clearAll } from "@/app/client/caching/LocalStorageRouter";
import FolderView from "./components/ui/Views/FolderView";
import Header from "./components/ui/Header";
import ListDrives from "./components/ListDrives";
import ThemeButton from "@/app/components/ui/Theme/ThemeButton";

export default function Home() {
  const [selectedDiskLetter, setSelectedDiskLetter] = useState(GetKeyLocalStorage("currentDiskLetter") || null);
  const [path, setPath] = useState(null);
  const [files, setFiles] = useState([]);
  const [directories, setDirectories] = useState([]);
  const [loadingTime, setLoadingTime] = useState(0);
  const [search, setSearch] = useState('');
  const [selectedFiles, setSelectedFiles] = useState([]);
  const [isInDrive, setIsInDrive] = useState(true);

  console.log('files', files);

  console.log(path);

  useEffect(() => {
    SetKeyLocalStorage("currentDiskLetter", selectedDiskLetter);
    if (selectedDiskLetter) {
      const pathFromDiskLetter = `${selectedDiskLetter}:/`;
      console.log('pathFromDiskLetter', pathFromDiskLetter);
      SetKeyLocalStorage("currentPath", pathFromDiskLetter);
      setIsInDrive(false);
    }
  }, [selectedDiskLetter]);

  useEffect(() => {
    if (path && path !== true ) {
      SetKeyLocalStorage("currentPath", path);

      setIsInDrive(true);
    }
  } , [path]);

  const triggerReload = () => {
    const cachedPath = GetKeyLocalStorage("currentPath");
    if (!cachedPath) return;

    invoke("list_files", { path: cachedPath })
      .then((files) => {
        setFiles(files.files);
        setDirectories(files.dirs);
        setLoadingTime(files.loading_time);
      })
      .catch(console.error);
  };

  useEffect(() => {
    const newPath = path ? `${selectedDiskLetter}:${path}` : selectedDiskLetter;
    invoke("list_files", { path: newPath })
      .then((files) => {
        setFiles(files.files);
        setDirectories(files.dirs);
        setLoadingTime(files.loading_time);
      })
      .catch(console.error);
      console.log('selectedDiskLetter useEffect files', files);
  }, [selectedDiskLetter]);

  useEffect(() => {
    if (path !== true && path !== false && path) {
      
    
      const newPath = path.startsWith(`${selectedDiskLetter}:`) ? path : `${selectedDiskLetter}:${path}`;
      invoke("list_files_from_root", { path: newPath })
        .then((files) => {
          setFiles(files.files);
          setDirectories(files.dirs);
          setLoadingTime(files.loading_time);
        })
        .catch(console.error);
        console.log('path useEffect files', files);
    }
  }, [path]);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (search.length > 0) {
        const path = GetKeyLocalStorage('currentPath');
        invoke("search_keyword_in_files", { keyword: search, filepath: path })
          .then((result) => {
            setFiles(result.results.map(item => item.file));
          })
          .catch(console.error);
      }
    }, 250);

    return () => clearTimeout(timeoutId);

  }, [search]);

  return (
    <main className="flex min-h-screen w-[100vw] flex-col items-center justify-between bg-primary ">
      <div className="fixed top-0 mx-auto font-semibold text-sm  text-primary w-full items-center z-10">
        <Header
          path={path}
          setPath={setPath}
          setSelectedFiles={setSelectedFiles}
          selectedFiles={selectedFiles}
          triggerReload={triggerReload}
          setSelectedDiskLetter={setSelectedDiskLetter}
          search={search}
          setSearch={setSearch}
          setIsInDrive={setIsInDrive}
          isInDrive={isInDrive}
        />
      </div>
      
      <p className="fixed bottom-0 left-0 text-red-primary font-semibold text-md p-1 select-none bg-secondary border border-red-400 rounded-md z-100 opacity-40 hover:opacity-100 transition"> 
        Loaded in {loadingTime}ms

        <button onClick={() => clearAll()} className="ml-2">Clear All</button>
      </p>

      <div className="flex overflow-x-hidden ">
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
