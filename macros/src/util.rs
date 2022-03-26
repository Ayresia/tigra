use syn::{GenericArgument, PathArguments, Type};

pub fn type_to_string(ty: &Type) -> String {
    if let Type::Path(type_path) = ty {
        let path = &type_path.path;
        let ident = path.get_ident().expect("Unable to get ident");
        ident.to_string()
    } else {
        unimplemented!();
    }
}

pub fn option_to_type(ty: &Type) -> &Type {
    if let Type::Path(type_path) = ty {
        let path = &type_path.path;
        let segment = &path.segments[0];
        let ident = &segment.ident;
        let ident_str = ident.to_string();

        if ident_str == "Option" {
            let arguments = &segment.arguments;

            if let PathArguments::AngleBracketed(angle_bracketed) = arguments {
                if let GenericArgument::Type(ty) = &angle_bracketed.args[0] {
                    return ty;
                }
            }
        }

        panic!("Unable to parse option type");
    } else {
        unimplemented!();
    }
}
