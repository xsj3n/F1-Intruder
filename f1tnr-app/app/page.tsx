'use client'

import Image from "next/image";
import { Textarea } from "@/components/ui/textarea"
import { Combobox } from "@/components/ui/combobox";
import { Button } from "@/components/ui/button";
import { DataTable } from "@/components/ui/data_table";
import { remove_toggled_strs_was_ran, set_remove_toggled_strs_was_ran, strs_to_be_removed, string_columns, clear_strs_to_be_removed, table_inst } from "@/components/ui/s_columns";
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
import { sleep} from "./run/page";
import { GoArrowLeft, GoArrowRight } from "react-icons/go";




export let payload_src: String[] | number[] | null = null
export let file_path: String | null = null
export let http_request: String = ""

export default function Home() {
  

  const [initalr, setInitialr] = useState("")
  const [payloadstrs, setPayloadstrs] = useState<String[]>([])
  const payload_strings_ref = useRef<String[]>()
  const [payloadopt, setPayloadOpt] = useState("Word List")
  const [payloadsignlestr, setPayloadsinglestr] = useState("")
  
  const string_add_ref_inp = useRef<HTMLInputElement>(null)
  const start_number_input = useRef<HTMLInputElement>(null)
  const end_number_input = useRef<HTMLInputElement>(null)
  const step_number_input = useRef<HTMLInputElement>(null)
  const text_area_ref = useRef<HTMLTextAreaElement>(null)
  
  let lock_files_removed = false 
  useEffect(() => 
  {
    console.log("Unlocking...")
    invoke("unlock_net_engine", {}).then(() => {})
    if (text_area_ref.current != null)
    {
      text_area_ref.current.value = invoke("parse_burp_file", {}).then(s => s) as unknown as string
    }
    

  }, [])



  const error_cache_memo: String = useMemo(() => 
  {

    switch (payloadopt)
    {
      case "numbers":
        if (start_number_input.current?.value != "" || end_number_input.current?.value  != "") {return "None"}
        console.error("Number fields not filled")
        return "Start and End fields must be filled out."

      case "wordlist": 
        if (payloadstrs.length != 0) {return "None"}
        console.error("Payload strings are empty")
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
    file_path = selected

    const contents = readTextFile(selected)

    if ((await contents).length == 0) { return }
    let data: String[] = (await contents).split("\n")
    data = data.filter((s) => s.trim() != "")

   
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
    payload_src = null
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
      payload_src = payloadstrs
      setPayloadstrs(data)
  }

  function LoadPayloadOpts()
  {
    if (payloadopt == "wordlist")
    {
      
      return (
      <div>
      <div className="grid grid-cols-3">
        <div className="w-1/2"><Button variant={"outline"} onClick={wl_open}>Load</Button></div>
        <div><Button variant={"outline"} onClick={clearpayloadstrs}>Clear</Button></div>
        <div><Button variant={"outline"} onClick={clear_selected_payload_str}>Remove</Button></div>
        <div className="mt-4 col-span-2"><DataTable columns={string_columns} data={payloadstrs} cn={"w-72"}></DataTable></div>
      </div >
        <div className="flex mt-2">
          <Input ref={string_add_ref_inp} onChange={handlestringaddinput} type="Add payload string" placeholder="Add payload string" className="mr-2"></Input>
          
          <Button id="addinput"  variant="outline" onClick={handlestringaddst} className="mr-1">Add</Button>
          <Button variant="outline" onClick={() => table_inst?.previousPage()}  className="mr-1">
            <GoArrowLeft></GoArrowLeft>
            </Button>
            <Button variant="outline" onClick={() => table_inst?.nextPage()} >
              <GoArrowRight></GoArrowRight>
            </Button>
        </div>
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



  async function handle_run()
  {
    async function modal_peep(error: String)
    {
      if (error_cache_memo == "None") {return}

      const dialog: HTMLDialogElement | null = document.getElementById("notifcation_modal") as HTMLDialogElement
      dialog?.showModal()
      await sleep(2)
      dialog?.close()
    }

    async function turn_nums_to_num_array()
    {
      let start = Number(start_number_input.current?.value)
      let end = Number(end_number_input.current?.value)
      let step = Number(step_number_input.current?.value)

      if (isNaN(start) || isNaN(end)) 
      { 
        modal_peep("Number fields must be populated with numbers")
        return
      }

      let num_payload_indicators = []

      num_payload_indicators.push(start)
      num_payload_indicators.push(end)
      if (!isNaN(step)) { num_payload_indicators.push(step)}

      payload_src = num_payload_indicators

      return 
    }

    switch (error_cache_memo)
    {
      case "None":
        if (payloadopt == "wordlist") 
        {
          payload_src = payloadstrs
        } else 
        {
          turn_nums_to_num_array()
        }

        if (text_area_ref.current == null) 
        {
          console.error("text area ref is null, for some reason")
          return
        }  
        console.log("Permuations: ", payloadstrs)
        http_request = text_area_ref.current.value   //src: trust me bro
        document.getElementById("run_btn")?.click()

      case "Start and End fields must be filled out.":
        modal_peep(error_cache_memo)

      case  "Add at least one string to the payload table or load strings from a line-breaked file.":
        modal_peep(error_cache_memo)
    }

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
          <div className="mb-3">
             <Combobox setPayloadOpt={setPayloadOpt}></Combobox>
          </div>
          <div>
            {LoadPayloadOpts()}
          </div>
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



