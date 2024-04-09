import HttpTable from "@/components/httptable";
import { DataTable } from "@/components/ui/data_table";
import { Progress } from "@/components/ui/progress";
import { http_columns } from "@/components/ui/s_columns";
import { Textarea } from "@/components/ui/textarea";



export default function run()
{
    return(
        <div className="grid grid-rows-1">
            <div className=""><HttpTable></HttpTable></div>
            <div><Progress></Progress></div>
            <div className="flex h-1/2 ">
                <div className="w-1/2"><Textarea></Textarea></div>
                <div className="w-1/2"><Textarea></Textarea></div>
            </div>
        </div>
    )
}

