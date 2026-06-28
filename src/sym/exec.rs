use crate::*; use ketypes::*;

Ke!
{
    KeExecPanic             or   |_|            loop{core::hint::spin_loop()}

    KeExecExit              or   |_|            loop{core::hint::spin_loop()}

    KeExecYield             or   ||             ()

    KeExecSleep             or   |_|            ()

    KeExecSpawn             or   |_,_,_,_,_|    0

    KeExecArgumentedSpawn   or   |_,_,_,_,_,_|  0

    KeExecWaitChild         or   |_|            -1

    KeExecSetDeadline       or   |_,_|          ()
}
