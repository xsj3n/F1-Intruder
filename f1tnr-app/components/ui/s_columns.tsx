"use client"

import { Checkbox } from "@radix-ui/react-checkbox"
import { ColumnDef } from "@tanstack/react-table"
import * as CheckboxPrimitive from "@radix-ui/react-checkbox"
import { GoDash } from "react-icons/go"



export type HttpData = {
    payload: String,
    status_code: Number,
    length: Number
}

export const http_columns: ColumnDef<HttpData>[] = [
    {
        accessorKey: "payload",
        header: "Payload",
    },
    {
        accessorKey: "status_code",
        header: "Status Code",
    },
    {
        accessorKey: "length",
        header: "Length"
    }
]

export const string_columns: ColumnDef<String>[] = [
    {
        id: "select",
        header: ({ table }) => (
          <></>
        ),
        cell: ({ row }) => 
        (
          <div className="inline-flex items-center">
            <label
              className="relative flex items-center rounded-full cursor-pointer"
              htmlFor="custom"
              >
              <input
                type="checkbox"
                onClick={ (row) => {
                  
                }}
                className="peer relative appearance-none w-5 h-5 border rounded-md border-blue-gray-200 cursor-pointer transition-all before:content[''] before:block before:bg-blue-gray-500 before:w-12 before:h-12 before:rounded-full before:absolute before:top-2/4 before:left-2/4 before:-translate-y-2/4 before:-translate-x-2/4 before:opacity-0 hover:before:opacity-10 before:transition-opacity checked:bg-gray-900 checked:border-gray-900 checked:before:bg-gray-900"
                id="custom"
                /><span
                className="absolute text-white transition-opacity opacity-0 pointer-events-none top-2/4 left-2/4 -translate-y-2/4 -translate-x-2/4 peer-checked:opacity-100"
                ><svg
                  xmlns="http://www.w3.org/2000/svg"
                  className=" ml-1 mt-1 w-4 h-4"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <GoDash></GoDash></svg></span></label>
          </div>  


        ),
        enableSorting: false,
        enableHiding: false,
    },
    {
        accessorKey: "payload strings",
        header: "Payload strings",
        cell: ({row}) => 
        {
            let r = row.original
            return (<>{r}</>)
        }
    },
]

