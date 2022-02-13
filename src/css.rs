use std::collections::HashMap;

pub fn parse(css: String) -> HashMap<String, HashMap<String, String>> {
    let mut rules = HashMap::new();
    let mut params = HashMap::new();
    let mut in_selector = true;
    let mut selector_content = String::from("");
    let mut in_key = true;
    let mut key = String::from("");
    let mut value = String::from("");
    for c in css.trim().chars() {
	if in_selector {
	    if c == '{' {
		in_selector = false;
		in_key = true;
	    } else {
		selector_content.push(c);
	    }
	} else if in_key {
	    if c == ':' {
		in_key = false;
	    } else if c == '}'{
		rules.insert(selector_content.trim().to_string(), params.clone());
		params = HashMap::new();
		selector_content = "".to_string();
		in_selector = true;
	    } else {
		key.push(c);
	    }
	} else {
	    if c == ';' {
		in_key = true;
		params.insert(key.trim().to_string(), value.trim().to_string());
		key = "".to_string();
		value = "".to_string();
	    } else {
		value.push(c);
	    }
	}
    }
    return rules;
}
