use crate::rules;
use crate::renderer::layout::LayoutBox;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub enum NodeType {
    // string inside of text
    Text(String),
    // container type, children, and params
    Container(String, RefCell<Vec<Rc<Node>>>, HashMap<String, String>),
    // document
    Document(RefCell<Vec<Rc<Node>>>)
}
pub struct Node {
    // whether node is text or container
    pub node_type: NodeType,
    // parent node
    parent: RefCell<Option<Rc<Node>>>,
    // css properties
    pub css: RefCell<HashMap<String, String>>,
    // layout render
    pub render: Rc<RefCell<LayoutBox>>,
}
impl Node {
    // empty document
    fn get_document() -> Node {
	return Node{node_type: NodeType::Document(RefCell::new(Vec::new())), parent: RefCell::new(None), css: RefCell::new(HashMap::new()), render: Rc::new(RefCell::new(LayoutBox::empty()))}
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
	    // make sure that this is actually a parameter
	    if param_parts.len() == 2 {
		params.insert(param_parts[0].to_string(), param_parts[1].to_string());
	    }
	}
	Node{node_type: NodeType::Container(tag_name.to_string(), RefCell::new(Vec::new()), params), parent: RefCell::new(None), css: RefCell::new(HashMap::new()), render: Rc::new(RefCell::new(LayoutBox::empty()))}
    }
    // gets children, if there are any
    pub fn children(&self) -> &RefCell<Vec<Rc<Node>>> {
	match &self.node_type {
	    NodeType::Document(children) => children,
	    NodeType::Container(_, children, _) => children,
	    NodeType::Text(_) => panic!("Node has no children!"),
	}
    }
    // get new text node from text
    fn from_text(text: String) -> Node {
	return Node{node_type: NodeType::Text(text), parent: RefCell::new(None), css: RefCell::new(HashMap::new()), render: Rc::new(RefCell::new(LayoutBox::empty()))}
    }
    // checks if container node has end tag
    fn is_empty_element(&self) -> bool {
	match &self.node_type {
	    NodeType::Container(tag_name, _, _) => rules::EMPTY_ELEMENTS.iter().any(|e| e==&tag_name),
	    _ => false
	}
    }
    // gets css from <style> tags
    pub fn find_css(&self) -> String {
	let mut css = String::from("");
	match &self.node_type {
	    NodeType::Document(children) => {
		for child in children.borrow().iter() {
		    css += &child.find_css();
		}
	    },
	    NodeType::Container(tag_name, children, _) => {
		if tag_name == &String::from("style") {
		    for child in children.borrow().iter() {
			match &child.node_type {
			    NodeType::Text(text) => css += text,
			    _ => {}
			}
		    }
		} else {
		    for child in children.borrow().iter() {
			css += &child.find_css();
		    }
		}
	    }
	    NodeType::Text(_) => {},
	}
	return css;
    }
    // figures out if a basic selector (tag name, class name, id, etc.) applies
    fn basic_selector_applies(&self, selector: String) -> bool {
	match &self.node_type {
	    NodeType::Container(tag_name, _, params) => {
		if selector == "*" {
		    return true;
		} if selector.starts_with(".") {
		    let class_selector = selector.split_at(1).1.to_string();
		    return params.get("class") == Some(&class_selector);
		} else if selector.starts_with("#") {
		    let id_selector = selector.split_at(1).1.to_string();
		    return params.get("id") == Some(&id_selector);
		} else {
		    return &selector == tag_name;
		}
	    },
	    _ => false
	}
    }
}
// print tree for debugging
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.node_type {
	    NodeType::Text(s) => write!(f, "{}\n", s),
	    NodeType::Container(tag_name, children, params) => {
		let mut printed = format!("{}", tag_name);
		if params.len() > 0 {
		    printed += "(";
		    for (param, value) in params {
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
		for child in children.borrow().iter() {
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
	    NodeType::Document(children) => {
		let mut printed = String::from("");
		for child in children.borrow().iter() {
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
		if c == '/' && tag_content == "".to_string() {
		    tag_open = false;
		} else if tag_content == "!--" && !in_str {
		    in_tag = false;
		    in_comment = true;
		    tag_content = "".to_string();
		} else if tag_content.to_uppercase() == "!DOCTYPE" && !in_str {
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
				current_containers[index].children().borrow_mut().push(Rc::clone(&node));
				*node.parent.borrow_mut() = Some(Rc::clone(&current_containers[index])).to_owned();
			    } else {
				document.children().borrow_mut().push(Rc::clone(&node));
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
			current_containers[index].children().borrow_mut().push(Rc::clone(&node));
			*node.parent.borrow_mut() = Some(Rc::clone(&current_containers[index])).to_owned();
		    } else {
			document.children().borrow_mut().push(Rc::clone(&current_containers.remove(0)));
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
			current_containers[index].children().borrow_mut().push(Rc::clone(&node));
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
    match &node.node_type {
	NodeType::Document(children) => {
	    // get to all the other nodes in tree
	    for child in children.borrow().iter() {
		apply_css(css_rules.clone(), Rc::clone(child));
	    }
	},
	NodeType::Container(tag_name, children, params) => {
	    // applies default rules
	    let default = rules::DEFAULT_CSS.iter().find(|t| t.0 == tag_name);
	    match default {
		Some((_, default)) => {
		    let rules = default.split(";");
		    for rule in rules {
			let parts = rule.split(":").collect::<Vec<&str>>();
			if parts.len() == 2 {
			    let key = parts[0].trim().to_string();
			    let value = parts[1].trim().to_string();
			    apply_css_rule(Rc::clone(&node), key, value);
			}
		    }
		},
		None => {},
	    }
	    // applies rules
	    for (selector, rules) in css_rules.clone() {
		if selector_applies(Rc::clone(&node), selector) {
		    for (key, value) in rules {
			apply_css_rule(Rc::clone(&node), key, value);
		    }
		}
	    }
	    // applies inline rules
	    match params.get("style") {
		None => {},
		Some(s) => {
		    let rules = s.split(";");
		    for rule in rules {
			let parts = rule.split(":").collect::<Vec<&str>>();
			if parts.len() == 2 {
			    let key = parts[0].trim().to_string();
			    let value = parts[1].trim().to_string();
			    apply_css_rule(Rc::clone(&node), key, value);
			}
		    }
		}
	    }
	    // get to all the other nodes in tree
	    for child in children.borrow().iter() {
		apply_css(css_rules.clone(), Rc::clone(child));
	    }
	},
	_ => {}
    }
}
fn apply_css_rule(node: Rc<Node>, key: String, value: String) {
    match &node.node_type {
	NodeType::Container(_, children, _) => {
	    if rules::INHERITED_PROPERTIES.iter().any(|e| e==&key) {
		for child in children.borrow_mut().iter() {
		    apply_css_rule(Rc::clone(child), key.clone(), value.clone());
		}
	    }
	},
	_ => {},
    }
    node.css.borrow_mut().insert(key, value);
}
