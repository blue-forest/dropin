;; MEMORY ADDRESSES
;;   0: tmp1 (u32)
;;   4: tmp2 (u32)
;;   8: tmp3 (u32)
;;  69: test content (value to print)
;;  79: other content (value to ignore)
;; 420: values header
;; 436: head

(module
  (import "wasi_unstable" "fd_read" 
    (func $fd_read (param i32 i32 i32 i32) (result i32))
  )
  (import "wasi_unstable" "fd_write" 
    (func $fd_write (param i32 i32 i32 i32) (result i32))
  )
  (memory 1)
  (export "memory" (memory 0))

  ;; tb: test base      |
  ;; tl: test len       |
  ;; ob: other base     | tb          tl          ob          ol
  ;; ol: other len      | 69          10          79          11
  (data (i32.const 420) "\45\00\00\00\0a\00\00\00\4f\00\00\00\0b\00\00\00")
  (data  $test (i32.const 69)  "old value\n")
  (data $other (i32.const 79) "other data\n")

  (func $main (export "_start")
    (local       $addr i32)
    (local      $nread i32)
    (local $total_read i32)
    (local       $head i32)
    (local   $buf_size i32)
    (local        $eof i32)

    (local.set     $addr (i32.const 420))
    (local.set $buf_size (i32.const   3))
    (local.set      $eof (i32.const  10)) ;; interpret \n as EOF
    (local.set     $head (i32.add
      (local.get $addr)
      (i32.const 16)
    )) ;; head after string

    (i32.store (i32.const 8) (local.get $buf_size)) ;; iovs.len
    (loop $read
      (i32.store (i32.const 4)
        (i32.add
          (local.get       $head)
          (local.get $total_read)
        )
      ) ;; iovs.base
      (call $fd_read
        (i32.const 0) ;; 0 for stdin
        (i32.const 4) ;; *iovs
        (i32.const 1) ;; iovs_len
        (i32.const 0) ;; nread
      )
      drop

      (local.set      $nread (i32.load (i32.const 0)))
      (local.set $total_read (i32.add
        (local.get $total_read)
        (local.get      $nread)
      ))

      (i32.ne                                       ;; last byte == eof ?
        (i32.load (i32.sub
          (i32.add (local.get $head) (local.get $total_read))
          (i32.const 1)
        ))
        (local.get $eof)
      )
      (br_if $read)
    )
    (i32.store (local.get $addr) (local.get $head)) ;; test.base
    (i32.store                                      ;; test.len
      (i32.add (local.get $addr) (i32.const 4))
      (local.get $total_read)
    )
    (local.set $head (i32.add                       ;; update head
      (local.get       $head)
      (local.get $total_read)
    ))

    (call $fd_write
      (i32.const     1) ;; file_descriptor - 1 for stdout
      (local.get $addr) ;; *iovs
      (i32.const     1) ;; iovs_len
      (i32.const     0) ;; nwritten
    )
    drop
  )
)
