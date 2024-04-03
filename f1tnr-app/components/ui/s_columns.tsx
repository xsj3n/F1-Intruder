"use client"

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
        accessorKey: "payload",
        header: "Payload",
        cell: ({row}) => 
        {
           return (<>{row}</>)
        }
    }
]