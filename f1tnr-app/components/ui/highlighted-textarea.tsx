"use client"

import { request } from "http"
import React from "react"

export function color_request(request: String)
{

  
  let colored_lines: React.JSX.Element[] = []
  let header_only: string = request.split("\r\n\r\n")[0]
  let lines: string[] = header_only.split("\r\n")
  for (let i = 0; i < lines.length; i++)
  {
    if (i == 0)
      {
        let verb_path_version: string[] = lines[i].split(" ")
        colored_lines.push((<span className="text-orange-300">{verb_path_version[0]} </span>))
        colored_lines.push((<span>{verb_path_version[1]} </span>))
        colored_lines.push((<><span className="text-orange-300">{verb_path_version[2]}</span><br /></>))
        continue
      }

    let space_split_line: string[] = lines[i].split(" ")
    let remaining_text: string[] = space_split_line.slice(1)
    colored_lines.push((<><span className="text-emerald-300">{space_split_line[0]}</span> {remaining_text.map(text => text)}<br /></>))
    
  }

  return (<code>
    {colored_lines.map(l => l)}
  </code>)

}



interface HlTxt
{
    className: string,
    text: string,
    width: string
    height: string
}

export const Hlta = React.forwardRef(function HighlightedTextArea({ text, className, width, height } : HlTxt, ref: React.ForwardedRef<HTMLDivElement>)
{

  if (!text)
  {
    return 
  }

  if (ref == null)
  {
    return(
      <div className={className + " overflow-y-scroll overflow-x-scroll outline-none"} style={{height: height}}contentEditable="true"><p className="text-wrap text-sm">{color_request(text)}</p></div>
  )
  }
    return(
        <div ref={ref} className={className + " overflow-y-scroll overflow-x-scroll outline-none"}style={{height: height, width: width}}contentEditable="true"><p className="text-wrap text-sm">{color_request(text)}</p></div>
    )
})