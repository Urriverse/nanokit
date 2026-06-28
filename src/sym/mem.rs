use crate::*; use ketypes::*;

Ke!
{
    KeMemAlloc              or  |_|     0 as _

    KeMemFree               or  |_,_|   ()

    KeMemAllocStack         or  |_|     KeVaddr::from_raw(0)

    KeMemVirtToPhys         or  |_|     KePaddr::from_raw(0)

    KeMemPhysToVirt         or  |_|     KeVaddr::from_raw(0)
}
