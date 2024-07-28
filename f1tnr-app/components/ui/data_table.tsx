
 
import {
  ColumnDef,
  flexRender,
  getPaginationRowModel,
  getCoreRowModel,
  useReactTable,
  RowData,
  RowModel,
  Row,
} from "@tanstack/react-table"
import { useVirtualizer } from '@tanstack/react-virtual';

 
import { FixedSizeList } from "react-window";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import React, { useCallback, useMemo, useRef } from "react"
import { HttpData, remove_toggled_strs_was_ran, set_remove_toggled_strs_was_ran } from "./s_columns"
 
interface DataTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[]
  data: TData[]
  cn: String,
}

interface HttpTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[]
  data: TData[]
  cn: String,
  sethr: (request: any, response: any) => any
}




export function DataTable<TData, TValue>({
  columns,
  data,
  cn,
} : DataTableProps<TData, TValue>) {

  columns = useMemo(() => columns as ColumnDef<TData>[],[])
  const [rowSelection, setRowSelection] = React.useState({})

  const table =  useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    onRowSelectionChange: setRowSelection,
    getPaginationRowModel: getPaginationRowModel(),
    state: {
      rowSelection,
    },
  })
 

  if (remove_toggled_strs_was_ran == true)
  {
    setRowSelection({})
    set_remove_toggled_strs_was_ran(false)
  } 

 
  return (
    <>

    <div className={"rounded-md border " + cn}>
      <Table className="">
        <TableHeader>
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </TableHead>
                )
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableRow
                key={row.id}
                data-state={row.getIsSelected() && "selected"}
              >
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell colSpan={columns.length} className="h-24 text-center">
                No results.
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </div>
    
    </>
  )
}

export function HttpTable<TData, TValue>({
  columns,
  data,
  cn,
  sethr
} : HttpTableProps<TData, TValue>) {

  columns = useMemo(() => columns as ColumnDef<TData>[],[])
  const [rowSelection, setRowSelection] = React.useState({})

  const table =  useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    onRowSelectionChange: setRowSelection,
  })
 

  if (remove_toggled_strs_was_ran == true)
  {
    setRowSelection({})
    set_remove_toggled_strs_was_ran(false)
  } 

  

  const {rows} = table.getRowModel()
  const parent_ref = useRef<HTMLDivElement>(null)
  const virtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: useCallback(() => parent_ref.current, []),
    estimateSize: useCallback(() => 48, []),
  })
 
  return (
    <>

    <div ref={parent_ref} className={"rounded-md border container" + cn}>
      <Table style={{
      height: `${virtualizer.getTotalSize()}px`
    }} >
        <TableHeader className="sticky top-0 z-[1]">
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id} >
              {headerGroup.headers.map((header) => {
                return (
                  <TableHead key={header.id} colSpan={header.colSpan} style={{width: header.getSize()}}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </TableHead>
                )
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
        {
          rows?.length ? (
            virtualizer.getVirtualItems().map((virt_row, index) =>
            {
              const r = rows[virt_row.index] as Row<HttpData>
              return(
                <TableRow key={r.id} data-state={r.getIsSelected() && "selected"} onClick={() => {
                  let hdr = r.original as HttpData
                  sethr(hdr.request, hdr.response)
                }}  style={{
                    height: `${virt_row.size}px`,
                    transform: `translateY(${virt_row.start - index * virt_row.size})`
            }}>
                {r.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
              )
            })
          ) :(
            <TableRow>
              <TableCell colSpan={columns.length} className="h-24 text-center">
                No results.
              </TableCell>
            </TableRow>
          )
        }

        </TableBody>
      </Table>
    </div>
    
    </>
  )
}




/*
          {table.getRowModel().rows?.length ? (

            table.getRowModel().rows.map((row) => (
              <TableRow key={row.id} data-state={row.getIsSelected() && "selected"} onClick={() =>
                {
                  let hdr = row.original as HttpData
                  sethr(hdr.request, hdr.response)
                }
              }>
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell colSpan={columns.length} className="h-24 text-center">
                No results.
              </TableCell>
            </TableRow>
          )}
 */
