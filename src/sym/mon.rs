use crate::*; use ketypes::*;

Ke!
{
    KeMonLog                or  |_,_,_,_,_| ()

    KeMonAddSink            or  |_|         { error!("KeMonAddSink not provided"); }
}
