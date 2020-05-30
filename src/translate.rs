use gtk::BuilderExt;
use glib::object::ObjectExt;
use std::collections::HashMap;

use gettextrs::{setlocale, LocaleCategory};

pub struct Translate{
    translated_map: HashMap<String, HashMap<String, String>>, //= HashMap::new();
    prev_translated : HashMap<String, String>,
    curr_locale : String,
}

impl Translate{
    pub fn translate_ui(&mut self, builder: &gtk::Builder)
    {
        //println!("translating ui");
        
        let obs = BuilderExt::get_objects(builder);
        let mut ts;
        for x in obs
        {
            if let Ok(lts) = x.get_property("label")
            {
                ts = self.get_translate(lts.get().unwrap().unwrap());

                
                if let Err(set_t) = x.set_property("label", &ts)
                {
                    println!("Error translating element: {}", set_t);
                }
            }
        }
        
    }

    pub fn set_lang(&mut self, lang_code: &str)
    {
        //let m = TRANSLATE_MAP.lock().unwrap();
        //let mut s = CURR_LOCALE.lock().unwrap();
        self.curr_locale.clear();
        if self.translated_map.contains_key(lang_code)
        {
            self.curr_locale.push_str(lang_code);
        }
        else {
            self.curr_locale.push_str("en");
        }
        
    }

    pub fn get_lang(&self) -> String
    {
        self.curr_locale.clone().to_string()
    }
    pub fn get_img_offset(&self) -> usize
    {
        match self.get_lang().as_str()
        {
            "en" => 0,
            "ru" => 5,
            "pl" => 10,
            _ => 0
        }
    }

    pub fn get_translate(&mut self, string_to_translate: String) -> String
    {
        //let m = TRANSLATE_MAP.lock().unwrap();
        //let mut m_p = PREV_TRANSLATED.lock().unwrap();
        //let s = CURR_LOCALE.lock().unwrap();
        
        let translation_map = self.translated_map.get(self.curr_locale.as_str()).unwrap();
        let ret;
        let tr_string = translation_map.get(&string_to_translate);
        
        //i have no idea how i go this to work.  it can find translations of strings previously translated
        if tr_string.is_some()
        {
            ret = tr_string.unwrap().to_string();
            
            self.prev_translated.insert((&ret).clone(), string_to_translate);
        }
        else if self.prev_translated.get(&string_to_translate).is_some()
        {
            let weird_translate = self.prev_translated.get(&string_to_translate).unwrap().to_string();
            //ret = get_translate(m_p.get(&string_to_translate).unwrap().to_string());  //this freezes the thing, i think its because of the global stuff
            ret = translation_map.get(&weird_translate).unwrap_or(&string_to_translate).to_string();
            self.prev_translated.insert((&ret).clone(), weird_translate);
        }
        else {
            
            ret = string_to_translate;
        }
        ret
    }

    pub fn load_translation() -> Translate
    {
        //create local variables
        let mut m : HashMap<String, HashMap<String, String>> = HashMap::new();
        let m_p : HashMap<String, String> = HashMap::new(); 
        let mut s = String::new();

        //load locale from system
        let lc = setlocale(LocaleCategory::LcMessages, "".to_owned()).unwrap_or("en_US.UTF-8".to_string());
        let lc_trunc;
        let lc_l = lc.to_ascii_lowercase().clone();
        if lc == "Polish_Poland.1250"
        {
            //windows reports polish locale as being "Polish_Poland.1250", which when truncated becomes
            //"po".  The standard UTF-8 string for polish is "pl.UTF-8", so we convert it to the utf 8 version
            lc_trunc = Some("pl");
            //maybe in the future if this becomes a gigantic pain to keep track of, I need to make like a windows to utf-8
            //locale converter
        }
        else {
            
            lc_trunc = lc_l.get(0..2);
        }
        println!("Locale is: {}", lc);

        //self.set_lang(lc_trunc.unwrap());

        //let mut m = TRANSLATE_MAP.lock().unwrap();
        use std::io::{prelude::*, BufReader};
        let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/locale/translations/").to_string();
        
        let entries = std::fs::read_dir(file_path);
        let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/locale/translations/").to_string();

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

            
            let buff = BufReader::new(file.unwrap());

            let mut line_iter = buff.lines();
            
            
            let mut line_t;
            let mut line_n_t;

            let mut lhm : HashMap<String, String> = HashMap::new();

            
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
        if m.contains_key(lc_trunc.unwrap())
        {
            s = lc_trunc.unwrap().to_string();
        }
        else {
            s = "en".to_string();
        }


        Translate{translated_map: m , prev_translated: m_p, curr_locale: s}
        /*
        for (orig, sts) in m.iter()
        {
            println!("{} {:?}", orig, sts);
        }*/ 
        
        
        
    }
}
/*
fn clear_prev()
{
    let mut m_p = PREV_TRANSLATED.lock().unwrap();
    m_p.clear();
}*/