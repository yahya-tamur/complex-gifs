//haskell-style function composition.
//absolutely unnecessary, and slightly embarassing but I thought it was fun
//
// allows multiple arguments to the last function:
//
//hc![f . g . h @ x, y]  = f(g(h(x,y)))
//
// doesn't support for partial application like g. (f 5) $ 6 for g(f(5,6))

macro_rules! hc {
    ( $f:ident $($h:expr)* , $($tail:tt)* ) => {{
        $f ( $($h,)* hc!($($tail)* ) )
    }};
    ( $f:ident @ $($x:expr),+ ) => {{
        $f ( $($x),+ )
    }}
}
