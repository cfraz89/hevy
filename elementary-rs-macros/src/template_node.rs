use std::sync::Arc;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum TemplateNode {
    Text(String),
    HtmlElement {
        element: HtmlElement,
        child_nodes: Arc<Vec<TemplateNode>>,
    },
    ComponentElement {
        element: ComponentElement,
        child_nodes: Arc<Vec<TemplateNode>>,
        world_identifier: Ident,
    },
    Expression(TokenStream),
}

#[derive(Debug)]
pub struct ComponentElement {
    pub name: String,
    pub properties: Vec<(Ident, TokenStream)>,
}

#[derive(Debug)]
pub struct HtmlElement {
    pub tag: String,
    pub attributes: Vec<(String, String)>,
}

/// Macrotic writing out TemplateNode -> Node
impl ToTokens for TemplateNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        {
            tokens.extend(match self {
                TemplateNode::HtmlElement {
                    element: HtmlElement { tag, attributes },
                    child_nodes,
                } => {
                    let attributes = attributes.iter().map(|(k, v)| {
                        quote! {
                        (#k.to_string(), #v.to_string())
                        }
                    });
                    quote! {
                        elementary_rs_lib::node::NodeRef::from(elementary_rs_lib::node::Node::HtmlElement {
                            element: elementary_rs_lib::node::HtmlElement {
                                tag: #tag.to_string(),
                                attributes: vec![#(#attributes),*]
                            },
                            child_nodes: vec![#(#child_nodes),*]
                        })
                    }
                }
                TemplateNode::Text(text) => quote! {
                    elementary_rs_lib::node::NodeRef::from(elementary_rs_lib::node::Node::Text(#text.to_string()))
                }
                .into(),
                TemplateNode::ComponentElement {
                    element: ComponentElement { name, properties },
                    child_nodes,
                    world_identifier
                } => {
                    let name_ident = format_ident!("{}", name);
                    let properties = properties.iter().map(|(k, v)| {
                        quote! {
                        #k: #v.into()
                        }
                    });
                    quote! {
                        elementary_rs_lib::node::NodeRef::from(elementary_rs_lib::node::Node::Component(
                            elementary_rs_lib::components::BuildWebComponent::build_entity(#name_ident {
                                #(#properties),*
                            }, #world_identifier, vec![#(#child_nodes),*])
                        ))
                    }.into()
                }
                TemplateNode::Expression(tokens) => {
                    let mut hasher = DefaultHasher::new();
                    tokens.to_string().hash(&mut hasher);
                    //Adding the e to make it a valid identifier
                    let hash = hasher.finish();
                    quote! {
                        elementary_rs_lib::node::NodeRef::from(elementary_rs_lib::node::Node::Expression(#hash.to_string(), Box::new({
                            move || (#tokens).to_string()
                    })))
                    }
                }
            })
        }
    }
}
