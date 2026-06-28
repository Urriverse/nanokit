macro x($($y:ident)+){$(pub mod $y; pub use $y::*;)+}
x! { device event exec fs mem module mon paging }
