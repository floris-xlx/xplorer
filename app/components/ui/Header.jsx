import React, { useState, useEffect } from "react";
import { ArrowLeftIcon, PencilSquareIcon, EyeDropperIcon, RectangleStackIcon } from '@heroicons/react/20/solid';
import { LuTrash } from "react-icons/lu";
import { RemoveKeyLocalStorage, SetKeyLocalStorage, GetKeyLocalStorage, clearAll } from "@/app/client/caching/LocalStorageRouter";
import { invoke } from "@tauri-apps/api/tauri";
import { Input } from '@headlessui/react';

const Header = ({ setPath, path, setSelectedFiles, selectedFiles, triggerReload, setSelectedDiskLetter, search, setSearch, setIsInDrive, isInDrive }) => {

    const removeAllCharsTillSlashFromEnd = (currentPath) => {
        const areWeInRoot = currentPath.match(/^[a-zA-Z]:\/$/);
        if (areWeInRoot) {
            setSelectedDiskLetter(null);
            clearAll();
            setIsInDrive(true)

            return;
        }


        const pathArray = currentPath.split('/');
        pathArray.pop();
        const path = pathArray.join('/') + '/';
        setPath(path);
        SetKeyLocalStorage('currentPath', path);
        triggerReload();
    };

    const handleFileAction = (action, folderPaths) => {
        invoke(action, { filepath_list: folderPaths })
            .then((result) => console.log(result))
            .catch(console.error);
        triggerReload();
        setSelectedFiles([]);
    };



    return (
        <div className="h-[50px] bg-secondary border-b border-primary w-full flex items-center mx-auto justify-between px-1">
            <div>

                <div className="hover:bg-accent transition p-1 rounded-md cursor-pointer ml-1" onClick={() => {
                    RemoveKeyLocalStorage('selectedFilePath');
                    setSelectedFiles([]);
                    removeAllCharsTillSlashFromEnd(path);
                }}>
                    <ArrowLeftIcon className="w-6 h-6 text-primary" />
                </div>

                {/* <Input name="full_name" type="text" onChange={(e) => setSearch(e.target.value)} value={search} className="border border-primary bg-secondary" /> */}
            </div>

            <div className="flex flex-row gap-x-1">
                <div className="rounded-md p-1 mr-2 cursor-pointer hover:bg-red-highlight transition" onClick={() => handleFileAction("delete_files", selectedFiles)}>
                    <LuTrash className="w-6 h-6 text-primary" />
                </div>
                <div className="rounded-md p-1 mr-2 cursor-pointer hover:bg-accent transition" onClick={() => handleFileAction("rename_files", selectedFiles)}>
                    <PencilSquareIcon className="w-6 h-6 text-primary" />
                </div>
                <div className="rounded-md p-1 mr-2 cursor-pointer hover:bg-accent transition" onClick={() => handleFileAction("remove_background", selectedFiles)}>
                    <EyeDropperIcon className="w-6 h-6 text-primary" />
                </div>
                <div className="rounded-md p-1 mr-2 cursor-pointer hover:bg-accent transition" onClick={() => handleFileAction("resize_images", selectedFiles)}>
                    <RectangleStackIcon className="w-6 h-6 text-primary" />
                </div>
            </div>
        </div>
    );
};

export default Header;
