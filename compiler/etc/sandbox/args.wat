(module
  (import "wasi_unstable" "fd_write" 
    (func $fd_write (param i32 i32 i32 i32) (result i32))
  )
  (import "wasi_unstable" "args_get" 
    (func $args_get (param i32 i32) (result i32))
  )
  (memory (export "memory") 1)
  (func (export "_start")
    (i32.store
      (i32.const 3000)
      (call $args_get (i32.const 0) (i32.const 1000))
    )
    (i32.store (i32.const 2000) (i32.const 1000)) ;; iovs[0].iov_base
    (i32.store (i32.const 2004) (i32.const 100)) ;; iovs[0].iov_len
    (call $fd_write
      (i32.const    1) ;; file_descriptor - 1 for stdout
      (i32.const 2000) ;; *iovs
      (i32.const    1) ;; iovs_len
      (i32.const 2100) ;; nwritten
    )
    drop
  )
)
