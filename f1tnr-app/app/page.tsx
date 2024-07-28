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
import hljs from 'highlight.js/lib/core'
import javascript from 'highlight.js/lib/languages/javascript'
import httl_hl from 'highlight.js/lib/languages/http'

import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import React, { LegacyRef, useEffect, useMemo, useRef, useState } from "react";
import { Separator } from "@/components/ui/separator"
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import { AlertCircle, HandMetal } from "lucide-react";

import { GoArrowLeft, GoArrowRight } from "react-icons/go";
import { setGlobalHttpBaseRequest, setGlobalPayloadFilepath, setGlobalPayloadSrc, setGlobalThreadNums } from "./global_state";
import { sleep } from "./sleep";
import { Hlta } from "@/components/ui/highlighted-textarea";


hljs.registerLanguage("javascript", javascript)
hljs.registerLanguage("http", httl_hl)




export default function Home() {
  invoke("empty_cache_dir", {})

  const [initalr, setInitialr] = useState("")
  const [payloadstrs, setPayloadstrs] = useState<String[]>([])
  const [payloadopt, setPayloadOpt] = useState("Word List")
  const [payloadsignlestr, setPayloadsinglestr] = useState("")
  
  const string_add_ref_inp = useRef<HTMLInputElement>(null)
  const start_number_input = useRef<HTMLInputElement>(null)
  const end_number_input = useRef<HTMLInputElement>(null)
  const step_number_input = useRef<HTMLInputElement>(null)
  const thread_number_input = useRef<HTMLInputElement>(null)
  const text_area_ref = useRef<HTMLDivElement>(null)
  
  useEffect(() => {

    const handle_enter_keypress = (e: KeyboardEvent) =>
    {
      if (e.key != "Enter") {return}
      e.preventDefault()
      document.getElementById("addinput")?.click()
    }
    if (string_add_ref_inp.current != null)
    {
      string_add_ref_inp.current.addEventListener("keypress", handle_enter_keypress)
    }

    return(() =>
    {
      string_add_ref_inp.current?.removeEventListener("keypress", handle_enter_keypress)
    })
  },[string_add_ref_inp.current])

  useEffect(() => 
  {




    
    const fetch_request = async () =>
    {
      invoke("unlock_net_engine", {})
      let initr: string  = await invoke("parse_burp_file", {})
      setInitialr(initr)
    }
    
    fetch_request().catch(console.error)

    return (() =>
    {
      setInitialr("")
      setPayloadstrs([])
      setPayloadOpt("")
      setPayloadsinglestr("")
    })

  }, [])



  const error_cache_memo: String = useMemo(() => 
  {

    switch (payloadopt)
    {
      case "numbers":
        if (start_number_input.current?.value != "" || end_number_input.current?.value  != "")
        {
          return "Ready"
        }
        console.error("Number fields not filled")
        return "Start and End fields must be filled out."

      case "wordlist": 
        if (payloadstrs.length != 0)
        {
          return "Ready"

        }

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
    setGlobalPayloadFilepath(selected)

    const contents = readTextFile(selected)

    if ((await contents).length == 0) { return }
    let data: String[] = (await contents).split("\n")
    data = data.filter((s) => s.trim() != "")

   
    setPayloadstrs(data)

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


  const clearpayloadstrs = async function ()
  {
    let data: String[] = []
    setGlobalPayloadSrc([" "])
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
      setGlobalPayloadSrc(payloadstrs)
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

    function turn_nums_to_num_array(): boolean
    {
      let start = Number(start_number_input.current?.value)
      let end   = Number(end_number_input.current?.value)
      let step  = Number(step_number_input.current?.value)

      if (isNaN(start) || isNaN(end)) 
      { 
        modal_peep("Number fields must be populated with numbers")
        return false
      }


      if (isNaN(step) || step == 0)
      {
        step = 1
      }

      let arr = [...Array(end).keys()]
      .map((i) => i + step)
      .filter((i) => i > start)
      .map((i) => i.toString())


      console.log("PSLR ARR", arr.unshift(start.toString()) )
      setGlobalPayloadSrc(arr)

      return true
    }

    switch (error_cache_memo)
    {

      case "Start and End fields must be filled out.":
        modal_peep(error_cache_memo)

      case  "Add at least one string to the payload table or load strings from a line-breaked file.":
        modal_peep(error_cache_memo)

      case "Ready":
        if (text_area_ref.current == null)
        {
          modal_peep("Request area empty")
          return
        }


        if (payloadopt == "wordlist") 
        {
          setGlobalPayloadSrc(payloadstrs)
        } else
        {
          turn_nums_to_num_array()
        }


        if (thread_number_input.current != null && !isNaN(Number(thread_number_input.current.value)))
        {
          setGlobalThreadNums(thread_number_input.current.value)
        }

        
        
        
        let run_button_node = document.getElementById("run_btn")
        if (!run_button_node)
        {
          console.error("[!] Run button vnode is null")
          return

        }

        console.log("Request:\n", text_area_ref.current.innerText)
        setGlobalHttpBaseRequest(text_area_ref.current.innerText)
        run_button_node.click()


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

    let sel = window.getSelection()
    if (!sel)
    {
      return
    }
    if (!sel.rangeCount)
    {
      return 
    }
    
    let range = sel.getRangeAt(0)
    range.deleteContents()
    range.insertNode(document.createTextNode("†‡"))
    

  }

  // <Textarea ref={text_area_ref} defaultValue={initalr}></Textarea>

  if (initalr.length == 0)
  {
      return (<><h2>Loading...</h2></>)
  }



  5
  return (

    <main className="h-full">
      <div className="flex gap-4 h-full">
        <Hlta className="w-1/2" ref={text_area_ref} text={initalr} height="90vh" width=""></Hlta>
        <div className="mt-3 grid-rows-2 gap-5" id="cb">
          {LabeledSeparator("Add delimiters")}
          <div className="flex mb-4">
            <div className="mr-4"><Button variant="outline" onClick={() => handle_cross_add(0)}>Add †‡</Button></div>
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
          <div className=""><Input className="w-6/12" type="Number of requests per thread" placeholder="Number of requests per thread" ref={thread_number_input}></Input></div>
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



