use crate::rules;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub enum NodeType {
    // string inside of text
    Text(String),
    // container type
    Container(String),
    // document
    Document
}
pub struct Node {
    // whether node is text or container
    pub node_type: NodeType,
    // child nodes
    pub children: RefCell<Vec<Rc<Node>>>,
    // parent node
    parent: RefCell<Option<Rc<Node>>>,
    // parameters
    params: HashMap<String, String>,
    // css properties
    pub css: RefCell<HashMap<String, String>>,
}
impl Node {
    // empty document
    fn get_document() -> Node {
	return Node{node_type: NodeType::Document, children: RefCell::new(Vec::new()), parent: RefCell::new(None), params: HashMap::new(), css: RefCell::new(HashMap::new())}
    }
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
		} else if c == '\\' && !str_ignore_next {
		    str_ignore_next = true;
		} else {
		    word.push(c);
		    str_ignore_next = false;
		}
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
	Node{node_type: NodeType::Container(tag_name.to_string()), children: RefCell::new(Vec::new()), parent: RefCell::new(None), params: params, css: RefCell::new(HashMap::new())}
    }
    // get new text node from text
    fn from_text(text: String) -> Node {
	return Node{node_type: NodeType::Text(text), children: RefCell::new(Vec::new()), parent: RefCell::new(None), params: HashMap::new(), css: RefCell::new(HashMap::new())}
    }
    // checks if container node has end tag
    fn is_empty_element(&self) -> bool {
	match &self.node_type {
	    NodeType::Container(tag_name) => rules::EMPTY_ELEMENTS.iter().any(|e| e==&tag_name),
	    _ => false
	}
    }
    pub fn find_css(&self) -> String {
	let mut css = String::from("");
	match &self.node_type {
	    NodeType::Document => {
		for child in self.children.borrow().iter() {
		    css += &child.find_css();
		}
	    },
	    NodeType::Container(tag_name) => {
		if tag_name == &String::from("style") {
		    for child in self.children.borrow().iter() {
			match &child.node_type {
			    NodeType::Text(text) => css += text,
			    _ => {}
			}
		    }
		} else {
		    for child in self.children.borrow().iter() {
			css += &child.find_css();
		    }
		}
	    }
	    NodeType::Text(_) => {},
	}
	return css;
    }
    fn basic_selector_applies(&self, selector: String) -> bool {
	if selector == "*" {
	    return true;
	} else if selector.starts_with(".") {
	    let class_selector = selector.split_at(1).1.to_string();
	    return self.params.get("class") == Some(&class_selector);
	} else if selector.starts_with("#") {
	    let id_selector = selector.split_at(1).1.to_string();
	    return self.params.get("id") == Some(&id_selector);
	} else {
	    match &self.node_type {
		NodeType::Container(tag_name) => return &selector == tag_name,
		_ => return false
	    }
	}
    }
}
// print tree
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.node_type {
	    NodeType::Text(s) => write!(f, "{}\n", s),
	    NodeType::Container(tag_name) => {
		let mut printed = format!("{}", tag_name);
		if self.params.len() > 0 {
		    printed += "(";
		    for (param, value) in &self.params {
			printed += &format!("{}=\"{}\",", param, value);
		    }
		    printed.pop();
		    printed += ")";
		}
		if self.css.borrow().len() > 0 {
		    printed += "{";
		    for (key, value) in &*self.css.borrow() {
			printed += &format!("{}:{},", key, value);
		    }
		    printed.pop();
		    printed += "}";
		}
		if !rules::EMPTY_ELEMENTS.iter().any(|e| e==&tag_name) {
		    printed += ":";
		}
		printed += "\n";
		for child in self.children.borrow().iter() {
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
	    NodeType::Document => {
		let mut printed = String::from("");
		for child in self.children.borrow().iter() {
		    printed += &format!("{}", child);
		}
		write!(f, "{}", printed)
	    }
	}
    }
}

pub fn parse(html: String) -> Rc<Node> {
    // document to return
    let document = Rc::new(Node::get_document());
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
    // whether parser is in doctype declaration
    let mut in_doctype = false;
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
		} else if tag_content == "!--" && !in_str {
		    in_tag = false;
		    in_comment = true;
		    tag_content = "".to_string();
		} else if tag_content == "!DOCTYPE" && !in_str {
		    in_doctype = true;
		    tag_content = "".to_string();
		} else if c == '>' && !in_str {
		    in_tag = false;
		    if in_doctype {
			in_doctype = false;
		    } else {
			// adds new container node
			let node = Rc::new(Node::from_tag(tag_content.clone()));
			if !node.is_empty_element() {
			    current_containers.push(node);
			} else {
			    if current_containers.len() > 0 {
				let index = current_containers.len()-1;
				current_containers[index].children.borrow_mut().push(Rc::clone(&node));
				*node.parent.borrow_mut() = Some(Rc::clone(&current_containers[index])).to_owned();
			    } else {
				document.children.borrow_mut().push(Rc::clone(&node));
			    }
			}
		    }
		    tag_content = "".to_string();
		} else if !(in_str && c == str_char && !str_ignore_next) && !(!in_str && (c == '\'' || c == '\"')){
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
			current_containers[index].children.borrow_mut().push(Rc::clone(&node));
			*node.parent.borrow_mut() = Some(Rc::clone(&current_containers[index])).to_owned();
		    } else {
			document.children.borrow_mut().push(Rc::clone(&current_containers.remove(0)));
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
		    if text_content.trim() != "".to_string() {
			// adds new text node
			let node = Rc::new(Node::from_text(text_content));
			let index = current_containers.len()-1;
			current_containers[index].children.borrow_mut().push(Rc::clone(&node));
			*node.parent.borrow_mut() = Some(Rc::clone(&current_containers[index])).to_owned();

		    }
		    text_content = "".to_string();
		} else {
		    text_content.push(c);
		}
	    }
	}
    }
    return document;
}

// check wheter css selector applies
fn selector_applies(node: Rc<Node>, selector: String) -> bool {
    let mut descendants = selector.split(" ").collect::<Vec<&str>>();
    let mut applies = true;
    if node.basic_selector_applies(descendants[descendants.len()-1].to_string()) {
	descendants.pop();
	descendants.reverse();
    } else {
	return false;
    }
    let mut current_node = Rc::clone(&node);
    for basic_selector in descendants {
	let mut basic_applies = false;
	loop {
	    if current_node.basic_selector_applies(basic_selector.to_string()) {
		basic_applies = true;
		break;
	    }
	    match &*Rc::clone(&current_node).parent.borrow() {
		None => break,
		Some(n) => current_node = Rc::clone(&n),
	    }
	}
	if !basic_applies {
	    applies = false;
	    break;
	}
    }
    return applies;
}


// apply css to all nodes
pub fn apply_css(css_rules: Vec<(String, HashMap<String, String>)>, node: Rc<Node>) {
    // get to all the other nodes in tree
    for child in node.children.borrow().iter() {
	apply_css(css_rules.clone(), Rc::clone(child));
    }
    // applies rules
    for (selector, rules) in css_rules {
	if selector_applies(Rc::clone(&node), selector) {
	    for (key, value) in rules {
		node.css.borrow_mut().insert(key, value);
	    }
	}
    }
    // applies inline rules
    match &node.params.get("style") {
	None => {},
	Some(s) => {
	    let rules = s.split(";");
	    for rule in rules {
		let parts = rule.split(":").collect::<Vec<&str>>();
		if parts.len() == 2 {
		    let key = parts[0].trim().to_string();
		    let value = parts[1].trim().to_string();
		    node.css.borrow_mut().insert(key, value);
		}
	    }
	}
    }
}
