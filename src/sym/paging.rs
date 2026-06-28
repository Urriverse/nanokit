use crate::*; use ketypes::*;

Ke!
{
    KePagingMap             or  |_,_,_,_|   Err("function not provided")

    KePagingRemap           or  |_,_,_|     Err("function not provided")

    KePagingUnmap           or  |_,_|       Err("function not provided")

    KePagingMerge           or  |_,_|       ()

    KePagingQuery           or  |_|         None
}
