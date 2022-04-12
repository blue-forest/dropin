(module
  (import "wasi_unstable" "fd_read" 
    (func $fd_read 
      (param i32 i32 i32 i32) 
      (result i32)))
  (import "wasi_unstable" "fd_write" 
    (func $fd_write 
      (param i32 i32 i32 i32) 
      (result i32)))

  (memory 1)
  (export "memory" (memory 0))

  (func $main (export "_start")

    ;; buffer of 100 chars to read into
    (i32.store (i32.const 4) (i32.const 12))
    (i32.store (i32.const 8) (i32.const 100))

    (call $fd_read
      (i32.const 0) ;; 0 for stdin
      (i32.const 4) ;; *iovs
      (i32.const 1) ;; iovs_len
      (i32.const 8) ;; nread
    )
    drop

    (call $fd_write
      (i32.const 1) ;; 1 for stdout
      (i32.const 4) ;; *iovs 
      (i32.const 1) ;; iovs_len
      (i32.const 0) ;; nwritten
    )
    drop
  )
)
