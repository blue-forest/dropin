(module
  (import "test" "table" (table 1 funcref))
  (type $test.start (func (param i32)))

  (func $main (export "_start")
    (call_indirect 0 (type $test.start) (i32.const 1) (i32.const 0))
  )
)
