use convert_case::{Case, Casing};
use protobuf::descriptor::{field_descriptor_proto::Label, FileDescriptorProto};
use protobuf_parse::Parser;
use std::{ffi::OsStr, path::Path};

use crate::proto_file::{Field, FieldType, Node, ProtoFile, Token};

/// The parser for the libg_query proto file
pub struct ProtoParser {
    inner: FileDescriptorProto,
}

impl ProtoParser {
    pub fn new(file_path: &impl AsRef<OsStr>) -> Self {
        let proto_file = Path::new(file_path);
        let proto_dir = proto_file.parent().unwrap();

        let result = Parser::new()
            .pure()
            .include(proto_dir)
            .input(proto_file)
            .parse_and_typecheck()
            .unwrap();

        ProtoParser {
            inner: result.file_descriptors[0].clone(),
        }
    }

    pub fn parse(&self) -> ProtoFile {
        ProtoFile {
            tokens: self.tokens(),
            nodes: self.nodes(),
        }
    }

    fn tokens(&self) -> Vec<Token> {
        self.inner
            .enum_type
            .iter()
            .find(|e| e.name == Some("Token".into()))
            .unwrap()
            .value
            .iter()
            .map(|e| Token {
                // token names in proto are UPPERCASE_SNAKE_CASE
                name: e.name.clone().unwrap().to_case(Case::UpperCamel),
                value: e.number.unwrap(),
            })
            .collect()
    }

    fn get_enum_variant_name(&self, type_name: &str) -> Option<String> {
        let variant = self
            .inner
            .message_type
            .iter()
            .find(|e| e.name == Some("Node".into()))
            .unwrap()
            .field
            .iter()
            .find(|e| e.type_name().split(".").last().unwrap() == type_name);
        match variant {
            Some(v) => Some(v.name.clone().unwrap().to_case(Case::UpperCamel)),
            None => None,
        }
    }

    fn nodes(&self) -> Vec<Node> {
        self.inner
            .message_type
            .iter()
            .find(|e| e.name == Some("Node".into()))
            .unwrap()
            .field
            .iter()
            .map(|e| {
                let name: String = e.name.to_owned().unwrap().to_case(Case::UpperCamel);
                let node = self
                    .inner
                    .message_type
                    .iter()
                    .find(|n| {
                        n.name.clone().unwrap().to_case(Case::UpperCamel)
                            == e.json_name.as_ref().unwrap().to_case(Case::UpperCamel)
                    })
                    .unwrap();

                let mut fields: Vec<Field> = Vec::new();
                // from node fields
                fields.append(&mut
                        node
                        .field
                        .iter()
                        .filter_map(|e| {
                            // skip one of fields, they are handled separately
                            if e.has_oneof_index() {
                                return None;
                            }
                            // use label and type to get the field type
                            let type_name: FieldType = match e.type_name() {
                                "" => match e.type_() {
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_DOUBLE => FieldType::Double,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_FLOAT => FieldType::Float,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT64 => FieldType::Int64,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT64 => FieldType::Uint64,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT32 => FieldType::Int32,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED64 => FieldType::Fixed64,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED32 => FieldType::Fixed32,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL => FieldType::Bool,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING => FieldType::String,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_GROUP => FieldType::Group,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE => FieldType::Message,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_BYTES => FieldType::Bytes,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT32 => FieldType::Uint32,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM => FieldType::Enum,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED32 => FieldType::Sfixed32,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED64 => FieldType::Sfixed64,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT32 => FieldType::Sint32,
                                    protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT64 => FieldType::Sint64,
                                },
                                _ => {
                                    if !e.type_name().starts_with(".pg_query") {
                                        panic!("Unknown type: {}", e.type_name());

                                    }
                                    if e.type_() == protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM {
                                        FieldType::Enum
                                    } else {
                                        FieldType::Node
                                    }
                                },
                            };
                            let mut node_name = None;
                            let mut enum_variant_name = None;
                            if e.type_name().starts_with(".pg_query") {
                                let n = e.type_name().split(".").last().unwrap().to_string();
                                node_name = Some(n.clone());
                                if n != "Node" {
                                    enum_variant_name = self.get_enum_variant_name(e.type_name().split(".").last().unwrap().to_string().as_str());
                                }
                            }
                            // TODO: node name must be derived from the property name in the node
                            // enum
                            Some(Field {
                                name: e.name.clone().unwrap(),
                                node_name,
                                enum_variant_name,
                                field_type: type_name,
                                repeated: e.label() == Label::LABEL_REPEATED,
                                is_one_of: false,
                            })
                        })
                        .collect()
                    );

                    // one of declarations
                    fields.append(&mut
                        node
                        .oneof_decl
                        .iter()
                        .map(|e| {
                            Field {
                       name: e.name.clone().unwrap(),
                       node_name: Some("Node".to_string()),
                       enum_variant_name: None,
                       field_type: FieldType::Node,
                       repeated: false,
                       is_one_of: true,
                   }
                        })
                        .collect()
                              );
                Node {
                    // token names in proto are UPPERCASE_SNAKE_CASE
                    name: name.clone(),
                    fields,
                }
            })
            .collect()
    }
}
