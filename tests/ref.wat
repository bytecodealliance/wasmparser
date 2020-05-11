(module
  (type (;0;) (func))
  (type (;1;) (func (param funcref)))
  (func (;0;) (type 0)
    (local externref funcref)
    ref.null extern
    local.set 0
    ref.null func
    local.set 1)
  (func (;1;) (type 1) (param funcref)
    global.get 0
    ref.is_null extern
    drop
    local.get 0
    ref.is_null func
    drop)
  (global (;0;) externref (ref.null extern)))
