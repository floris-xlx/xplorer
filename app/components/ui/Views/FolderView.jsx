import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { FolderIcon, DocumentIcon } from '@heroicons/react/20/solid';
import { Checkbox } from "@nextui-org/react";
import { SetKeyLocalStorage, RemoveKeyLocalStorage, GetKeyLocalStorage } from "@/app/client/caching/LocalStorageRouter";

const FolderView = ({ files, directories, setPath, path, selectedFiles, setSelectedFiles }) => {
    useEffect(() => {
        const storedFiles = JSON.parse(GetKeyLocalStorage('selectedFilePath')) || [];
        setSelectedFiles(storedFiles);
    }, []);

    console.log("files", files);
    console.log("directories", directories);

    useEffect(() => {
        SetKeyLocalStorage('selectedFilePath', JSON.stringify(selectedFiles));
    }, [selectedFiles]);

    const openFile = (folderPath) => {
        const fileExtension = folderPath.split('.').pop().toLowerCase();
        const invokeMethod = fileExtension === 'avif' ? "convert_avif_to_webp" : "open_file_from_path";
        invoke(invokeMethod, { path: folderPath })
            .then((result) => console.log(`${invokeMethod} result:`, result))
            .catch(console.error);
    };

    const handleCheckboxChange = (file) => {
        setSelectedFiles((prevSelectedFiles) => prevSelectedFiles.includes(file)
            ? prevSelectedFiles.filter(selectedFile => selectedFile !== file)
            : [...prevSelectedFiles, file]);
    };

    const isThisBoxTicked = (file) => selectedFiles.includes(file);

    return (
        <div>
            <ul>
                {directories.sort((a, b) => a.directory.localeCompare(b.directory)).map((directory) => (
                    <li key={directory.directory}>
                        <div className="flex flex-row gap-1 rounded-md hover:bg-accent transition mt-[2px] select-none p-1 cursor-pointer"
                            onClick={() => setPath(directory.directory)}>
                            <FolderIcon className="w-6 h-6 text-secondary" />
                            <p className="text-primary">{directory.name}</p>
                        </div>
                    </li>
                ))}
            </ul>

            <ul>
                {files.sort((a, b) => a.file.localeCompare(b.file)).map((file) => (
                    <li key={file.file}>
                        <div className="flex flex-row gap-1 rounded-md hover:bg-accent transition mt-[2px] select-none p-1 cursor-pointer"
                            onClick={() => openFile(file.file)}>
                            {["png", "jpeg", "jpg", "ico", "gif", "mp4", "avi"].includes(file.file.split('.').pop().toLowerCase()) && file.preview ? (
                                <img src={`data:image/png;base64,${file.preview}`} alt="preview" className="w-[24px] h-[24px] rounded-md" />
                            ) : (
                                <div className="flex flex-row gap-x-1 z-0">
                                    <Checkbox isSelected={isThisBoxTicked(file.file)} onChange={() => handleCheckboxChange(file.file)} color="success">
                                        <DocumentIcon className="w-6 h-6 text-secondary" />
                                    </Checkbox>
                                </div>
                            )}
                            <p className="text-primary flex items-center">{file.filename}</p>
                        </div>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default FolderView;
