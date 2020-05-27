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
    //println!("language to load: {}", file_path);
    //let buff;
    let mut file;
    if std::fs::metadata( file_path.clone() ).is_ok()
    {
        file = std::fs::File::open( file_path ).unwrap();
    }
    else {
        file = std::fs::File::open( concat!(env!("CARGO_MANIFEST_DIR"), "/locale/en.tr")).unwrap();
    }

    let buff = BufReader::new(file);
    //let buff = BufReader::new(file.unwrap());

    let mut line_iter = buff.lines();
    
    translate_map.clear();
    let mut line_t;
    let mut line_n_t;

    while let Some(line) = line_iter.next()
    {
        line_t = line.unwrap().clone();
        if line_t.trim().starts_with("msgid")
        {
            let v: Vec<&str> = line_t.trim().splitn(3, '\"').collect();
            while let Some(line_n) = line_iter.next()
            {
                line_n_t = line_n.unwrap().clone();
                if line_n_t.trim().starts_with("msgstr")
                {
                    let v_n: Vec<&str> = line_n_t.trim().splitn(3, '\"').collect();
                    translate_map.entry(String::from(v[1])).or_insert(String::from(v_n[1]));
                    break;
                }
            }
        }
        
    }
    
    /*
    for (orig, sts) in translate_map.iter()
    {
        println!("{} {}", orig, sts);
    }
    */
    
    
    
}