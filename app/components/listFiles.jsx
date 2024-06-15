"use client";

import { useEffect, useState } from 'react';

import { FolderIcon, DocumentIcon } from '@heroicons/react/20/solid';

const ListFiles = ({ files, directories }) => {
    console.log(files);
    console.log(directories);

    return (
        <div className="">
         
            <ul>
                {directories.map((directory) => (
                    <li key={directory.directory}>
                        <div className="flex flex-row gap-1 rounded-md hover:bg-accent transition mt-[2px] select-none p-1 cursor-pointer">
                            < FolderIcon className="w-6 h-6 text-secondary" />
                            <p className="text-primary">
                                {directory.directory.substring(3)}
                            </p>
                        </div>

                    </li>
                ))}
            </ul>
            
            <ul>
                {files.map((file) => (
                    <li key={file.file}>
                        <div className="flex flex-row gap-1 rounded-md hover:bg-accent transition mt-[2px] select-none p-1 cursor-pointer">
                            {["png", "jpeg", "jpg", "ico", "gif", "mp4", "avi"].includes(file.file.split('.').pop().toLowerCase()) && file.preview ? (
                                <img src={`data:image/png;base64,${file.preview}`} alt="preview" className="w-6 h-6" />
                            ) : (
                                < DocumentIcon className="w-6 h-6 text-secondary" />
                            )}
                            <p className="text-primary">{file.file.substring(3)}</p>
                        </div>
                    </li>
                ))}
            </ul>


        </div>
    );
};

export default ListFiles