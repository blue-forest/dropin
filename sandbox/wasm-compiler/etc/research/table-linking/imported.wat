(module
  ;; (File Descriptor, *iovs, iovs_len, nwritten) -> Returns number of bytes written
  (import "wasi_unstable" "fd_write" 
    (func $fd_write (param i32 i32 i32 i32) (result i32))
  )
  (table 1 funcref)
  (export "table" (table 0))
  (elem (i32.const 0) $main)

  (type $main.type (func ))

  (memory 1)
  (export "memory" (memory 0))

  (data (i32.const 8) "printed: a\n")

  (func $main (param $p i32)
    (i32.store (i32.const 0) (i32.const 8))  ;; iov.iov_base
    (i32.store (i32.const 4) (i32.const 11))  ;; iov.iov_len
    (i32.store8 (i32.const 17) (i32.add (i32.const 0x61) (local.get $p))) ;; change 'a'

    (call $fd_write
      (i32.const 1) ;; file_descriptor - 1 for stdout
      (i32.const 0) ;; *iovs
      (i32.const 1) ;; iovs_len
      (i32.const 20) ;; nwritten
    )
    drop
  )
)
