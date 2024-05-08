"use client"

import { use, useState } from "react"
import { Button } from "./ui/button"
import { GoArrowLeft, GoArrowRight } from "react-icons/go"
import Link from "next/link"
import { usePathname, useRouter } from "next/navigation"



export enum CurrentPage
{
  options,
  run
}

export default function Navbuttonclientwrapper({})
{
    const router = useRouter()
    const path = usePathname()

    function navbuttonhandler()
    {
        if (path == "/")
        {
            router.push("/run")
            return

        }
        
        router.push("/")
        return
    }

    function definenavbutton()
    {
        if (path == "/")
        {
            return(<GoArrowRight></GoArrowRight>)
        }

        return(<GoArrowLeft></GoArrowLeft>)
    }
    return(
    <div className="mr-4">
        <Button id="run_btn" variant="outline" onClick={navbuttonhandler}>{definenavbutton()}</Button>
        </div>
    )
}