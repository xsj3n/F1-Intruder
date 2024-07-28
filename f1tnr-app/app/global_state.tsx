"use client"
import { invoke } from "@tauri-apps/api/tauri"
export function setGlobalPayloadSrc(payload_sv: String[])
{
    const set = async (p: String[]) => 
        {
            await invoke("set_cached_payloads", {newPayloadSv: p})
        }
    
        set(payload_sv)
}




export function getGlobalPayloadSrc(): Promise<String[]>
{
    const get = async () => 
        {
            let o = invoke<String[]>("get_cached_payloads", {})
            
            return o
        }

    return get()
}


export function setGlobalPayloadFilepath(wordlist_filepath: String)
{
    const set = async (fp: String) => 
    {
        await invoke("set_cached_filepath", {newFilepath: fp})
    }

    set(wordlist_filepath).then(() => {})
}

export async function getGlobalPayloadFilepath(): Promise<String>
{
    const get = async () => 
    {
        let fp: String = await invoke("get_cached_filepath", {})
        return fp
    }
    let fp =  get().then(s =>  {return s}) 
    return fp
}

export function setGlobalHttpBaseRequest(request: String)
{
    const set = async (http_r: String) => 
    {
        await invoke("set_cached_http_request", {newRequest: http_r})
    }

    set(request).then(() => {})
}

export function getGlobalHttpBaseRequest(): Promise<String>
{
    const get = async () => 
        {
            let s: String = await invoke("get_cached_http_request", {})
            return s
        }
        let s =  get()
        return s
}

export function setGlobalThreadNums(threads_n: String)
{
    const set = async (thread_s: String) => 
        {
            await invoke("set_cached_thread_num", {newThreadNumS: thread_s})
        }
    
        set(threads_n)
}


export function getGlobalThreadNums(): Promise<String>
{
    const get = async () => 
        {
            let s: String = await invoke<String>("get_cached_thread_num", {})
            return s
        }
        let s =  get()
        return s
}
