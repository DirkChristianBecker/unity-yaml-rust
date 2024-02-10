use unity_yaml_rust::YamlLoader;
use unity_yaml_rust::YamlDocument;
use unity_yaml_rust::Yaml;
use std::collections::HashMap;

fn main() 
{
    let s = r#"
%YAML 1.1
%TAG !u! tag:unity3d.com,2011:
--- !u!1001 &100100000
Prefab:
    m_ObjectHideFlags: 1
    serializedVersion: 2
    m_Modification:
    m_TransformParent: {fileID: 0}
    m_Modifications: []
    m_RemovedComponents: []
    m_ParentPrefab: {fileID: 0}
    m_RootGameObject: {fileID: 1484891345853528}
    m_IsPrefabParent: 1
--- !u!1 &1167328666661238
GameObject:
    m_ObjectHideFlags: 0
    m_PrefabParentObject: {fileID: 0}
    m_PrefabInternal: {fileID: 100100000}
    serializedVersion: 5
    m_Component:
    - component: {fileID: 4612343571051242}
    - component: {fileID: 33327063403659474}
    - component: {fileID: 23114533275561460}
    m_Layer: 0
    m_Name: SM_Bld_Apartment_Glass_01
    m_TagString: Untagged
    m_Icon: {fileID: 0}
    m_NavMeshLayer: 0
    m_StaticEditorFlags: 0
    m_IsActive: 1
"#;
    let docs = YamlLoader::load_from_str(s).unwrap();
    let mut r : HashMap<u64, YamlDocument> = HashMap::new();
    

    for i in 0..docs.len()
    {
        let node = &docs[i];
        match node 
        {
            Yaml::Original(_) => 
            {
                continue;
            }

            Yaml::Hash(_) => 
            {
                if i > 0
                {
                    match docs[i - 1]
                    {
                        Yaml::DocumentMeta(t, id) => 
                        {
                            let d = YamlDocument::new(id, t, node.to_owned());
                            r.insert(id, d);
                        }
                        _ => { }
                    }
                }
            }
            _ =>
            {

            }
        }
    }

    for (_id, document) in r.iter()
    {
        document.print();
    }
}