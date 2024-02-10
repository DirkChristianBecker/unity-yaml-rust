# unity-yaml-rust

`unity-yaml-rust` is a pure Rust YAML parser for Unity.

Add Unity YAML spec and mutable access Base on [yaml-rust](https://github.com/chyh1990/yaml-rust) 


## Quick Start

Since this version of unity-yaml-rust is not (yet?) available via cargo, add the following to the Cargo.toml of your project:

```toml
[dependencies]
unity-yaml-rust = { git = "https://github.com/DirkChristianBecker/unity-yaml-rust.git" }
```

Use `yaml::YamlLoader` to load the YAML documents and access it
as Vec/HashMap:

```rust
use unity_yaml_rust::{yaml::YamlLoader, emmitter::YamlEmitter};

fn main() {
    let s =
r#"
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
    let mut docs = YamlLoader::load_from_str(s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    for doc in docs.iter_mut() {
        // Debug support
        println!("{:?}", doc);

        if !matches!(doc, Yaml::Original(_)) {
            //IndexMut
            let sprite_atlas = &mut doc["SpriteAtlas"];
            
            assert_eq!(sprite_atlas["m_ObjectHideFlags"].as_i64(), Some(0i64));
            assert!(sprite_atlas["m_ObjectHideFlags"].replace_i64(3));
            assert_eq!(sprite_atlas["m_ObjectHideFlags"].as_i64(), Some(3i64));
            
            sprite_atlas["m_Name"].replace_string("launch".to_string());
            assert_eq!(sprite_atlas["m_Name"].as_str(), Some("launch"));

            sprite_atlas.insert("custom", Yaml::Boolean(true));
            assert_eq!(sprite_atlas["custom"].as_bool(), Some(true));

            sprite_atlas.remove("m_PackedSprites");
            assert!(sprite_atlas["m_PackedSprites"].is_badvalue());

            sprite_atlas["m_EditorData"]["packables"].remove_at(0);
            sprite_atlas["m_EditorData"]["packables"].push(Yaml::String("ppp".to_string()));
            sprite_atlas["m_MasterAtlas"].insert("plus", Yaml::Boolean(false));
            sprite_atlas["m_MasterAtlas"].remove("fileID");
        }

        // Dump the YAML object
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(doc).unwrap(); // dump the YAML object to a String
        }
        println!("{}", out_str);
    }
}
```
Beside that, there is the AssetFile API. Which allows to access elements by a path:
```rust
use unity_yaml_rust::AssetFile;
use unity_yaml_rust::AssetType;

fn main() {
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

    // Access a file via file id.
    // The file id can be found after the &-Symbol in the document start tag: 
    // --- !u!1001 &100100000
    let doc1 = f.get_document(100100000);

    assert_eq!(doc1.get_asset_type(), AssetType::PrefabInstance);
    assert_eq!(doc1.get_i64("Prefab/m_ObjectHideFlags"), 1);
    assert_eq!(doc1.get_i64("Prefab/serializedVersion"), 2);

    let doc2 = f.get_document(1167328666661238);
    assert_eq!(doc2.get_f64("GameObject/m_StaticEditorFlags"), 0.1);
}
```


## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).
