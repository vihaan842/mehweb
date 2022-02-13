use std::collections::HashMap;

pub fn parse(css: String) -> Vec<(String, HashMap<String, String>)> {
    let mut rules = Vec::new();
    let mut params = HashMap::new();
    let mut in_selector = true;
    let mut selector_content = String::from("");
    let mut in_key = true;
    let mut in_comment = false;
    let mut comment_content = String::from("");
    let mut key = String::from("");
    let mut value = String::from("");
    for c in css.trim().chars() {
	if in_comment {
	    comment_content.push(c);
	    if comment_content.ends_with("*/") {
		in_comment = false;
		comment_content = "".to_string();
	    }
	} else if in_selector {
	    if selector_content.ends_with("/*") {
		in_comment = true;
		selector_content.pop();
		selector_content.pop();
	    } else if c == '{' {
		in_selector = false;
		in_key = true;
	    } else {
		selector_content.push(c);
	    }
	} else if in_key {
	    if key.ends_with("/*") {
		in_comment = true;
		key.pop();
		key.pop();
	    } else if c == ':' {
		in_key = false;
	    } else if c == '}'{
		rules.push((selector_content.trim().to_string(), params.clone()));
		params = HashMap::new();
		selector_content = "".to_string();
		in_selector = true;
	    } else {
		key.push(c);
	    }
	} else {
	    if value.ends_with("/*") {
		in_comment = true;
		value.pop();
		value.pop();
	    } else if c == ';' {
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
