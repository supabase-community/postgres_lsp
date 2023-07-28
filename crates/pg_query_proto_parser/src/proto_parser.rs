use convert_case::{Case, Casing};
use protobuf::descriptor::{field_descriptor_proto::Label, FileDescriptorProto};
use protobuf_parse::Parser;
use std::path::Path;

use crate::proto_file::{Field, FieldType, Node, ProtoFile, Token};

/// The parser for the libg_query proto file
pub struct ProtoParser {
    inner: FileDescriptorProto,
}

impl ProtoParser {
    pub fn new(file_path: &str) -> Self {
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

    fn nodes(&self) -> Vec<Node> {
        self.inner
            .message_type
            .iter()
            .find(|e| e.name == Some("Node".into()))
            .unwrap()
            .field
            .iter()
            .map(|e| {
                let name: String = e.json_name.to_owned().unwrap();
                Node {
                    // token names in proto are UPPERCASE_SNAKE_CASE
                    name: name.clone(),
                    fields: self
                        .inner
                        .message_type
                        .iter()
                        .find(|n| n.name.clone().unwrap() == name)
                        .unwrap()
                        .field
                        .iter()
                        .map(|e| {
                            // use label and type to get the field type
                            let type_name: FieldType = match e.type_name() {
                                ".pg_query.Node" => FieldType::Node,
                                _ => match e.type_() {
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
                            };
                            Field {
                                name: e.name.clone().unwrap(),
                                field_type: type_name,
                                repeated: e.label() == Label::LABEL_REPEATED,
                            }
                        })
                        .collect(),
                }
            })
            .collect()
    }
}
