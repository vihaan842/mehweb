use crate::rules;

use std::collections::HashMap;

pub enum Node {
    // string inside of text
    Text(String),
    // container type and contents
    Container{tag_name: String, children: Vec<Node>, params: HashMap<String, String>},
}
impl Node {
    // get new container node from tag
    fn from_tag(tag_content: String) -> Node {
	let mut parts = Vec::new();
	let mut word = String::from("");
	let mut in_str = false;
	let mut str_char = '\"';
	let mut str_ignore_next = false;
	for c in tag_content.chars() {
	    if in_str {
		if c == str_char && !str_ignore_next {
		    in_str = true;
		} else {
		    word.push(c);
		}
		str_ignore_next = c == '\\' && !str_ignore_next;
	    } else {
		if c == '\'' || c == '\"' {
		    in_str = true;
		    str_char = c.clone();
		} else if c == ' ' {
		    parts.push(word.clone());
		    word = "".to_string();
		} else {
		    word.push(c);
		}
	    }
	}
	parts.push(word.clone());
	let tag_name = parts.remove(0);
	let mut params = HashMap::new();
	for param in parts {
	    let param_parts = param.splitn(2, "=").collect::<Vec<&str>>();
	    params.insert(param_parts[0].to_string(), param_parts[1].to_string());
	}
	Node::Container{tag_name: tag_name.to_string(), children: Vec::new(), params: params}
    }
    // adds child node if node is container
    fn add_child(&mut self, child: Node) {
	match self {
	    Node::Container{tag_name:_, children, params:_} => children.push(child),
	    _ => {},
	}
    }
    // checks if container node has end tag
    fn is_empty_element(&self) -> bool {
	match self {
	    Node::Container{tag_name, children:_, params:_} => rules::EMPTY_ELEMENTS.iter().any(|e| e==tag_name),
	    _ => false
	}
    }
}
// print tree
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
	    Node::Text(s) => write!(f, "{}\n", s),
	    Node::Container{tag_name, children, params} => {
		let mut printed = format!("{}", tag_name);
		if params.len() > 0 {
		    printed += "(";
		    for (param, value) in params {
			printed += &format!("{}=\"{}\",", param, value);
		    }
		    printed.pop();
		    printed += ")";
		}
		if !rules::EMPTY_ELEMENTS.iter().any(|e| e==tag_name) {
		    printed += ":";
		}
		printed += "\n";
		for child in children {
		    // make sure that everything is indented
		    for line in format!("{}", child).split("\n") {
			// ignore whitespace
			if line.chars().filter(|c| !c.is_whitespace()).collect::<String>() != "" {
			    printed += &format!("  {}\n", line);
			}
		    }
		}
		write!(f, "{}", printed)
	    },
	}
    }
}

pub fn parse(html: String) -> Vec<Node> {
    // tree to return
    let mut tree = Vec::new();
    // vec containing containers parser is currently in
    let mut current_containers = Vec::new();
    // whether parser is currently inside of tag
    let mut in_tag = false;
    // whether this tag is an open tag
    let mut tag_open = true;
    // text content inside of tag
    let mut tag_content = String::from("");
    // text content inside of containers
    let mut text_content = String::from("");
    // whether parser is in comment
    let mut in_comment = false;
    // stuff inside of comment (to detect end)
    let mut comment_content = String::from("");
    // special string stuff
    let mut in_str = false;
    let mut str_char = '\"';
    let mut str_ignore_next = false;
    // goes through all characters
    for c in html.chars() {
	if in_tag {
	    if tag_open {
		if in_str {
		    if c == str_char && !str_ignore_next {
			in_str = false;
		    }
		    str_ignore_next = c == '\\' && !str_ignore_next;
		} else {
		    if c == '\'' || c == '\"' {
			in_str = true;
			str_char = c.clone();
		    }
		}
		if c == '/' && !in_str {
		    tag_open = false;
		} else if tag_content == "!--" && !in_str{
		    in_tag = false;
		    in_comment = true;
		    tag_content = "".to_string();
		} else if c == '>' && !in_str {
		    in_tag = false;
		    // adds new container node
		    let node = Node::from_tag(tag_content.clone());
		    if !node.is_empty_element() {
			current_containers.push(node);
		    } else {
			if current_containers.len() > 0 {
			    let index = current_containers.len()-1;
			    current_containers[index].add_child(node);
			} else {
			    tree.push(node);
			}
		    }
		    tag_content = "".to_string();
		} else {
		    tag_content.push(c);
		}
	    } else {
		if c == '>' {
		    in_tag = false;
		    // checks if there are multiple containers
		    if current_containers.len() > 1 {
			// removes node but adds as child to parent node
			let index = current_containers.len()-2;
			let node = current_containers.remove(current_containers.len()-1);
			current_containers[index].add_child(node);
		    } else {
			tree.push(current_containers.remove(0));
		    }
		    tag_content = "".to_string();
		} else {
		    tag_content.push(c);
		}
	    }
	} else {
	    if in_comment {
		comment_content.push(c);
		if comment_content.ends_with("-->") {
		    in_comment = false;
		    comment_content = "".to_string();
		}
	    } else {
		if c == '<' {
		    in_tag = true;
		    tag_open = true;
		    if text_content != "".to_string() {
			// adds new text node
			let index = current_containers.len()-1;
			current_containers[index].add_child(Node::Text(text_content));
		    }
		    text_content = "".to_string();
		} else {
		    text_content.push(c);
		}
	    }
	}
    }
    return tree;
}
