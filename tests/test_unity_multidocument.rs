use unity_yaml_rust::AssetFile;
use unity_yaml_rust::AssetType;

#[test]
fn test_unity_yaml() {
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
    m_StaticEditorFlags: 0.1
    m_IsActive: 1
"#;
    let f = AssetFile::from_str(s);
    f.print();
    let doc1 = f.get_document(100100000);

    assert_eq!(doc1.get_asset_type(), AssetType::PrefabInstance);
    assert_eq!(doc1.get_i64("Prefab/m_ObjectHideFlags"), 1);
    assert_eq!(doc1.get_i64("Prefab/serializedVersion"), 2);

    let doc2 = f.get_document(1167328666661238);
    assert_eq!(doc2.get_f64("Prefab/m_StaticEditorFlags"), 0.1);
}
