extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_asm_nops(_item: TokenStream) -> TokenStream {
    let nop_count: usize = option_env!("NOP_COUNT").unwrap_or("1").parse().unwrap();
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
