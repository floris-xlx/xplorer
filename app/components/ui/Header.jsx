import React, { useEffect, useState } from "react";

import { ArrowLeftIcon, PencilSquareIcon, EyeDropperIcon, RectangleStackIcon } from '@heroicons/react/20/solid';


import { LuTrash } from "react-icons/lu";

import { RemoveKeyLocalStorage, SetKeyLocalStorage, GetKeyLocalStorage } from "@/app/client/caching/LocalStorageRouter";

// tauri imports
import { invoke } from "@tauri-apps/api/tauri";
import { Input } from '@headlessui/react'

const Header = ({
    setPath,
    path,
    setSelectedFiles,
    selectedFiles,
    triggerReload,
    setSelectedDiskLetter
}) => {
    const [search, setSearch] = useState('');

    console.log('search', search);


    const removeOnePath = (path) => {
        const currentPath = GetKeyLocalStorage('currentPath');
        // if the path is only x:/ then setSelectedDiskLetter to null
        // and dont use the path variable
        if (currentPath.length === 3) {
            setSelectedDiskLetter(null);
            return '';
        }



        if (path === null) {
            return path;
        }
        const lastBackslash = path.lastIndexOf('\\');
        const lastSlash = path.lastIndexOf('/');

        const lastSeparator = Math.max(lastBackslash, lastSlash);
        if (lastSeparator !== -1) {
            const newPath = path.substring(0, lastSeparator);
            if (newPath === "C:" || newPath === "G:") {
                return newPath + "/";
            }
            return newPath;
        }

        return '';
    }


    const removeFilePathCache = () => {
        RemoveKeyLocalStorage('selectedFilePath');
        setTimeout(() => {
            setSelectedFiles([]);
        }, 100);

        SetKeyLocalStorage('lastEvent', 'cache_flushed_by_header');
    }

    const removeFilesInSelection = (folderPaths) => {
        invoke("delete_files", { filepath_list: folderPaths })
            .then((result) => console.log(result))
            .catch(console.error);

        // reload window
        triggerReload();

        setTimeout(() => {
            setSelectedFiles([]);
        }, 100);
    }

    const handleRenameFilesInSelection = (folderPaths) => {
        invoke("rename_files", { filepath_list: folderPaths })
            .then((result) => console.log(result))
            .catch(console.error);

        // reload window
        triggerReload();

        setTimeout(() => {
            setSelectedFiles([]);
        }, 100);
    }

    const handleRemoveBackgroundInSelection = (folderPaths) => {
        invoke("remove_background", { filepath_list: folderPaths })
            .then((result) => console.log(result))
            .catch(console.error);

        // reload window
        triggerReload();

        setTimeout(() => {
            setSelectedFiles([]);
        }, 100);
    }

    const handleImageResizerInSelection = (folderPaths) => {
        invoke("resize_images", { filepath_list: folderPaths })
            .then((result) => console.log(result))
            .catch(console.error);

        // reload window
        triggerReload();

        setTimeout(() => {
            setSelectedFiles([]);
        }, 100);
    }

    useEffect(() => {
        const path = GetKeyLocalStorage('currentPath');

        if (search.length > 0) {
            invoke("search_keyword_in_files", { keyword: search, filepath: path})
                .then((result) => console.log(result))
                .catch(console.error);
        }
    }, [search]);



    return (
        <div className="h-[50px] bg-secondary border-b border-primary w-full flex items-center mx-auto justify-between px-1">

            <div
                onClick={() => setPath(removeOnePath(path))}
            >
                <div className="hover:bg-accent transition p-1 rounded-md cursor-pointer ml-1"
                    onClick={removeFilePathCache}
                >
                    < ArrowLeftIcon className="w-6 h-6 text-primary " />
                </div>


                

                <Input name="full_name" type="text" onChange={(e) => setSearch(e.target.value)} className="border border-primary bg-secondary"/>
            </div>


            <div className="flex flex-row gap-x-1">
                <div className="rounded-md p-1 mr-2 cursor-pointer hover:bg-red-highlight transition"
                    onClick={() => removeFilesInSelection(selectedFiles)}
                >
                    < LuTrash className="w-6 h-6 text-primary" />
                </div>

                <div className="rounded-md p-1 mr-2 cursor-pointer hover:bg-accent transition"
                    onClick={() => handleRenameFilesInSelection(selectedFiles)}
                >
                    < PencilSquareIcon className="w-6 h-6 text-primary" />
                </div>

                <div className="rounded-md p-1 mr-2 cursor-pointer hover:bg-accent transition"
                    onClick={() => handleRemoveBackgroundInSelection(selectedFiles)}
                >
                    < EyeDropperIcon className="w-6 h-6 text-primary" />

                </div>

                <div className="rounded-md p-1 mr-2 cursor-pointer hover:bg-accent transition"
                    onClick={() => handleImageResizerInSelection(selectedFiles)}
                >
                    < RectangleStackIcon className="w-6 h-6 text-primary" />

                </div>
            </div>


        </div>
    );
}
export default Header;
