'use client'

import Image from "next/image";
import { Textarea } from "@/components/ui/textarea"
import { Combobox } from "@/components/ui/combobox";
import { Button } from "@/components/ui/button";
import { DataTable } from "@/components/ui/data_table";
import { string_columns } from "@/components/ui/s_columns";
import { open } from '@tauri-apps/api/dialog'
import React from "react";

async function r_open()
{
  const selected = await open({});
  
}


export default function Home() {
  const [payloadstrs, setPayloadstrs] = React.useState<String[]>([])
  const [payloadopt, setPayloadOpt] = React.useState("Word List")
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
        <div className="grid grid-rows-3 gap-0.5">
          <div></div>
        </div>
      )
    }

    return (<h1>Fell through</h1>)
  
  }
  

  return (
    <main>
      <div className="grid grid-col-2 grid-flow-col gap-4">
        <div className="min-h-full">
          <Textarea></Textarea>
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


