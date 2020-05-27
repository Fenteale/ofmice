use gtk::*;
use glib::object::ObjectExt;
use std::collections::HashMap;

use std::fs::*;
use std::iter::FilterMap;
use lazy_static::lazy_static;
use std::sync::Mutex;


lazy_static! {
    static ref TRANSLATE_MAP: Mutex<HashMap<String, HashMap<String, String>>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

lazy_static! {
    static ref PREV_TRANSLATED: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

lazy_static! {
    static ref CURR_LOCALE: Mutex<String> = {
        let mut s = String::new();
        Mutex::new(s)
    };
}

fn clear_prev()
{
    let mut m_p = PREV_TRANSLATED.lock().unwrap();
    m_p.clear();
}

pub fn translate_ui(builder: &gtk::Builder)
{
    //println!("translating ui");
    
    let obs = BuilderExt::get_objects(builder);
    let mut ts;
    for x in obs
    {
        if let Ok(lts) = x.get_property("label")
        {
            ts = get_translate(lts.get().unwrap().unwrap());

            //println!("{}", ts);
            if let Err(set_t) = x.set_property("label", &ts)
            {
                println!("Error translating element: {}", set_t);
            }
        }
    }
    //clear_prev();
    //m.clear()
}

pub fn set_lang(lang_code: &str)
{
    let mut m = TRANSLATE_MAP.lock().unwrap();
    let mut s = CURR_LOCALE.lock().unwrap();
    s.clear();
    if m.contains_key(lang_code)
    {
        s.push_str(lang_code);
    }
    else {
        s.push_str("en");
    }
    
}

pub fn get_translate(string_to_translate: String) -> String
{
    let mut m = TRANSLATE_MAP.lock().unwrap();
    let mut m_p = PREV_TRANSLATED.lock().unwrap();
    let mut s = CURR_LOCALE.lock().unwrap();
    //let translated_string = m.get(&string_to_translate).unwrap_or(&string_to_translate).to_string();
    let translation_map = m.get(s.as_str()).unwrap();
    let ret;
    let tr_string = translation_map.get(&string_to_translate);
    //println!("Got to if block");
    if tr_string.is_some()//unwrap_or(&string_to_translate).to_string()
    {
        ret = tr_string.unwrap().to_string();
        //println!("Translating {}", ret);
        m_p.insert((&ret).clone(), string_to_translate);
    }
    else if m_p.get(&string_to_translate).is_some()
    {
        let weird_translate = m_p.get(&string_to_translate).unwrap().to_string();
        //ret = get_translate(m_p.get(&string_to_translate).unwrap().to_string());  //this freezes the thing, i think its because of the global stuff
        ret = translation_map.get(&weird_translate).unwrap_or(&string_to_translate).to_string();
        m_p.insert((&ret).clone(), weird_translate);
    }
    else {
        //println!("else");
        ret = string_to_translate;
    }
    ret
}

pub fn load_translation()
{
    let mut m = TRANSLATE_MAP.lock().unwrap();
    use std::io::{prelude::*, BufReader};
    let mut file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/locale/translations/").to_string();
    
    let entries = std::fs::read_dir(file_path);
    let mut file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/locale/translations/").to_string();

    let lang_files = entries.unwrap().filter_map(|entry| {
        entry.ok().and_then(|e| e.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s))))
    }).collect::<Vec<String>>();

    m.clear();
    for loc in lang_files
    {
        let v_loc: Vec<&str> = loc.split(".").collect();
        let mut file_to_load = String::new();
        file_to_load.push_str(file_path.as_str());
        file_to_load.push_str(v_loc[0]);
        file_to_load.push_str(".tr");
        //println!("{}", file_to_load);
        let file = std::fs::File::open( file_to_load );

        //let buff = BufReader::new(file);
        let buff = BufReader::new(file.unwrap());

        let mut line_iter = buff.lines();
        
        
        let mut line_t;
        let mut line_n_t;

        let mut lhm : HashMap<String, String> = HashMap::new();

        //for 
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
                        lhm.entry(String::from(v[1])).or_insert(String::from(v_n[1]));
                        
                        break;
                    }
                }
            }
            
        }
        m.entry(String::from(v_loc[0])).or_insert(lhm);
    }
    
    /*
    for (orig, sts) in m.iter()
    {
        println!("{} {:?}", orig, sts);
    }*/ 
    
    
    
}