"use client"

import { Checkbox } from "@radix-ui/react-checkbox"
import { ColumnDef, Row, Table } from "@tanstack/react-table"
import * as CheckboxPrimitive from "@radix-ui/react-checkbox"
import { GoArrowLeft, GoArrowRight, GoDash } from "react-icons/go"
import { table } from "console"

import { ChangeEventHandler, useMemo } from "react"
import { Button } from "./button"



export type HttpData = {
    payload: String,
    status_code: Number,
    status_string: String,
    length: Number
}

export let http_table_inst: Table<HttpData> | null = null
export const http_columns: ColumnDef<HttpData>[] = [
    {
      id: "head",
      header: ({table}) =>
      {
        http_table_inst = table
      }
    },
    {
        accessorKey: "payload",
        header: "Payload",
        cell: ({row}) =>
        {
        return (<>{row.original.payload}</>)
        }
    },
    {
        accessorKey: "status_code",
        header: "Status Code",
        cell: ({row}) =>
        {
          return (<>{row.original.status_code}</>)
        }
    },
    {
      accessorKey: "status_string",
      header: "Status String",
      cell: ({row}) =>
      {
        return (<>{row.original.status_string}</>)
      }
    },
    {
        accessorKey: "length",
        header: "Length",
        cell: ({row}) =>
        {
          return (<>{row.original.length}</>)
        }
    }
]
export let strs_to_be_removed: String[]  = []
export let remove_toggled_strs_was_ran  = false;
export async function set_remove_toggled_strs_was_ran(bool:  boolean) 
{
  remove_toggled_strs_was_ran = bool 
}

export async function clear_strs_to_be_removed() {
  strs_to_be_removed = []
}

function handle_table_selection(row: Row<String>, value: any) 
{
  row.toggleSelected(!row.getIsSelected())
  let str_cell_value: String = row.original
  let index = strs_to_be_removed.findIndex((s) => s == str_cell_value);

  if (row.getIsSelected() == true)
  {
    strs_to_be_removed.splice(index, 1)
    return
  } 

  strs_to_be_removed.push(str_cell_value)
  console.log(strs_to_be_removed)
  return undefined
  
}

export let table_inst: Table<String> | null

export const string_columns: ColumnDef<String>[] = [
  {

      id: "select",
      header: ({ table }) => 
      {
        table_inst = table
        return (
          <></>
        )
      },
      cell: ({ row }) => 
      (
        <div className="inline-flex items-center">
          <label
            className="relative flex items-center rounded-full cursor-pointer"
            htmlFor="custom"
            >
            <input
              type="checkbox"
              checked={row.getIsSelected()}
              onChange={(value) => handle_table_selection(row, value)}
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
      id: "payload_strings",
      cell: ({row}) => 
      {
          let r = row.original
          return (<p className="">{r}</p>)
      }
  },
]

