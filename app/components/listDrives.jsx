import React, { useEffect, useState } from "react";
import { LuHardDrive } from "react-icons/lu";
import { Progress } from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/tauri";

const ListDrives = ({ setDriveLetter }) => {
  const [drives, setDrives] = useState([]);

  useEffect(() => {
    invoke("list_drives")
      .then((result) => setDrives(result.drives))
      .catch(console.error);
  }, []);

  return (
    <div>
      <h1 className="text-primary">Drives ({drives.length})</h1>
      <ul>
        {drives.map((drive) => (
          <div key={drive.drive_name} className="flex flex-row border-2 border-primary rounded-md items-center mb-2 p-1 px-2 hover:bg-accent transition cursor-pointer">
            <div className="p-4">
              <LuHardDrive className="text-primary" />
            </div>
            <div>
              <li className="text-primary flex flex-col" onClick={() => setDriveLetter(drive.drive_letter)}>
                <p className="text-primary font-semibold select-none">{drive.drive_name} ({drive.drive_letter}:)</p>
                <div className="w-[180px]">
                  <Progress color="default" value={drive.drive_percentage_full} />
                </div>
                <p className="text-secondary font-normal text-sm select-none">{drive.drive_available_space} free of {drive.drive_total_space}</p>
              </li>
            </div>
          </div>
        ))}
      </ul>
    </div>
  );
};

export default ListDrives;
