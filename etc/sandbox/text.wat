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

  (func $encode
    (param $self i32)
    (param $head i32)
    (param $arg.addr i32)
    (param $arg.len i32)
    (local $addr i32)

    (i32.store (i32.const 4) (local.get $arg.addr))
    (i32.store (i32.const 8) (local.get $arg.len))

    (call $fd_write
      (i32.const 1) ;; 1 for stdout
      (i32.const 4) ;; *iovs
      (i32.const 1) ;; iovs_len
      (i32.const 0) ;; nwrite
    )
    drop

    ;; (local.set $addr
    ;;   (select
    ;;     (local.get $arg.addr) ;; false
    ;;     (local.get $head) ;; true
    ;;     (i32.lt_s (local.get $arg.len) (i32.const 3))
    ;;   )
    ;; )
    ;; (i32.store16
    ;;   (local.get $addr)
    ;;   (i32.const 0xc0ff)
    ;; )
    ;; (i32.store8
    ;;   (i32.add (local.get $addr) (i32.const 2))
    ;;   (i32.const 0xee)
    ;; )
  )

  (func $main (export "_start")
    (local $len  i32)
    (local $cap  i32)
    (local $addr i32)

    ;; buffer 100 bytes @ 0x00
    (local.set $addr (i32.const 12))
    (i32.store (i32.const 4) (local.get $addr))
    (local.set $cap (i32.const 100))
    (i32.store (i32.const 8) (local.get $cap))

    (call $fd_read
      (i32.const 0) ;; 0 for stdin
      (i32.const 4) ;; *iovs
      (i32.const 1) ;; iovs_len
      (i32.const 0) ;; nread
    )
    (local.set $len)
    
    (call $encode
      (i32.const 0)     ;; self
      (i32.const 112)   ;; head
      (local.get $addr) ;; argument.addr
      (local.get $len)  ;; argument.len
    )
  )
)
