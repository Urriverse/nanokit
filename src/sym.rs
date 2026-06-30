macro x($($y:ident)+){$(pub mod $y; #[allow(unused)] pub use $y::*;)+}
x! { device event exec fs mem module mon paging }
