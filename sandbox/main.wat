(module
  (import "env" "memory" (memory 1)) 
  (import "env" "table" (table 1 funcref)) 
  (import "wasi_unstable" "fd_write" 
    (func $fd_write (param i32 i32 i32 i32) (result i32))
  )
  (type $imported_type (func (result i32 i32)))
  (func $main (export "_start")
    (call_indirect 0 (type $imported_type) (i32.const 0))
  )
)
