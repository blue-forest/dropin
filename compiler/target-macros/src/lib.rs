use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Stage)]
pub fn stage(_item: TokenStream) -> TokenStream {
  quote!().into()
  /*
  let mut r#struct: ItemStruct = syn::parse(item).unwrap();
  let generics = &mut r#struct.generics;
  let fields = &mut r#struct.fields;
  let Fields::Unnamed(fields) = fields else {
    panic!("combine takes a tuple-like struct");
  };
  let mut states = Vec::<Type>::with_capacity(fields.unnamed.len());
  for (field_i, field) in fields.unnamed.iter_mut().enumerate() {
    let mut attr_i = 0;
    while attr_i < field.attrs.len() {
      let meta = &field.attrs[attr_i].meta;
      attr_i += 1;
      let Meta::List(list) = meta else {
        continue;
      };
      if list.path.is_ident("state") {
        if states.len() > field_i {
          panic!("multiple states provided");
        }
        attr_i -= 1;
        let attr = field.attrs.remove(attr_i);
        let Meta::List(list) = attr.meta else {
          unreachable!();
        };
        states.push(syn::parse(list.tokens.into()).unwrap());
      }
    }
    if states.len() != field_i + 1 {
      panic!("no state provided");
    }
    let generic = Ident::new(&format!("S{field_i}"), Span::call_site());
    generics.params.push(parse_quote!(#generic: Stage));
    let Type::Path(ty) = &mut field.ty else {
      panic!("unexpected type");
    };
    let PathArguments::AngleBracketed(args) =
      &mut ty.path.segments.last_mut().unwrap().arguments
    else {
      panic!("unexpected args");
    };
    args.args.push(parse_quote!(#generic));
  }

  let name = r#struct.ident.clone();
  let generics_declaration = r#struct.generics.clone();
  let generics = r#struct
    .generics
    .params
    .iter()
    .map(|generic| match generic {
      GenericParam::Lifetime(generic) => generic.lifetime.to_token_stream(),
      GenericParam::Type(generic) => generic.ident.to_token_stream(),
      GenericParam::Const(_) => todo!("const generics"),
    })
    .collect::<Vec<_>>();
  let mut stream = r#struct.to_token_stream();
  for (i, state) in states.into_iter().enumerate() {
    let i = Index::from(i);
    stream.extend(quote!(
      impl #generics_declaration Stated<#state> for #name <#(#generics),*> {
        fn state(&self) -> &#state {
          &self.#i.state()
        }
      }
    ));
  }
  stream.extend(quote!(
    impl #generics_declaration Stage for #name <#(#generics),*> {
      fn ir(&self) -> &Model {
        self.0.ir()
      }
    }
  ));
  stream.into()
  */
}
