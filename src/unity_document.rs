use crate::dump_node;
use crate::AssetType;
use crate::Yaml;

pub struct UnityDocument {
    document_id: u64,
    asset_type: AssetType,
    node: Yaml,
}

impl UnityDocument {
    pub fn new(id: u64, t: u64, node: Yaml) -> Self {
        let asset_type = AssetType::try_from(t).expect("Could not convert u64 to an asset type.");
        UnityDocument {
            document_id: id,
            asset_type: asset_type,
            node: node,
        }
    }

    pub fn get_id(&self) -> u64 {
        return self.document_id;
    }
    pub fn get_asset_type(&self) -> AssetType {
        return self.asset_type;
    }

    pub fn print(&self, dump: bool) {
        println!("Document: {} Type: {:?}", self.document_id, self.asset_type);

        if dump {
            dump_node(&self.node, 0);
        }
    }

    /// Get a named node from this document.
    /// It has to be a top level element.
    pub fn get_child(&self, name: &str) -> &Yaml {
        &self.node[name]
    }

    /// Get a node inside this document. The path is a '/' separated path to the
    /// node in question, where each node on the path must be a Yaml::Hash.
    pub fn get_node(&self, path: &str) -> Option<&Yaml> {
        // The first element in the document should be a hash-map.
        let elements = path.split('/');

        let mut result: Option<&Yaml> = Some(&self.node);
        for n in elements {
            if let Some(h) = result?.as_hash() {
                result = h.get(&Yaml::from_str(n));
                continue;
            }

            return None;
        }

        return result;
    }

    /// Read a value from this document. The path is a sequence of elements
    /// separated by '/' the gives the order of elements to access.
    pub fn get_i64(&self, path: &str) -> i64 {
        if let Some(n) = self.get_node(path) {
            if let Some(i) = n.as_i64() {
                return i;
            }

            eprintln!(
                "Could not convert '{}' to i64 from document {}.",
                path, self.document_id
            );

            return -1;
        }

        return -1;
    }

    /// Read a value from this document. The path is a sequence of elements
    /// separated by '/' the gives the order of elements to access.
    pub fn get_f64(&self, path: &str) -> f64 {
        if let Some(n) = self.get_node(path) {
            if let Some(i) = n.as_f64() {
                return i;
            }

            eprintln!(
                "Could not convert '{}' to f64 from document {}.",
                path, self.document_id
            );

            return -1.0;
        }

        return -1.0;
    }

    /// Read a value from this document. The path is a sequence of elements
    /// separated by '/' the gives the order of elements to access.
    pub fn get_string(&self, path: &str) -> &str {
        if let Some(n) = self.get_node(path) {
            if let Some(i) = n.as_str() {
                return i;
            }

            eprintln!(
                "Could not convert '{}' to f64 from document {}.",
                path, self.document_id
            );

            return "";
        }

        return "";
    }

    /// Get an array from this document. The path is a sequence of elements
    /// separated by '/' the gives the order of elements to access.
    pub fn get_array(&self, path: &str) -> &Vec<Yaml> {
        if let Some(n) = self.get_node(path) {
            if let Some(i) = n.as_vec() {
                return i;
            }

            eprintln!(
                "Could not convert '{}' to f64 from document {}.",
                path, self.document_id
            );
        }

        panic!("Could not find array in this document {}", path);
    }
}
