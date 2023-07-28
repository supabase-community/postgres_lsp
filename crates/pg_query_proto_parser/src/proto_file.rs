/// The FieldTypes of a protobuf message
#[derive(Debug)]
pub enum FieldType {
    Node,
    Double,
    Float,
    Int64,
    Uint64,
    Int32,
    Fixed64,
    Fixed32,
    Bool,
    String,
    Group,
    Message,
    Bytes,
    Uint32,
    Enum,
    Sfixed32,
    Sfixed64,
    Sint32,
    Sint64,
}

/// A libg_query token
#[derive(Debug)]
pub struct Token {
    pub name: String,
    pub value: i32,
}

/// A libg_query field
#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub repeated: bool,
}

/// A libg_query node
#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub fields: Vec<Field>,
}

/// The libg_query proto file
pub struct ProtoFile {
    pub tokens: Vec<Token>,
    pub nodes: Vec<Node>,
}

impl ProtoFile {
    pub fn node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }
}
