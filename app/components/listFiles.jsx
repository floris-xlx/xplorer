"use client";

import { useEffect, useState } from 'react';

const ListFiles = ({ files, directories }) => { 
    console.log(files);
    console.log(directories);
    return (
        <div>
            <h1 className="text-primary">Files</h1>
            <ul>
                {files.map((file) => (
                    <li key={file.file}>
                        <p className="text-primary">{file.file}</p>
                    </li>
                ))}
            </ul>

            <h1 className="text-primary">Directories</h1>
            <ul>
                {directories.map((directory) => (
                    <li key={directory.directory}>
                        <p className="text-primary">{directory.directory}</p>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default ListFiles