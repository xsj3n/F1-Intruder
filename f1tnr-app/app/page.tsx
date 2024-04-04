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
import React, { useEffect } from "react";




export default function Home() {

  const [initalr, setInitialr] = React.useState("")
  const [payloadstrs, setPayloadstrs] = React.useState<String[]>([])
  const [payloadopt, setPayloadOpt] = React.useState("Word List")



  const readcache = async function()
  {
    const path = "/home/xis/request.txt"
    const content = await readTextFile(path).then((s) => s)
    
    return content

  }

  const r_open = async function()
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
  

  function LoadPayloadOpts()
  {
    if (payloadopt == "wordlist")
    {
      
      
      return (
      <div className="grid grid-cols-3 gap-0.5">
        <div><Button variant={"outline"} onClick={r_open}>Load</Button></div>
        <div><Button variant={"outline"}>Clear</Button></div>
        <div><Button variant={"outline"}>Remove</Button></div>
        <div className="mt-4 col-span-2"><DataTable columns={string_columns} data={payloadstrs}></DataTable></div>
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



  return (
    <main>
      <div className="grid grid-col-2 grid-flow-col gap-4">
        <div className="min-h-full">
          <Textarea onLoad={readcache}></Textarea>
        </div>
        <div className="mt-3 grid-rows-2 gap-5" id="cb">
          <div className="mb-3">
            <Combobox setPayloadOpt={setPayloadOpt}></Combobox>
          </div>
          <div>
            {LoadPayloadOpts()}
          </div>
        </div>
      </div>
    </main>
  );
}

