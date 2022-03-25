(module
  (import "wasi_unstable" "fd_write" 
    (func $fd_write (param i32 i32 i32 i32) (result i32))
  )
  (memory 1)
  (export "memory" (memory 0))

  (data (i32.const 8) "hello\n")

  (func $main (export "_start") (local $x i64)
    (local.set $x (i64.const 8))
    (i32.store (i32.const  0)                 ;; iovs[0].iov_base
      (i32.wrap_i64 (local.get $x))
    )
    (i32.store (i32.const  4) (i32.const  6)) ;; iovs[0].iov_len
    (call $fd_write
      (i32.const  1) ;; file_descriptor - 1 for stdout
      (i32.const  0) ;; *iovs
      (i32.const  1) ;; iovs_len
      (i32.const 28) ;; nwritten
    )
    drop
  )
)
