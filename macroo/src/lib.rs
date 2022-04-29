mod build_game;

#[proc_macro_attribute]
pub fn build_game(attr:proc_macro::TokenStream, input: proc_macro::TokenStream)->proc_macro::TokenStream{
    return build_game::build(attr,input);
}
