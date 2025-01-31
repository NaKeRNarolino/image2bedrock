use askama::Template;
use colored::*;
use std::{fs, path::PathBuf};

#[derive(Template)]
#[template(path = "block_def.json", escape = "none")]
struct BlockDefTemplate<'a> {
    pub pack_id: &'a str,
    pub block_id: &'a str,
    pub display_name: &'a str,
}

#[derive(Template)]
#[template(path = "glass.json", escape = "none")]
struct GlassTemplate<'a> {
    pub pack_id: &'a str,
    pub block_id: &'a str,
    pub display_name: &'a str,
}

#[derive(Template)]
#[template(path = "crop.json", escape = "none")]
struct CropTemplate<'a> {
    pub pack_id: &'a str,
    pub block_id: &'a str,
    pub display_name: &'a str,
}

#[derive(Template)]
#[template(path = "trapdoor.json", escape = "none")]
struct TrapdoorTemplate<'a> {
    pub pack_id: &'a str,
    pub block_id: &'a str,
    pub display_name: &'a str,
    pub texture: &'a str,
}

#[derive(Template)]
#[template(path = "glass_pane.json", escape = "none")]
struct GlassPaneTemplate<'a> {
    pub pack_id: &'a str,
    pub block_id: &'a str,
    pub display_name: &'a str,
    pub texture: &'a str,
}

#[derive(Template)]
#[template(path = "slab.json", escape = "none")]
struct SlabTemplate<'a> {
    pub pack_id: &'a str,
    pub block_id: &'a str,
    pub display_name: &'a str,
    pub texture: &'a str,
}

fn uppercase_words(data: &str) -> String {
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value);
            if value == ' ' {
                first = true;
            }
        }
    }
    result
}

fn workflow(id: String, name: String, path: PathBuf) {
    println!("{} {} {}", id, name, "PATH");

    if !path.is_dir() {
        println!("{} DIR DOESNT EXIST", path.display());
        return;
    }

    let dir_data = match path.read_dir() {
        Ok(data) => data,
        Err(_) => {
            println!("UNABLE TO READ DIR");
            return;
        }
    };

    match fs::create_dir_all("./result/blocks") {
        Ok(_) => "OK",
        Err(_) => "ERR",
    };

    match fs::create_dir_all("./result/textures/blocks") {
        Ok(_) => "OK",
        Err(_) => "ERR",
    };

    let mut terrain_atlas: Vec<String> = vec![];

    let mut blocksjson: Vec<String> = vec![];

    for file in dir_data {
        if let Ok(file) = file {
            if file.path().is_dir() {
                return;
            }

            let mut name = String::new();

            for entry in file
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase()
                .split("_")
            {
                let word = uppercase_words(entry);
                name.push_str(word.as_str());
                name.push_str(" ");
            }
            name.pop();

            // println!(
            //     "Checking > {}",
            //     file.file_name()
            //         .to_str()
            //         .unwrap()
            //         .to_lowercase()
            //         .bright_cyan()
            // );

            let file_ext = match file.path().extension() {
                Some(ext) => match ext.to_str() {
                    None => return,
                    Some(data) => data.to_lowercase(),
                },
                None => continue,
            };

            if file_ext == String::from("png") {
                print!(
                    "{} Is PNG > ",
                    file.file_name()
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                        .bright_magenta()
                );
                match fs::copy(
                    file.path().display().to_string(),
                    format!(
                        "./result/textures/blocks/{}",
                        file.file_name().to_str().unwrap().to_lowercase()
                    ),
                ) {
                    Ok(_) => "OK",
                    Err(_) => "ERR",
                };
                if file
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .contains("_glass")
                    || file
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                        .contains("_leaves")
                {
                    print!("{}", "Is Glass || Leaves > ".bright_yellow());
                    if file
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                        .contains("_glass")
                    {
                        print!("{}", "Is Glass so Making a Pane > ".bright_yellow());
                        match fs::write(
                            format!(
                                "./result/blocks/{}_pane.json",
                                file.path()
                                    .file_stem()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_lowercase()
                            ),
                            GlassPaneTemplate {
                                display_name: &name,
                                pack_id: &id,
                                block_id: format!(
                                    "{}_pane",
                                    &file
                                        .path()
                                        .file_stem()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_lowercase()
                                )
                                .as_str(),
                                texture: file
                                    .path()
                                    .file_stem()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_lowercase()
                                    .as_str(),
                            }
                            .render()
                            .unwrap(),
                        ) {
                            Ok(_) => (),
                            Err(_) => (),
                        }
                    }
                    match fs::write(
                        format!(
                            "./result/blocks/{}.json",
                            file.path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase()
                        ),
                        GlassTemplate {
                            pack_id: &id,
                            block_id: &file
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase(),
                            display_name: &name,
                        }
                        .render()
                        .unwrap(),
                    ) {
                        Ok(_) => "OK",
                        Err(_) => "ERR",
                    };
                } else if file
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .contains("_slab")
                {
                    print!("{}", "Is Slab > ".bright_yellow());
                    match fs::write(
                        format!(
                            "./result/blocks/{}.json",
                            file.path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase(),
                        ),
                        SlabTemplate {
                            pack_id: &id,
                            block_id: &file
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase(),
                            display_name: &name,
                            texture: file
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase()
                                .as_str(),
                        }
                        .render()
                        .unwrap(),
                    ) {
                        Ok(_) => "OK",
                        Err(_) => "ERR",
                    };
                } else if file
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .contains("_crop")
                    || file
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                        .contains("_mushroom")
                    || file
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                        .contains("_fungus")
                    || file
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                        .contains("_sapling")
                {
                    print!("{}", "Is Plant > ".bright_yellow());
                    match fs::write(
                        format!(
                            "./result/blocks/{}.json",
                            file.path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase()
                        ),
                        // format!(
                        //     "{{
                        //     \"format_version\": \"1.20.0\",
                        //     \"minecraft:block\": {{
                        //         \"description\": {{
                        //             \"identifier\": \"{}\",
                        //             \"menu_category\": {{
                        //                 \"category\": \"construction\"
                        //             }}
                        //         }},
                        //         \"components\": {{
                        //             \"minecraft:destructible_by_mining\": {{
                        //                 \"seconds_to_destroy\": 3
                        //             }},
                        //             \"minecraft:material_instances\": {{
                        //                 \"*\": {{
                        //                   \"render_method\": \"blend\"
                        //                 }}
                        //             }},
                        //             \"minecraft:display_name\": \"{}\"
                        //         }}
                        //     }}
                        // }}",
                        //     format!(
                        //         "{}:{}",
                        //         id,
                        //         file.path()
                        //             .file_stem()
                        //             .unwrap()
                        //             .to_str()
                        //             .unwrap()
                        //             .to_lowercase()
                        //     ),
                        //     name
                        // ),
                        CropTemplate {
                            display_name: &name,
                            pack_id: &id,
                            block_id: &file
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase(),
                        }
                        .render()
                        .unwrap(),
                    ) {
                        Ok(_) => "OK",
                        Err(_) => "ERR",
                    };
                } else if file
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .contains("_trapdoor")
                {
                    print!("{}", "Is Trapdoor > ".bright_yellow());
                    match fs::write(
                        format!(
                            "./result/blocks/{}.json",
                            file.path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase()
                        ),
                        TrapdoorTemplate {
                            display_name: &name,
                            pack_id: &id,
                            block_id: &file
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase(),
                            texture: file
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase()
                                .as_str(),
                        }
                        .render()
                        .unwrap(),
                    ) {
                        Ok(_) => "OK",
                        Err(_) => "ERR",
                    };
                } else {
                    print!("{}", "Is Regular > ".bright_yellow());
                    match fs::write(
                        format!(
                            "./result/blocks/{}.json",
                            file.path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase()
                        ),
                        BlockDefTemplate {
                            pack_id: &id,
                            block_id: &file
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase(),
                            display_name: &name,
                        }
                        .render()
                        .unwrap(),
                    ) {
                        Ok(_) => "OK",
                        Err(_) => "ERR",
                    };
                }

                println!(
                    "{}",
                    format!(
                        "Done! ({}:{})",
                        &id,
                        &file
                            .path()
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_lowercase(),
                    )
                    .bright_green()
                );

                let block_sound;

                if file
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .contains("_glass")
                {
                    block_sound = String::from("glass");
                } else if file
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .contains("_leaves")
                {
                    block_sound = String::from("grass");
                } else {
                    block_sound = String::from("stone");
                }

                terrain_atlas.push(format!(
                    "\"{}\": {{
                    \"textures\": \"textures/blocks/{}\"
                }}",
                    file.path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase(),
                    file.path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                ));

                blocksjson.push(format!(
                    "\"{}:{}\": {{
                    \"sound\": \"{}\",
                    \"textures\": \"{}\"
                }}",
                    id,
                    file.path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase(),
                    block_sound,
                    file.path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                ));
            }
        }
    }
    let mut terrain_atlas_content = format!(
        "{{
        \"num_mip_levels\": 4,
        \"padding\": 8,
        \"resource_pack_name\": \"{}\",
        \"texture_name\": \"atlas.terrain\",
        \"texture_data\": {{",
        name
    );
    for item in terrain_atlas {
        terrain_atlas_content.push_str(item.as_str());
        terrain_atlas_content.push_str(",");
    }
    terrain_atlas_content.pop();
    terrain_atlas_content.push_str(
        "}
}",
    );

    let mut blocksjson_content = String::from(
        "{
        \"format_version\": [
            1,
            1,
            0
        ],",
    );

    for item in blocksjson {
        blocksjson_content.push_str(item.as_str());
        blocksjson_content.push_str(",");
    }
    blocksjson_content.pop();
    blocksjson_content.push_str("}");

    let resq = match fs::write("./result/terrain_texture.json", terrain_atlas_content) {
        Ok(_) => String::from("OK!"),
        Err(e) => e.to_string(),
    };
    println!("Terrain Atlas > {}", resq.bright_green());

    let resq = match fs::write("./result/blocks.json", blocksjson_content) {
        Ok(_) => String::from("OK!"),
        Err(e) => e.to_string(),
    };
    println!("Blocks.json > {}", resq.bright_green());
}

fn main() {
    println!("Enter ID: ");
    let mut res1: String = String::new();
    std::io::stdin().read_line(&mut res1).unwrap();
    println!("Enter RP Name: ");
    let mut res2: String = String::new();
    std::io::stdin().read_line(&mut res2).unwrap();
    workflow(
        res1.trim().to_owned(),
        res2.trim().to_owned(),
        PathBuf::from("./i2bdata/"),
    );
}
