(module
  ;; (File Descriptor, *iovs, iovs_len, nwritten) -> Returns number of bytes written
  (import "wasi_unstable" "fd_write" 
    (func $fd_write (param i32 i32 i32 i32) (result i32))
  )
  (memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $main))

  (data (i32.const 16) "hello")
  (data (i32.const 21) "world\n")

  (func $main (local i32)
    (i32.add (i32.const 20) (i32.const 1)) ;; dst
    (i32.const 16)                         ;; src
    (i32.const  5)                         ;; size
    (memory.copy)
    (i32.store (i32.const  0) (i32.const 21)) ;; iovs[0].iov_base
    (i32.store (i32.const  4) (i32.const  6)) ;; iovs[0].iov_len
    (call $fd_write
      (i32.const  1) ;; file_descriptor - 1 for stdout
      (i32.const  0) ;; *iovs
      (i32.const  2) ;; iovs_len
      (i32.const 28) ;; nwritten
    )
    drop
  )
)
