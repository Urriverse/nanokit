use crate::*; use ketypes::*;

Ke!
{
    KeEventSubscribe    or |_,_|    Err(())

    KeEventUnsubscribe  or |_,_|    Err(())

    KeEventPublish      or |_,_,_|  Err(())
}
