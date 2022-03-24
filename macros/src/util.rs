use syn::Type;

pub fn type_to_string(ty: &Type) -> String {
    if let Type::Path(type_path) = ty {
        let path = &type_path.path;
        let ident = path.get_ident().unwrap();
        ident.to_string()
    } else {
        unimplemented!();
    }
}
