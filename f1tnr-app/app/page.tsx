'use client'

import Image from "next/image";
import { Textarea } from "@/components/ui/textarea"
import { Combobox } from "@/components/ui/combobox";
import { Button } from "@/components/ui/button";
import { DataTable } from "@/components/ui/data_table";
import { remove_toggled_strs_was_ran, set_remove_toggled_strs_was_ran, strs_to_be_removed, string_columns, clear_strs_to_be_removed } from "@/components/ui/s_columns";
import { Input } from "@/components/ui/input"
import { open } from '@tauri-apps/api/dialog'
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"

import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import React, { LegacyRef, useEffect, useMemo, useRef, useState } from "react";
import { Separator } from "@/components/ui/separator"
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import { AlertCircle, HandMetal } from "lucide-react";
import { resolve } from "path";
import { table } from "console";
import { sleep, wsocket } from "./run/page";







export default function Home() {
  

  const [initalr, setInitialr] = useState("")
  const [payloadstrs, setPayloadstrs] = useState<String[]>([])
  const [payloadopt, setPayloadOpt] = useState("Word List")
  const [payloadsignlestr, setPayloadsinglestr] = useState("")
  
  const string_add_ref_inp = useRef<HTMLInputElement>(null)
  const start_number_input = useRef<HTMLInputElement>(null)
  const end_number_input = useRef<HTMLInputElement>(null)
  const step_number_input = useRef<HTMLInputElement>(null)
  const text_area_ref = useRef<HTMLTextAreaElement>(null)

  const error_cache_memo: String = useMemo(() => 
  {

    switch (payloadopt)
    {
      case "numbers":
        if (start_number_input.current?.value != "" || end_number_input.current?.value  != "") {return "None"}
        //modal_peep("Start and End fields must be filled out.")
        return "Start and End fields must be filled out."

      case "wordlist": 
        if (payloadstrs.length != 0) {return "None"}
        return "Add at least one string to the payload table or load strings from a line-breaked file."
      
        default: 
        return "None"
    }
    
  }, [payloadopt, payloadstrs, start_number_input.current?.value, end_number_input.current?.value])
  
  function LabeledSeparator(label: String) : React.JSX.Element
  {
    return (
      <><div><h2>{label}</h2></div><div className="w-1/2 mb-3 mt-1"> <Separator></Separator> </div></>
    )
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
      strs_to_be_removed.map((s): any => 
      {
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
        <div className="mt-4 col-span-2"><DataTable columns={string_columns} data={payloadstrs}></DataTable></div>
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
          <div className=""><Input ref={start_number_input} type="Start" placeholder="Start"></Input></div>
          <div className=""><Input ref={end_number_input} type="End" placeholder="End"></Input></div>
          <div className=""><Input ref={step_number_input} type="Step" placeholder="Step"></Input></div>
        </div>
      )
    }

    return (<></>)
  }


  const readcache = async function() 
  {
    if (initalr != "") {return}
    const path = "/home/xis/Documents/request.txt"
    const content = await readTextFile(path).then((s) => s)
    console.log(content)
    setInitialr(content)
  }



  readcache()

  async function handle_run()
  {
    async function modal_peep(error: String)
    {
      const dialog: HTMLDialogElement | null = document.getElementById("notifcation_modal") as HTMLDialogElement
      dialog?.showModal()
      await sleep(4)
      dialog?.close()
    }

    switch (error_cache_memo)
    {
      case "None":
        permutate_strings_and_run()
      case "Start and End fields must be filled out.":
        modal_peep(error_cache_memo)
      case  "Add at least one string to the payload table or load strings from a line-breaked file.":
        modal_peep(error_cache_memo)
      default:
    }

  }

  function permutate_strings_and_run()
  {
    wsocket?.send(payloadstrs.join("\n"))
    document.getElementById("run_btn")?.click()
  }

  function AlertDestructive() {
    return (

        <Alert className="" variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertTitle>Error</AlertTitle>
          <AlertDescription>
            {error_cache_memo}
          </AlertDescription>
      </Alert>


    )
  }

 function handle_cross_add(kind: number)
 {
    text_area_ref.current?.focus()
    let start = text_area_ref.current?.selectionStart 
    let end = text_area_ref.current?.selectionEnd 
    let chars = ["†", "‡"]

    if (start == undefined || end == undefined ) {return}

    text_area_ref.current?.setRangeText( chars[kind], start , end)
    return undefined
  }


  return (
    <main>
      <div className="grid grid-col-2 grid-flow-col gap-4">
        <div className="min-h-full">
          <Textarea ref={text_area_ref} defaultValue={initalr}></Textarea>
        </div>
        <div className="mt-3 grid-rows-2 gap-5" id="cb">
          {LabeledSeparator("Add delimiters")}
          <div className="flex mb-4">
            <div className="mr-4"><Button variant="outline" onClick={() => handle_cross_add(0)}>Add †</Button></div>
            <div className=""><Button variant="outline" onClick={() => handle_cross_add(1)}>Add ‡</Button></div>
          </div>
          {LabeledSeparator("Payload options")}
          <div className="mb-3"> <Combobox setPayloadOpt={setPayloadOpt}></Combobox> </div>
          <div>{LoadPayloadOpts()}</div>
          <div></div>
          <br />
          {LabeledSeparator("Threading options")}
          <div className=""><Input className="w-6/12" type="Number of requests per thread" placeholder="Number of requests per thread"></Input></div>
          <br />
         <div className=""><Button className="w-1/2" variant={"outline"} onClick={handle_run}>Run</Button></div>
         <dialog id="notifcation_modal">
          {AlertDestructive()}
          </dialog>
        </div>
      </div>

    </main>
  );
}



