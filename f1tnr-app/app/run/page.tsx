"use client"
import { useEffect, useState } from "react";
import { getGlobalHttpBaseRequest, setGlobalPayloadSrc, getGlobalThreadNums, getGlobalPayloadSrc, getGlobalPayloadFilepath } from "../global_state";
import RunInner from "@/components/run";
import { sleep } from "../sleep";
import { error } from "console";
import { invoke } from "@tauri-apps/api/tauri";







export default function Run()
{
 


  const get_fp = async () => 
  {
    let fp: string = await invoke("get_cached_filepath", {})
    console.log(fp)
    setFilePath(fp)
  }

  const get_pl = async () => 
  {
    let o = await invoke<string[]>("get_cached_payloads", {})
    setPayloads(o)
    return o
  }

  const get_htr = async () => 
  {
    let s: string = await invoke("get_cached_http_request", {})
    setRequest(s)
    return s
  }


  const [file_path, setFilePath] = useState<string>("")
  const [payloads, setPayloads] = useState<string[]>([])
  const [thread_n, setThreadN] = useState<string>("")
  const [request, setRequest]= useState<string>("")
  useEffect(() =>
  {
        
    get_fp()
    get_pl()
    get_htr()
    


  
  }, [])

    

  if (request.length == 0)
    {
      return(
        <>
          <h1>Loading...</h1>
        </>
        )
    }
 

  return(
    <>
    <div className="grid grid-rows-1">
      
      <RunInner payloads={payloads} filepath={file_path} thread_num={thread_n} request={request}></RunInner>
    </div>
        
    </>
    )
}

