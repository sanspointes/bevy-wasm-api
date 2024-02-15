mod utils;
pub mod analyze;
pub mod codegen;

mod tests {
    use proc_macro2::Literal;
    use quote::quote;

    use super::{analyze::analyze, codegen::codegen};

    #[test]
    pub fn integration_example() {
        let ts = quote! {
            impl MyApi {
                pub fn my_method(&self, arg1: &mut World) {
                    println!("Hello")
                }
                pub fn my_method_2(&self, arg1: &mut World) -> String {
                    "Hello".to_string()
                }
            }
        };
        let model = analyze(ts).unwrap();
        let ts2 = codegen(model);

        println!("\n{}", ts2.to_string());
        assert!(false)
    }
}
