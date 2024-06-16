import React, { useEffect, useState } from "react";

import { ArrowLeftIcon } from '@heroicons/react/20/solid';

const Header = ({
    setPath,
    path
}) => {
    const removeOnePath = (path) => {
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
    return (
        <>
      
        <div className="h-[50px] bg-secondary border-b border-primary w-full flex items-center mx-auto"
            onClick={() => setPath(removeOnePath(path))}
        >
            <div className="hover:bg-accent transition p-1 rounded-md cursor-pointer ml-1">
                < ArrowLeftIcon className="w-6 h-6 text-primary" />
            </div>
            

        </div>
        </>
    );
    }
export default Header;
