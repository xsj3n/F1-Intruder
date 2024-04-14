'use client'

import Image from "next/image";
import { Textarea } from "@/components/ui/textarea"
import { Combobox } from "@/components/ui/combobox";
import { Button } from "@/components/ui/button";
import { DataTable } from "@/components/ui/data_table";
import { remove_toggled_strs_was_ran, set_remove_toggled_strs_was_ran, strs_to_be_removed, string_columns, clear_strs_to_be_removed } from "@/components/ui/s_columns";
import { Input } from "@/components/ui/input"
import { open } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import React, { LegacyRef, useEffect, useMemo, useRef, useState } from "react";
import { Separator } from "@/components/ui/separator"
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import { HandMetal } from "lucide-react";
import { resolve } from "path";
import { table } from "console";


// reducer needed for this component 

async function sleep(seconds: any)
{
  return new Promise(resolve => setTimeout(resolve, seconds * 1000))
}

async function handle_ws_msg(data: String, ws: WebSocket)
{
 if (data == "PING") 
 { 
    await sleep(10)
    ws.send("PONG")
    console.log("PONG")
  }
 

}


const wsocket = new WebSocket("ws://127.0.0.1:3001")
wsocket.addEventListener("open", e => {console.log("connected via ws!")})
wsocket.addEventListener("message", e =>  handle_ws_msg(e.data, wsocket))
wsocket.addEventListener("close", e => {console.log("disconnected via ws!")})


export default function Home() {

  const [serverstate, setServerstate] = useState(false)
 
  startIPC()
  

  const [initalr, setInitialr] = useState("")
  const [payloadstrs, setPayloadstrs] = useState<String[]>([])
  const [payloadopt, setPayloadOpt] = useState("Word List")
  const [payloadsignlestr, setPayloadsinglestr] = useState("")
  const string_add_ref_inp = useRef<HTMLInputElement>(null)

  async function startIPC()
  {
     
    if (serverstate == false)
    {
      invoke("start_ipc_server")
      setServerstate(true)
    }

  }  
  
  function LabeledSeparator(label: String) : React.JSX.Element
  {
    return (
      <><div><h2>{label}</h2></div><div className="w-1/2 mb-3 mt-1"> <Separator></Separator> </div></>
    )
  }

  const readcache = async function() 
  {
    const path = "/home/xis/Documents/request.txt"
    const content = await readTextFile(path).then((s) => s)
    console.log(content)
    setInitialr(content)


  }

  const wl_open = async function()
  {
    const selected = await open({});

    if (Array.isArray(selected) || selected == null) { return }

    const contents = readTextFile(selected)

    if ((await contents).length == 0) { return }
    let data: String[] = (await contents).split("\n")
    data = data.filter((s) => s.trim() != "")

    if (data.length > 500) { return }
    await setPayloadstrs(data)

  }

  const handlestringaddinput = async function (e:  React.ChangeEvent<HTMLInputElement>)
  {
    e.preventDefault()
    if (e.target.value == "") { return }

    setPayloadsinglestr(e.target.value)

  }

  const handlestringaddst = async function () {

    function addthenclear()
    {
      payloadstrs.push(payloadsignlestr)
      let data = payloadstrs.slice(0)
      
      setPayloadstrs(data)
      if (string_add_ref_inp.current != null) {string_add_ref_inp.current.value = ""}
    }

    if (payloadsignlestr == "" || payloadsignlestr == payloadstrs.slice(-1)[0]) {return}
    addthenclear()
  }

  string_add_ref_inp.current?.addEventListener("keypress", (e) =>
  {
    if (e.key != "Enter") {return}
    e.preventDefault()
    document.getElementById("addinput")?.click()
  })

  const clearpayloadstrs = async function ()
  {
    let data: String[] = []
    set_remove_toggled_strs_was_ran(true)
    setPayloadsinglestr("")
    setPayloadstrs(data)
  }

  const clear_selected_payload_str = async function () 
  {
      //let data: String[] = []
      //let target_indexes: Number[] = []
    
      strs_to_be_removed.map((s): any => {
        payloadstrs.splice(payloadstrs.findIndex((ps) => ps == s), 1)
      })
      let data = payloadstrs.filter((s) => true)
      clear_strs_to_be_removed()
      set_remove_toggled_strs_was_ran(true)
      setPayloadstrs(data)
  }

  function LoadPayloadOpts()
  {
    if (payloadopt == "wordlist")
    {
      
      return (
      <div>
      <div className="grid grid-cols-3 gap-0.5">
        <div><Button variant={"outline"} onClick={wl_open}>Load</Button></div>
        <div><Button variant={"outline"} onClick={clearpayloadstrs}>Clear</Button></div>
        <div><Button variant={"outline"} onClick={clear_selected_payload_str}>Remove</Button></div>
        <div className="mt-4 col-span-2"><DataTable setData={setPayloadstrs} columns={string_columns} data={payloadstrs}></DataTable></div>
      </div >
        <div className="flex mt-2">
          <Input ref={string_add_ref_inp} onChange={handlestringaddinput} type="Add payload string" placeholder="Add payload string" className="mr-2"></Input>
          <Button id="addinput"  variant="outline" onClick={handlestringaddst}>Add </Button>
        </div>
        <script>
        
        </script>
      </div>
      
      )
    }

    if (payloadopt == "numbers")
    {
      return (
        <div className="grid grid-rows-3 gap-4">
          <div className=""><Input type="Start" placeholder="Start"></Input></div>
          <div className=""><Input type="End" placeholder="End"></Input></div>
          <div className=""><Input type="Step" placeholder="Step"></Input></div>
        </div>
      )
    }

    return (<></>)
  }



  

  
  
  readcache()
  return (
    <main>
      <div className="grid grid-col-2 grid-flow-col gap-4">
        <div className="min-h-full">
          <Textarea defaultValue={initalr}></Textarea>
        </div>
        <div className="mt-3 grid-rows-2 gap-5" id="cb">
          {LabeledSeparator("Payload options")}
          <div className="mb-3"> <Combobox setPayloadOpt={setPayloadOpt}></Combobox> </div>
          <div>{LoadPayloadOpts()}</div>
          <div></div>
          <br />
          {LabeledSeparator("Threading options")}
          <div className=""><Input className="w-6/12" type="Number of requests per thread" placeholder="Number of requests per thread"></Input></div>
         
        </div>
      </div>
    </main>
  );
}


