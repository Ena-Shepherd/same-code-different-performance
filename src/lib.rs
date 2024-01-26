extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_asm_nops(_item: TokenStream) -> TokenStream {
    let nop_count: usize = read_nop_count();
    let nops = std::iter::repeat(r#""nop""#)
        .take(nop_count)
        .collect::<Vec<_>>()
        .join(", ");
    let code = format!(
        "#[inline(always)] unsafe fn __asm_nops() {{ std::arch::asm! {{ {} }} }}",
        nops
    );
    code.parse().unwrap()
}

#[proc_macro]
pub fn nop_count(_item: TokenStream) -> TokenStream {
    let code = format!("{}", read_nop_count());
    code.parse().unwrap()
}

fn read_nop_count() -> usize {
    option_env!("NOP_COUNT").unwrap_or("1").parse().unwrap()
}
