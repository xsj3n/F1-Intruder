"use client"

import { Checkbox } from "@radix-ui/react-checkbox"
import { ColumnDef } from "@tanstack/react-table"



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
          <Checkbox
            checked={
              table.getIsAllPageRowsSelected() ||
              (table.getIsSomePageRowsSelected() && "indeterminate")
            }
            onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
            aria-label="Select all"
          />
        ),
        cell: ({ row }) => (
          <Checkbox
            checked={row.getIsSelected()}
            onCheckedChange={(value) => row.toggleSelected(!!value)}
            aria-label="Select row"
          />
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