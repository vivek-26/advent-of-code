use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn, LitInt};

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let day = parse_macro_input!(args as LitInt);
    let day = day.base10_parse::<u8>().unwrap();

    let input_path = get_input_path(day);
    let input = std::fs::read_to_string(input_path).unwrap();
    let input = input.as_str();

    let mut aoc_solution = parse_macro_input!(item as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let expanded = quote! {
        #aoc_solution
        fn main() {
            let start = std::time::Instant::now();
            let answer = aoc_solution(#input);
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

fn get_input_path(day: u8) -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("inputs");
    path.join(format!("{:02}.in", day))
}
