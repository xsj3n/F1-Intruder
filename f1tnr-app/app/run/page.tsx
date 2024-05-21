"use client"

import { DataTable, HttpTable } from "@/components/ui/data_table";
import { Progress } from "@/components/ui/progress";
import { HttpData, http_columns } from "@/components/ui/s_columns";
import { Textarea } from "@/components/ui/textarea";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useMemo, useRef, useState } from "react";
import { file_path, http_request, payload_src, thread_num } from "../page";
import { listen, emit } from "@tauri-apps/api/event";







export async function sleep(seconds: any)
{
  return new Promise(resolve => setTimeout(resolve, seconds * 1000))
}


interface HttpDataObject
{
  http_request : HttpRequestObject,
  http_response: HttpResponseObject
}

interface HttpRequestObject
{
  method : String,
  path   : String,
  version: Number,
  full_request_string : String
  id : Number
}

interface HttpResponseObject
{
  full_response_string: String
  length : Number,
  status_code : Number,
  status_string : String,


}

export default function Run()
{
    console.log("Permutations to send: ", payload_src?.length)
    const [HttpRespData, setHttpRespData] = useState<HttpData[]>([])
    const http_data_buffer = useRef<HttpData[]>([])
    const known_ids = useRef<Number[]>([])
    const progress = useRef<Number>(0)
    const setHttpData = (data: HttpData[]) =>
    {
      http_data_buffer.current = data
      setHttpRespData(data)
    } 

    const http_request_textarea_ref = useRef<HTMLTextAreaElement>(null)
    const http_response_textarea_ref = useRef<HTMLTextAreaElement>(null)
    const setRequestResponseTextarea = (request: String, response: String) => 
    {
      if (http_request_textarea_ref.current != null && http_response_textarea_ref.current != null)
      {
        http_request_textarea_ref.current.value = request.valueOf()
        http_response_textarea_ref.current.value = response.valueOf()
      }
    }

    
    useEffect(() => 
      {
        emit("ready", {theMessage: "ready"})  
        invoke("start_async_engine", {httpRequest: http_request, permutationFilepath: "../../async_net_engine/wordlist-0-1.txt", threadsNum: thread_num})
        
        const unlisten_poll = listen("data-poll", (e) => 
          {
            invoke("refresh_datadir_cache", {})
            invoke("check_for_new_http_data", {})
          }) 
        const unlisten = listen('http-data', (event) =>
          {
            let hdr = event.payload as HttpDataObject
            if (hdr.http_request.full_request_string.length == 0) { return }
            if (known_ids.current.includes(hdr.http_request.id))  { return }
            known_ids.current.push(hdr.http_request.id)

            let payload_arr = payload_src as String[]
            const payload: String = payload_arr[hdr.http_request.id as number]
            const data: HttpData = 
            {
              id: hdr.http_request.id,
              payload: payload, 
              status_code: hdr.http_response.status_code,
              status_string: hdr.http_response.status_string,
              length: hdr.http_response.length,
              request: hdr.http_request.full_request_string,
              response: hdr.http_response.full_response_string
            }

           
            if (http_data_buffer.current.includes(data, 0)) {return }

            console.log("Pushing more data onto array. Current len: ", http_data_buffer.current.length)
            http_data_buffer.current.push(data)
            progress.current = progress.current as number + 1
            console.log("Buffer indexes: ", http_data_buffer.current.length)
            setHttpData([...http_data_buffer.current].sort((a,b) => a.id.valueOf() - b.id.valueOf()) )
            
          })

        return (() =>
        {
          unlisten.then(u => u())
          unlisten_poll.then((u => u()))
        })
      }, [])

    
 
    return(
        <div className="grid grid-rows-1">
            <HttpTable columns={http_columns} data={HttpRespData} cn={"w-full h-96 overflow-y-scroll"} sethr={setRequestResponseTextarea}></HttpTable>
            <div><Progress value={progress.current.valueOf()} max={payload_src?.length}></Progress></div>
            <div className="flex h-1/2 ">
                <div className="w-1/2"><Textarea ref={http_request_textarea_ref}></Textarea></div>
                <div className="w-1/2"><Textarea ref={http_response_textarea_ref}></Textarea></div>
            </div>
            < ></>
        </div>
    )
}

