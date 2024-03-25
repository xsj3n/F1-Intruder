
import Image from "next/image";
import { Textarea } from "@/components/ui/textarea"
import { Combobox } from "@/components/ui/combobox";


export default function Home() {
  return (
    <main>
      <div className="grid grid-rows-2 grid-flow-col gap-1">
        <div className="min-h-full row-span-2"><Textarea></Textarea></div>
        <div className=""><Combobox></Combobox></div>
      </div>
    </main>
  );
}
