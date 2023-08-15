//haskell-style function composition.
//absolutely unnecessary, and slightly embarassing but I thought it was fun
//
//hc![f , g  A , h @ B, C]  = f(g(A, h(B,C)))

macro_rules! hc {
    ( $f:ident $($h:expr)* , $($tail:tt)* ) => {{
        $f ( $($h,)* hc!($($tail)* ) )
    }};
    ( $f:ident @ $($x:expr),+ ) => {{
        $f ( $($x),+ )
    }}
}
