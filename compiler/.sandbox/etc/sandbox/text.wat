(module
  ;; (import "env" "memory" (memory 1))
  ;; (import "env" "table" (table 1 funcref))
  (import "wasi_unstable" "fd_read" 
    (func $fd_read (param i32 i32 i32 i32) (result i32))
  )
  (import "wasi_unstable" "fd_write" 
    (func $fd_write (param i32 i32 i32 i32) (result i32))
  )
  (memory 1)
  (export "memory" (memory 0))
  ;; (elem funcref (item (ref.func $encode)))

  ;; tb: test base      |
  ;; tl: test len       |
  ;; ob: other base     | tb          tl          ob          ol
  ;; ol: other len      | 69          10          79          11
  (data (i32.const 420) "\45\00\00\00\0a\00\00\00\4f\00\00\00\0b\00\00\00")
  (data  $test (i32.const 69)  "old value\n")
  (data $other (i32.const 79) "other data\n")

  (func $encode
    (param $self     i32)
    (param $head     i32)
    (param $argument i32)

    (local.set $addr
      (select
        (local.get $arg.addr) ;; false
        (local.get $head)     ;; true
        (i32.lt_s (local.get $arg.len) (i32.const 3))
      )
    )
    (i32.store16
      (local.get $addr)
      (i32.const 0xc0ff)
    )
    (i32.store8
      (i32.add (local.get $addr) (i32.const 2))
      (i32.const 0xee)
    )
  )

  (func $main (export "_start")
    (local $addr i32)

    (local.set $addr (i32.const 420))

    (call $fd_read
      (i32.const     0) ;; 0 for stdin
      (local.get $addr) ;; *iovs
      (i32.const     1) ;; iovs_len
      (i32.const     0) ;; nread
    )
    drop
    
    (call $encode
      (i32.const     0)     ;; self
      (i32.const   440)   ;; head
      (local.get $addr) ;; argument.addr
    )
  )
)
