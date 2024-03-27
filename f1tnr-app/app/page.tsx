
import Image from "next/image";
import { Textarea } from "@/components/ui/textarea"
import { Combobox } from "@/components/ui/combobox";
import { Button } from "@/components/ui/button";


export default function Home() {
  //const [payloadopt, setPayloadOpt] = React.useState(())


  return (
    <main>
      <div className="grid grid-col-2 grid-flow-col">
        <div className="min-h-full"><Textarea></Textarea></div>
        <div className="mt-2"><Combobox></Combobox></div>
        <div>
        <Button variant="outline"></Button>
        <Button variant="outline"></Button>
        </div>
      </div>
    </main>
  );
}


