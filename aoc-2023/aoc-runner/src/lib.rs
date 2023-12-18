use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut aoc_solution = parse_macro_input!(item as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let expanded = quote! {
        #aoc_solution
        fn main() {
            let start = std::time::Instant::now();
            let answer = aoc_solution();
            let elapsed = start.elapsed();
            println!("Answer: {}", answer);

            if elapsed.as_millis() > 0 {
                println!("Time: {}ms", elapsed.as_millis());
              } else {
                println!("Time: {}μs", elapsed.as_micros());
              }
        }
    };

    expanded.into()
}
