//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

use core::config::read_config;
use core::config::read_cfg;
use core::config::ApplicationCfg;


#[test]
fn test_load_config() {
    let config = read_cfg("src/test/example.toml");
    println!("{:#?}", config);
    let config: ApplicationCfg = config.expect("load_config returned Err");
    assert!(config.service.mvt.viewer);
    assert_eq!(config.datasource.dstype, "postgis");
    assert_eq!(config.grid.predefined, Some("web_mercator".to_string()));
    assert_eq!(config.tilesets.len(), 1);
    assert_eq!(config.tilesets[0].name, "osm");
    assert_eq!(config.tilesets[0].layers.len(), 3);
    assert_eq!(config.tilesets[0].layers[0].name, "points");
    assert!(config.cache.is_none());
    assert_eq!(config.webserver.port, Some(8080));
}

#[test]
fn test_parse_config() {
    let config = read_config("src/test/example.toml").unwrap();
    println!("{:#?}", config.as_table().unwrap());
    let expected_begin = r#"{
    "datasource": Table(
        {
            "type": String(
                "postgis"
            ),
            "url": String(
                "postgresql://postgres@127.0.0.1/natural_earth_vectors"
            )
        }
    ),
    "grid": Table(
        {
            "predefined": String(
                "web_mercator"
            )
        }
    ),
    "service": Table(
        {
            "mvt": Table(
                {
                    "viewer": Boolean(
                        true
                    )
                }
            )
        }
    ),
    "tileset": Array(
        [
            Table(
                {
                    "layer": Array(
                        [
                            Table(
                                {
                                    "fid_field": String(
                                        "id"
                                    ),
                                    "geometry_field": String(
                                        "wkb_geometry"
                                    ),
                                    "geometry_type": String(
                                        "POINT"
                                    ),
                                    "name": String(
                                        "points"
                                    ),"#;

    let expected_end = r#",
    "webserver": Table(
        {
            "bind": String(
                "0.0.0.0"
            ),
            "port": Integer(
                8080
            ),
            "threads": Integer(
                4
            )
        }
    )
}"#;
    assert!(format!("{:#?}", config.as_table().unwrap()).starts_with(expected_begin));
    assert!(format!("{:#?}", config.as_table().unwrap()).ends_with(expected_end));

    assert_eq!(config["datasource"]["type"].as_str(), Some("postgis"));
}

#[test]
fn test_parse_error() {
    let config = read_config("src/core/mod.rs");
    assert_eq!("src/core/mod.rs - unexpected character found: `/` at line 1",
               config.err().unwrap());

    let config = read_config("wrongfile");
    assert_eq!("Could not find config file!", config.err().unwrap());

    let config: Result<ApplicationCfg, _> = read_cfg("src/core/mod.rs");
    assert_eq!("src/core/mod.rs - unexpected character found: `/` at line 1",
               config.err().unwrap());

    let config: Result<ApplicationCfg, _> = read_cfg("wrongfile");
    assert_eq!("Could not find config file!", config.err().unwrap());
}
