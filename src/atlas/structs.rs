use std::{collections::HashMap, ops::Deref};

#[derive(Debug)]
pub struct Atlas<'a> {
    pub file: &'a str,
    pub format: &'a str,
    pub filter: (Filter,Filter),
    pub repeat: bool,
    pub items: HashMap<String, Item>,
}

#[derive(Debug)]
pub struct Item {
    pub rotate: bool,
    pub xy: (i32,i32),
    pub size: (i32,i32),
    pub orig: (i32,i32),
    pub offset: (i32,i32),
    pub index: Option<u32>,
}

#[derive(Debug)]
pub enum Filter {
    Linear,
    Nearest,
}



impl<'a> Atlas<'a> {
    pub fn parse(data: &str) -> Result<Self,String> {
        let lines = data.lines().collect::<Vec<&str>>();
        let start: u16 = 0;
        let start = lines.iter().enumerate().find(|&(_, line)| !line.is_empty()).map(|(i, _)| i).unwrap_or(0);
        let atlas = Atlas {
            file: if (lines[start].ends_with(".png") || lines[start].ends_with(".jpeg")) && lines[start].chars().take_while(|c| *c !='.').collect::<String>().len() != 0 {
                        lines[start] 
                    } else { 
                        return Err("Invalid file format must be png or jpeg".to_string()) 
                    },
            format: match find_field(lines.clone(), "format:") {
                        Some((_, value)) => value.as_str().trim(),
                        None => return Err("field \"format\" not found".to_string()),
                    },

        };

        todo!()
    }
}

fn find_field(lines: Vec<&str>,field: &str) -> Option<(u32,String)> {
    for line in lines.iter().enumerate() {
        if line.1.starts_with(field) {
            return Some((line.0 as u32,line.1.to_string()));
        };
        if !line.1.contains(":") && !line.1.is_empty() {
            return None;
        }
    };
    None
}

impl <'a> Deref for Atlas<'a> {
    type Target = HashMap<String, Item>;
    fn deref(&self) -> &Self::Target {
        return &self.items;
    }
}

