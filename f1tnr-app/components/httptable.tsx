"use client"
import { useState } from "react"
import { HttpData, http_columns } from "./ui/s_columns"
import { DataTable } from "./ui/data_table"

 

export default function HttpTable()
{
    const [httpdata, setHttpdata] = useState<HttpData[]>([])

    return(
        <DataTable columns={http_columns} data={httpdata}></DataTable>
    )
}
