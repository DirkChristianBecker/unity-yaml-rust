use unity_yaml_rust::YamlLoader;
use std::collections::HashMap;

pub AssetFile
{
    documents : HashMap<u64, YamlDocument> 
}

impl AssetFile
{
    /// Create an empty AssetFile 
    fn new() -> Self {
        AssetFile {
            documents : HashMap::new(),
        }
    }

    /// Add a document to the file.
    fn add_document(&mut self, doc : YamlDocument) {
        self.documents.insert(doc.get_id(), doc);
    }

    /// Create an asset file from 
    pub fn from_string(content : str) -> Self
    {
        let docs = YamlLoader::load_from_str(s).unwrap();
        let mut r = AssetFile::new();

        for i in 0..docs.len() {
            let node = &docs[i];
            match node {
                Yaml::Original(_) => {
                    continue;
                }
                
                // If it is a new hash and the previous element was a document meta
                // element than we habe the beginning of a new document.
                Yaml::Hash(_) => {
                    if i > 0 {
                        match docs[i - 1] {
                            Yaml::DocumentMeta(t, id) => {
                                let d = YamlDocument::new(id, t, node.to_owned());
                                r.add_document(d);
                            }
                            _ => { }
                        }
                    }
                }
                _ => {

                }
            }
        }

        r
    }
}