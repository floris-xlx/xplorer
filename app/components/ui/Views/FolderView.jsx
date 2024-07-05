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

    const [fileThumbnails, setFileThumbnails] = useState({});
    console.log('fileThumbnails', fileThumbnails);

    // invoke rust func 

    const [localFiles, setLocalFiles] = useState([]);
    console.log('localFiles', localFiles);

    useEffect(() => {
        const mergedFiles = files.map(file => ({
            ...file,
            thumbnail: fileThumbnails[file.file] || null
        }));
        console.log('mergedFiles', mergedFiles);
        setLocalFiles(mergedFiles);
    }, [files, fileThumbnails]);


    useEffect(() => {
        files.forEach((file) => {
            invoke("get_image_thumbnail", { path: file.file })
                .then((thumbnail) => {
                    setFileThumbnails((prevFileThumbnails) => ({
                        ...prevFileThumbnails,
                        [file.file]: thumbnail,
                    }));
                })
                .catch(console.error);

            invoke("get_video_thumbnail", { path: file.file })
                .then((thumbnail) => {
                    setFileThumbnails((prevFileThumbnails) => ({
                        ...prevFileThumbnails,
                        [file.file]: thumbnail,
                    }));
                })
                .catch(console.error);
        });
        console.log('get_thumbnail useEffect files', files);
    }, [files]);


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


    const filePreview = {
        'extra-small': 'w-[32px] h-[32px]',
        'small': 'w-[64px] h-[64px]',
        'medium': 'w-[128px] h-[128px]',
        'large': 'w-[256px] h-[256px]',
        'extra-large': 'w-[512px] h-[512px]',
    }

    const [previewSize, setPreviewSize] = useState('extra-large');

    // 

    return (
        <div>
            <React.Fragment>
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
            </React.Fragment>
            <React.Fragment>
                <ul className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                    {localFiles.sort((a, b) => a.file.localeCompare(b.file)).map((file) => (
                        <li key={file.file}>
                            <div className="flex flex-row gap-1 rounded-md hover:bg-accent transition mt-[2px] select-none p-1 cursor-pointer h-fit "
                                onClick={() => openFile(file.file)}>

                                {file.thumbnail ? (
                                    <img src={`data:image/png;base64,${file.thumbnail}`} alt="thumbnail" className={`${filePreview[previewSize]} rounded-md object-contain`} />
                                ) : (
                                    <div className="flex flex-row gap-x-1 z-0">
                                        <Checkbox isSelected={isThisBoxTicked(file.file)} onChange={() => handleCheckboxChange(file.file)} color="success">
                                            <DocumentIcon className="w-6 h-6 text-secondary" />
                                        </Checkbox>
                                    </div>
                                )}
                                <p className="text-primary flex items-center">{file.filename.length > 30 ? `${file.filename.slice(0, 30)}...` : file.filename}</p>
                            </div>
                        </li>
                    ))}
                </ul>
            </React.Fragment>
        </div>
    );
};

export default FolderView;
