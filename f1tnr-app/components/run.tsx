"use client"

import { HttpTable } from "@/components/ui/data_table";
import { Progress } from "@/components/ui/progress";
import { HttpData, http_columns } from "@/components/ui/s_columns";
import { invoke } from "@tauri-apps/api/tauri";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import { listen} from "@tauri-apps/api/event";

import { Hlta } from "./ui/highlighted-textarea";


interface HttpDataObject
{
  http_request : HttpRequestObject,
  http_response: HttpResponseObject
}

interface HttpRequestObject
{
  method : string,
  path   : string,
  version: number,
  full_request_string : string
  id : number
}

interface HttpResponseObject
{
  full_response_string: string
  length : string,
  status_code : number,
  status_string : string,


}

interface ExecuteInfo
{
  payloads: string[],
  filepath: string, 
  thread_num: string, 
  request: string 
}



export default function RunInner({payloads, filepath, thread_num, request}: ExecuteInfo)
{
    const [HttpRespData, setHttpRespData] = useState<HttpData[]>([])
    const [http_rr, setHttpRR] = useState<string[]>([])  

    const http_data_buffer = useRef<HttpData[] | null>([])
    const known_ids = useRef<number[] | null>([])
    const request_ref = useRef<HTMLDivElement | null>(null)
    const response_ref = useRef<HTMLDivElement | null>(null)
    const ws_ref = useRef<WebSocket | null>(null)




    const setRequestResponseHighlighted = useCallback((request: string, response: string) => 
    {
      let data: string[] = [request, response]
      setHttpRR(data)
    }, [])


    
    useEffect(() => 
      {
        let default_fp = "/tmp/f1_pslr/plsr.dat"
        if (filepath.length) 
        {
          default_fp = filepath
        }
        
        const ws = new WebSocket("ws://127.0.0.1:9005")

        function ws_re_open_handler()
        {
          ws_ref.current = new WebSocket("ws://127.0.0.1:9005")
        }
  
        
        function ws_on_open_handler()
        {
          if (ws_ref.current != null)
          {
              ws_ref.current.send("ping")
          }
        }
        
        async function ws_msg_handler(event: MessageEvent)
        {
          if (ws_ref.current == null) { return }
          if ( ws_ref.current.readyState != 1) { return }

          if (event.data == "ping")
          {
            ws_ref.current.send("pong")
            console.log("ws heart-beat")
            return
          } 

          console.log(event.data)
          const http_data:  HttpDataObject = JSON.parse(event.data)
          const  data: HttpData =
          {
            id: http_data.http_request.id,
            payload: payloads[http_data.http_request.id],
            status_code: http_data.http_response.status_code,
            status_string: http_data.http_response.status_string,
            length: http_data.http_response.length,
            request: http_data.http_request.full_request_string,
            response: http_data.http_response.full_response_string
          }

          async function set_httpdata_async()
          {
            setHttpRespData((prevHttpData) => [...prevHttpData, ...[data]]
            .sort((a,b) => a.id - b.id))

            return
          }

          
          set_httpdata_async()
          return
            
        }

        ws_ref.current = ws
        ws_ref.current.addEventListener("close", ws_re_open_handler)
        ws_ref.current.addEventListener("open", ws_on_open_handler)
        ws_ref.current.addEventListener("message", ws_msg_handler)
      

        
        invoke("start_async_engine", {httpRequest: request, permutationFilepath: default_fp, threadsNum: thread_num}).catch(console.error)
        
        const unlisten_poll = listen("data-poll", (e) => 
        {
            invoke("refresh_datadir_cache", {})
            invoke("check_for_new_http_data", {})
        }) 

        console.log(payloads.length)
        
        return (() =>
        {
          //unlisten.then(u => u())
          unlisten_poll.then((u => u()))

          http_data_buffer.current = null
          known_ids.current = null
          request_ref.current = null
          response_ref.current = null 
          

          

          ws_ref.current?.close()
          ws_ref.current?.removeEventListener("message", ws_msg_handler)
          ws_ref.current?.removeEventListener("open", ws_on_open_handler)
          ws_ref.current?.removeEventListener("close", ws_re_open_handler)
          ws_ref.current = null

          setHttpRR([])
          setHttpRespData([])
          

          
          
        
        })
      }, [])

 
    return(
      <>
      <div className="flex flex-col ">
        <div className="overflow-y-scroll h-96">
          <HttpTable columns={http_columns} data={HttpRespData} cn={"w-full"} sethr={setRequestResponseHighlighted}></HttpTable>
        </div >
        <div><Progress></Progress></div>
        <div className="flex">
          <Hlta className="" ref={request_ref} text={http_rr[0]} height="60vh" width="45vh"></Hlta>
          <Hlta className="" ref={response_ref} text={http_rr[1]} height="60vh" width="45vh"></Hlta>
        </div>
      </div>
      </>

    )
}

