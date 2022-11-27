(module
  (import "env" "print" (func $print (param i32 i32)))

  (data (i32.const 8) "hello\n")

  (memory 1)
  (export "memory" (memory 0))

  (func $main (export "_start")
    (call $print (i32.const 8) (i32.const 6))
  )
)
