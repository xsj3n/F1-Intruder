'use client'

import Image from "next/image";
import { Textarea } from "@/components/ui/textarea"
import { Combobox } from "@/components/ui/combobox";
import { Button } from "@/components/ui/button";
import { DataTable } from "@/components/ui/data_table";
import { string_columns } from "@/components/ui/s_columns";
import { Input } from "@/components/ui/input"
import { open } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import React, { useEffect, useRef, useState } from "react";
import { Separator } from "@/components/ui/separator"
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";


// reducer needed for this component 


export default function Home() {

  const [serverstate, setServerstate] = useState(false)
  
  if (serverstate == false)
  {
    invoke("start_ipc_server")
    setServerstate(true)
  }


  const [initalr, setInitialr] = useState("")
  const [payloadstrs, setPayloadstrs] = useState<String[]>([])
  const [payloadopt, setPayloadOpt] = useState("Word List")
  const [payloadsignlestr, setPayloadsinglestr] = useState("")
  const [wsocket, setWSocket] = useState(new WebSocket("ws://127.0.0.1:3001"))
  const stringaddref = useRef<HTMLInputElement>(null)
  

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
    if (e.target.value == "") 
    {
      
      return
    }
    setPayloadsinglestr(e.target.value)
  }

  const handlestringaddst = async function () {

    function addthenclear()
    {
      
      payloadstrs.push(payloadsignlestr)
      let data = payloadstrs.slice(0)
      
      setPayloadstrs(data) 
      if (stringaddref.current != null)
      {
        stringaddref.current.value = ""
      }
    }

    if (payloadsignlestr == "" || payloadsignlestr == payloadstrs.slice(-1)[0]) {return}
    addthenclear()
  }

  const clearpayloadstrs = async function ()
  {
    let data: String[] = []
    setPayloadsinglestr("")
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
        <div><Button variant={"outline"}>Remove</Button></div>
        <div className="mt-4 col-span-2"><DataTable columns={string_columns} data={payloadstrs}></DataTable></div>
      </div >
        <div className="flex mt-2">
          <Input ref={stringaddref} onChange={handlestringaddinput} type="Add payload string" placeholder="Add payload string" className="mr-2"></Input>
          <Button variant="outline" onClick={handlestringaddst}>Add </Button>
        </div>
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



  

  
  wsocket.addEventListener("open", e => {console.log("connected via ws!")})
  wsocket.addEventListener("message", e => {console.log("received msg: ", e.data)})
  wsocket.addEventListener("close", e => {console.log("disconnected via ws!")})
  
  
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


