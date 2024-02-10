use unity_yaml_rust::AssetFile;
use unity_yaml_rust::AssetType;
use unity_yaml_rust::dump_node;

#[test]
fn test_unity_yaml() {
    let unity_yaml = r#"
%YAML 1.1
%TAG !u! tag:unity3d.com,2011:
--- !u!687078895 &4343727234628468602
SpriteAtlas:
  m_ObjectHideFlags: 0
  m_CorrespondingSourceObject: {fileID: 0}
  m_PrefabInstance: {fileID: 0}
  m_PrefabAsset: {fileID: 0}
  m_Name: atlas_launch
  m_EditorData:
    serializedVersion: 2
    textureSettings:
      serializedVersion: 2
      anisoLevel: 1
      compressionQuality: 50
      maxTextureSize: 2048
      textureCompression: 0
      filterMode: 1
      generateMipMaps: 0
      readable: 0
      crunchedCompression: 0
      sRGB: 1
    platformSettings: []
    packingSettings:
      serializedVersion: 2
      padding: 4
      blockOffset: 1
      allowAlphaSplitting: 0
      enableRotation: 0
      enableTightPacking: 0
    variantMultiplier: 1
    packables:
    - {fileID: 102900000, guid: c5a32d8209c314631bad292a32582dfc, type: 3}
    bindAsDefault: 1
  m_MasterAtlas: {fileID: 0}
  m_PackedSprites:
  - {fileID: 21300000, guid: 3083aff0bd42a4268a9cfe6e564ab247, type: 3}
  - {fileID: 21300000, guid: 054656e6c52c2425eb7c5ec764d03587, type: 3}
  - {fileID: 21300000, guid: 55aba929877c26747acf9ad10ee7989c, type: 3}
  m_PackedSpriteNamesToIndex:
  - login_ic_logo
  - launch_icon_service
  - login_ic_logo_bak1
  m_Tag: atlas_launch
  m_IsVariant: 0
"#;

    let f = AssetFile::from_str(unity_yaml);
    f.print();
    let doc1 = f.get_document(4343727234628468602);

    assert_eq!(doc1.get_asset_type(), AssetType::SpriteAtlas);
    assert_eq!(doc1.get_i64("SpriteAtlas/m_ObjectHideFlags"), 0);
    assert_eq!(doc1.get_string("SpriteAtlas/m_Name"), "atlas_launch");

    assert_eq!(doc1.get_i64("SpriteAtlas/m_EditorData/serializedVersion"), 2);

    let array = doc1.get_node("SpriteAtlas/m_PackedSprites").expect("Should always exist").as_vec().expect("Should be an array.");
    dump_node(&array[0], 0);
}
