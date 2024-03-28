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
      <div>
        <Button variant={"outline"}>Load</Button>
        <Button variant={"outline"}>Clear</Button>
        <Button variant={"outline"}>Remove</Button>
      </div>)
    }
  
    if (payloadopt == "numbers")
    {
      return (<h1>Numbers</h1>)
    }

    return (<h1>Fell through</h1>)
  
  }
  

  return (
    <main>
      <div className="grid grid-col-2 grid-flow-col">
        <div className="min-h-full">
          <Textarea></Textarea>\
        </div>
        <div className="mt-2" id="cb">
          <Combobox setPayloadOpt={setPayloadOpt}></Combobox>
        </div>
        {LoadPayloadOpts()}
      </div>
    </main>
  );
}


