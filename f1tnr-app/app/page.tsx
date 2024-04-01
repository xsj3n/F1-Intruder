'use client'

import Image from "next/image";
import { Textarea } from "@/components/ui/textarea"
import { Combobox } from "@/components/ui/combobox";
import { Button } from "@/components/ui/button";
import React from "react";





export default function Home() {
  const [payloadopt, setPayloadOpt] = React.useState("Word List")
  function LoadPayloadOpts()
  {
    if (payloadopt == "wordlist")
    {
      return (
      <div className="grid grid-cols-3 gap-0.5">
        <div><Button variant={"outline"}>Load</Button></div>
        <div><Button variant={"outline"}>Clear</Button></div>
        <div><Button variant={"outline"}>Remove</Button></div>
      </div>
      
      
      )
    }
  
    if (payloadopt == "numbers")
    {
      return (<h1>Numbers</h1>)
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


