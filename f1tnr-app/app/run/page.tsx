"use client"

import HttpTable from "@/components/httptable";
import { DataTable } from "@/components/ui/data_table";
import { Progress } from "@/components/ui/progress";
import { http_columns } from "@/components/ui/s_columns";
import { Textarea } from "@/components/ui/textarea";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useMemo, useRef, useState } from "react";
import { payload_src } from "../page";





export async function sleep(seconds: any)
{
  return new Promise(resolve => setTimeout(resolve, seconds * 1000))
}
async function handle_ws_msg(data: String, ws: WebSocket | null)
{
    if (ws == null) {return}
    if (data == "PING") 
    { 
        await sleep(10)
        ws.send("PONG")
        console.log("PONG")
    }
    

}

invoke("start_ipc_server")
const wsocket = new WebSocket("ws://127.0.0.1:3001")
wsocket.addEventListener("open", e => {console.log("connected via ws!")})
wsocket.addEventListener("message", e =>  handle_ws_msg(e.data, wsocket))
wsocket.addEventListener("close", e => {console.log("disconnected via ws!")})

export default function Run()
{
    const permutation_request_ran = useRef(false)
    useEffect(() => 
    {
        
        if (payload_src == null || permutation_request_ran.current == true) { return }

        console.log("Preparing to communicate with IPC home")
        if ( typeof payload_src[1] ==  "string")
        {
            wsocket.send("PERMUTATE-S†" + payload_src.join("†"))  
        } else
        {
            let num_payload_indicators = "PERMUTATE-N†"
            payload_src.map((n) => num_payload_indicators = num_payload_indicators + n.toString() + "†")
            
            wsocket.send(num_payload_indicators)   
        }
        
        permutation_request_ran.current = true 
    }, [])

    return(
        <div className="grid grid-rows-1">
            <div className=""><HttpTable></HttpTable></div>
            <div><Progress></Progress></div>
            <div className="flex h-1/2 ">
                <div className="w-1/2"><Textarea></Textarea></div>
                <div className="w-1/2"><Textarea></Textarea></div>
            </div>
        </div>
    )
}

