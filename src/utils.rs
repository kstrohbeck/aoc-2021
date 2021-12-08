macro_rules! sep_arrays {
    ($( $name:ident, $num:literal ),*) => {
        $(
            pub fn $name<I, O, O2, E, F, G>(
                mut sep: G,
                mut parser: F,
            ) -> impl FnMut(I) -> ::nom::IResult<I, [O; $num], E>
            where
                F: nom::Parser<I, O, E>,
                G: nom::Parser<I, O2, E>,
                E: nom::error::ParseError<I>,
            {
                use std::mem::MaybeUninit;

                move |mut input| {
                    // Safe because we aren't actually able to access any uninitialized memory.
                    let mut array: [MaybeUninit<O>; $num] = unsafe { MaybeUninit::uninit().assume_init() };

                    for (i, cell) in array.iter_mut().enumerate() {
                        if i != 0 {
                            let (inp, _) = sep.parse(input)?;
                            input = inp;
                        }
                        let (inp, u) = parser.parse(input)?;
                        input = inp;
                        cell.write(u);
                    }

                    // Safe because:
                    // - [MaybeUninit<T>; n] and [T; n] are guaranteed to be the same size
                    // - each MaybeUninit<T> in the array has been initialized
                    let array = unsafe { array.as_ptr().cast::<[O; $num]>().read() };

                    Ok((input, array))
                }
            }
        )*
    };
}

sep_arrays! {
    sep_array_4, 4,
    sep_array_5, 5,
    sep_array_10, 10
}
