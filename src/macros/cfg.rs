macro_rules! cfg_dangerous {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "dangerous")]
            #[cfg_attr(docsrs, doc(cfg(feature = "dangerous")))]
            $item
        )*
    }
}
