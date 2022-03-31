(module
  (import "wasi_unstable" "fd_write" 
    (func $fd_write (param i32 i32 i32 i32) (result i32))
  )
  (memory 1)
  (export "memory" (memory 0))
  (data (i32.const  8) "nope\n")
  (data (i32.const 14) "ok\noverflow")
  (func $test (param $p i32) (result i32 i32)
    (local.set $p (i32.const 14))
    (i32.const 14)
    (i32.const  3)
  )
  (func $main (export "_start")
    (local $base i32)
    (local $len  i32)
    (local.set $base (i32.const 8))
    (local.set  $len (i32.const 5))

    (call $test (local.get $l))
    (local.set $base)
    (local.set  $len)

    (i32.store (i32.const 0) (local.get $base))
    (i32.store (i32.const 4) (local.get  $len))
    (call $fd_write
      (i32.const  1) ;; file_descriptor - 1 for stdout
      (i32.const  0) ;; *iovs
      (i32.const  1) ;; iovs_len
      (i32.const 20) ;; nwritten
    )
    drop
  )
)
