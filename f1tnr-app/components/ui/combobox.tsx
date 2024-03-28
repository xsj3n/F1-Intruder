"use client"

import * as React from "react"
import { Check, ChevronsUpDown } from "lucide-react"
import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from "@/components/ui/command"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"

const payload_t = [
  {
    value: "wordlist",
    label: "Word List",
  },
  {
    value: "numbers",
    label: "Numbers",
  },
]

export function Combobox({setPayloadOpt}: any) {
  const [open, setOpen] = React.useState(false)
  const [value, setValue] = React.useState<String>("")

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-[200px] justify-between"
        >
          {value ? payload_t.find((payload) => payload.value === value)?.label : "Select payload type..."}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[200px] p-0">
        <Command>
          <CommandInput placeholder="Search payload type..." />
          <CommandEmpty>No payload types found.</CommandEmpty>
          <CommandGroup>
          {payload_t.map((payload) => (
              <CommandItem
                key={payload.value}
                value={payload.value}
                onSelect={(currentValue) => {
                  setValue(currentValue === value ? "" : currentValue)
                  setOpen(false)
                  setPayloadOpt(currentValue)
                }}
              >
                <Check
                  className={cn(
                    "mr-2 h-4 w-4",
                    value === payload.value ? "opacity-100" : "opacity-0"
                  )}
                />
                {payload.label}
              </CommandItem>
            ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  )
}
