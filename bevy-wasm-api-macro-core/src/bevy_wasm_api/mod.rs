mod utils;
pub mod analyze;
pub mod codegen;

#[allow(unused_imports)]
mod tests {
    use quote::quote;

    use crate::bevy_wasm_api::{analyze::analyze, codegen::codegen};

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
