use gtk::*;
use glib::object::ObjectExt;
use std::collections::HashMap;



pub fn translate_ui(builder: &gtk::Builder, translate_map: &mut HashMap<String, String>)
{
    let obs = BuilderExt::get_objects(builder);
    for x in obs
    {
        if let Ok(lts) = x.get_property("label")
        {
            if let Err(set_t) = x.set_property("label", &get_translate(lts.get().unwrap().unwrap(), translate_map))
            {
                println!("Error translating element: {}", set_t);
            }
        }
    }
}

pub fn get_translate(string_to_translate: String, translate_map: &mut HashMap<String, String>) -> String
{
    translate_map.get(&string_to_translate).unwrap_or(&string_to_translate).to_string()
}

pub fn load_translation(lang_code: &str, translate_map: &mut HashMap<String, String>)
{
    use std::io::{prelude::*, BufReader};
    let mut file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/locale/").to_string();
    file_path.push_str(lang_code);
    file_path.push_str(".tr");
    println!("language to load: {}", file_path);
    let file = std::fs::File::open( file_path );
    let buff = BufReader::new(file.unwrap());

    let mut line_iter = buff.lines();
    
    translate_map.clear();
    
    while let Some(line) = line_iter.next()
    {
        if let Some(line_n) = line_iter.next()
        {
            translate_map.entry(line.unwrap()).or_insert(line_n.unwrap());
        }
        
    }
    /*
    for (orig, sts) in translate_map.iter()
    {
        println!("{} {}", orig, sts);
    }*/ 
    
    
    
}