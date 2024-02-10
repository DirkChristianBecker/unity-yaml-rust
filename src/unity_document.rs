use crate::Yaml;
use crate::dump_node;
use crate::AssetType;

pub struct UnityDocument
{
    document_id : u64,
    asset_type : AssetType,
    node : Yaml,
}

impl UnityDocument
{
    pub fn new(id : u64, t : u64, node : Yaml) -> Self
    {
        let asset_type = AssetType::try_from(t).expect("Could not convert u64 to an asset type.");
        UnityDocument
        {
            document_id : id, 
            asset_type : asset_type,
            node : node,
        }
    }

    pub fn get_id(&self) -> u64 { return self.document_id; }
    pub fn get_asset_type(&self) -> AssetType { return self.asset_type; }

    pub fn print(&self)
    {
        println!("Document: {} Type: {:?}", self.document_id, self.asset_type);
        dump_node(&self.node, 0);
    }

    /// Get a named node from this document.
    /// It has to be a top level element.
    pub fn get_child(&self, name : &str) -> Yaml
    {
        self.node[name].to_owned()
    }
}