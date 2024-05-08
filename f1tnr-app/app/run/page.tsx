"use client"

import { DataTable } from "@/components/ui/data_table";
import { Progress } from "@/components/ui/progress";
import { HttpData, http_columns } from "@/components/ui/s_columns";
import { Textarea } from "@/components/ui/textarea";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useMemo, useRef, useState } from "react";
import { file_path, http_request, payload_src } from "../page";





export async function sleep(seconds: any)
{
  return new Promise(resolve => setTimeout(resolve, seconds * 1000))
}



export default function Run()
{
    const [HttpRespData, setHttpRespData] = useState([])
    const async_engine_started_ref = useRef<boolean | null>(null);
    useEffect(() =>
    {
      
      if (async_engine_started_ref.current != null) { return }
  
      invoke("start_async_engine", {httpRequest: http_request, permutationFilepath: "../async_net_engine/wordlist-0-1.txt"})
      async_engine_started_ref.current = true;
  
    }, [])
 
    return(
        <div className="grid grid-rows-1">
            <div className=""><DataTable columns={http_columns} data={HttpRespData} cn={"w-full"}></DataTable></div>
            <div><Progress></Progress></div>
            <div className="flex h-1/2 ">
                <div className="w-1/2"><Textarea></Textarea></div>
                <div className="w-1/2"><Textarea></Textarea></div>
            </div>
        </div>
    )
}

