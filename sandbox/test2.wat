(module
  (import "test" "table" (table 0 funcref))
  (type $test.start (func))

  (func $main (export "_start")
    (call_indirect 0 (type $test.start) (i32.const 0))
  )
)
