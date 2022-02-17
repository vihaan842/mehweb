// elements that don't have an end tag
pub const EMPTY_ELEMENTS: [&str;15] = [
    "area",
    "base",
    "br",
    "col",
    "embed",
    "hr",
    "img",
    "input",
    "keygen",
    "link",
    "meta",
    "param",
    "source",
    "track",
    "wbr"
];

// default css colors
pub const DEFAULT_COLORS: [(&str,&str);141] = [
    ("aliceblue","#F0F8FF"),
    ("antiquewhite","#FAEBD7"),
    ("aqua","#00FFFF"),
    ("aquamarine","#7FFFD4"),
    ("azure","#F0FFFF"),
    ("beige","#F5F5DC"),
    ("bisque","#FFE4C4"),
    ("black","#000000"),
    ("blanchedalmond","#FFEBCD"),
    ("blue","#0000FF"),
    ("blueviolet","#8A2BE2"),
    ("brown","#A52A2A"),
    ("burlywood","#DEB887"),
    ("cadetblue","#5F9EA0"),
    ("chartreuse","#7FFF00"),
    ("chocolate","#D2691E"),
    ("coral","#FF7F50"),
    ("cornflowerblue","#6495ED"),
    ("cornsilk","#FFF8DC"),
    ("crimson","#DC143C"),
    ("cyan","#00FFFF"),
    ("darkblue","#00008B"),
    ("darkcyan","#008B8B"),
    ("darkgoldenrod","#B8860B"),
    ("darkgray","#A9A9A9"),
    ("darkgreen","#006400"),
    ("darkkhaki","#BDB76B"),
    ("darkmagenta","#8B008B"),
    ("darkolivegreen","#556B2F"),
    ("darkorange","#FF8C00"),
    ("darkorchid","#9932CC"),
    ("darkred","#8B0000"),
    ("darksalmon","#E9967A"),
    ("darkseagreen","#8FBC8F"),
    ("darkslateblue","#483D8B"),
    ("darkslategray","#2F4F4F"),
    ("darkturquoise","#00CED1"),
    ("darkviolet","#9400D3"),
    ("deeppink","#FF1493"),
    ("deepskyblue","#00BFFF"),
    ("dimgray","#696969"),
    ("dodgerblue","#1E90FF"),
    ("firebrick","#B22222"),
    ("floralwhite","#FFFAF0"),
    ("forestgreen","#228B22"),
    ("fuchsia","#FF00FF"),
    ("gainsboro","#DCDCDC"),
    ("ghostwhite","#F8F8FF"),
    ("gold","#FFD700"),
    ("goldenrod","#DAA520"),
    ("gray","#7F7F7F"),
    ("green","#008000"),
    ("greenyellow","#ADFF2F"),
    ("honeydew","#F0FFF0"),
    ("hotpink","#FF69B4"),
    ("indianred","#CD5C5C"),
    ("indigo","#4B0082"),
    ("ivory","#FFFFF0"),
    ("khaki","#F0E68C"),
    ("lavender","#E6E6FA"),
    ("lavenderblush","#FFF0F5"),
    ("lawngreen","#7CFC00"),
    ("lemonchiffon","#FFFACD"),
    ("lightblue","#ADD8E6"),
    ("lightcoral","#F08080"),
    ("lightcyan","#E0FFFF"),
    ("lightgoldenrodyellow", "#FAFAD2"),
    ("lightgreen","#90EE90"),
    ("lightgrey","#D3D3D3"),
    ("lightpink","#FFB6C1"),
    ("lightsalmon","#FFA07A"),
    ("lightseagreen","#20B2AA"),
    ("lightskyblue","#87CEFA"),
    ("lightslategray","#778899"),
    ("lightsteelblue","#B0C4DE"),
    ("lightyellow","#FFFFE0"),
    ("lime","#00FF00"),
    ("limegreen","#32CD32"),
    ("linen","#FAF0E6"),
    ("magenta","#FF00FF"),
    ("maroon","#800000"),
    ("mediumaquamarine","#66CDAA"),
    ("mediumblue","#0000CD"),
    ("mediumorchid","#BA55D3"),
    ("mediumpurple","#9370DB"),
    ("mediumseagreen","#3CB371"),
    ("mediumslateblue","#7B68EE"),
    ("mediumspringgreen","#00FA9A"),
    ("mediumturquoise","#48D1CC"),
    ("mediumvioletred","#C71585"),
    ("midnightblue","#191970"),
    ("mintcream","#F5FFFA"),
    ("mistyrose","#FFE4E1"),
    ("moccasin","#FFE4B5"),
    ("navajowhite","#FFDEAD"),
    ("navy","#000080"),
    ("navyblue","#9FAFDF"),
    ("oldlace","#FDF5E6"),
    ("olive","#808000"),
    ("olivedrab","#6B8E23"),
    ("orange","#FFA500"),
    ("orangered","#FF4500"),
    ("orchid","#DA70D6"),
    ("palegoldenrod","#EEE8AA"),
    ("palegreen","#98FB98"),
    ("paleturquoise","#AFEEEE"),
    ("palevioletred","#DB7093"),
    ("papayawhip","#FFEFD5"),
    ("peachpuff","#FFDAB9"),
    ("peru","#CD853F"),
    ("pink","#FFC0CB"),
    ("plum","#DDA0DD"),
    ("powderblue","#B0E0E6"),
    ("purple","#800080"),
    ("red","#FF0000"),
    ("rosybrown","#BC8F8F"),
    ("royalblue","#4169E1"),
    ("saddlebrown","#8B4513"),
    ("salmon","#FA8072"),
    ("sandybrown","#FA8072"),
    ("seagreen","#2E8B57"),
    ("seashell","#FFF5EE"),
    ("sienna","#A0522D"),
    ("silver","#C0C0C0"),
    ("skyblue","#87CEEB"),
    ("slateblue","#6A5ACD"),
    ("slategray","#708090"),
    ("snow","#FFFAFA"),
    ("springgreen","#00FF7F"),
    ("steelblue","#4682B4"),
    ("tan","#D2B48C"),
    ("teal","#008080"),
    ("thistle","#D8BFD8"),
    ("tomato","#FF6347"),
    ("turquoise","#40E0D0"),
    ("violet","#EE82EE"),
    ("wheat","#F5DEB3"),
    ("white","#FFFFFF"),
    ("whitesmoke","#F5F5F5"),
    ("yellow","#FFFF00"),
    ("yellowgreen","#9ACD32"),
];

pub const INHERITED_PROPERTIES: [&str;37] = [
    "border-collapse",
    "border-spacing",
    "caption-side",
    "color",
    "cursor",
    "direction",
    "empty-cells",
    "font-family",
    "font-size",
    "font-style",
    "font-variant",
    "font-weight",
    "font-size-adjust",
    "font-stretch",
    "font",
    "letter-spacing",
    "line-height",
    "list-style-image",
    "list-style-position",
    "list-style-type",
    "list-style",
    "orphans",
    "quotes",
    "tab-size",
    "text-align",
    "text-align-last",
    "text-decoration-color",
    "text-indent",
    "text-justify",
    "text-shadow",
    "text-transform",
    "visibility",
    "white-space",
    "widows",
    "word-break",
    "word-spacing",
    "word-wrap",
];

// pub const DEFAULT_CSS: [(&str, &str);100] = [
// a:link 	color: (internal value);
// text-decoration: underline;
// cursor: auto; 	
// a:visited 	color: (internal value);
// text-decoration: underline;
// cursor: auto; 	
// a:link:active 	color: (internal value);
	
// a:visited:active 	color: (internal value);
	
// address 	display: block;
// font-style: italic; 	
// area 	display: none; 	
// article 	display: block; 	
// aside 	display: block; 	
// b 	font-weight: bold; 	
// bdo 	unicode-bidi: bidi-override; 	
// blockquote 	display: block;
// margin-top: 1em;
// margin-bottom: 1em;
// margin-left: 40px;
// margin-right: 40px; 	
// body 	display: block;
// margin: 8px; 	
// body:focus 	outline: none; 	
// caption 	display: table-caption;
// text-align: center; 	
// cite 	font-style: italic; 	
// code 	font-family: monospace; 	
// col 	display: table-column; 	
// colgroup 	display: table-column-group 	
// datalist 	display: none; 	
// dd 	display: block;
// margin-left: 40px; 	
// del 	text-decoration: line-through; 	
// details 	display: block; 	
// dfn 	font-style: italic; 	
// div 	display: block; 	
// dl 	display: block;
// margin-top: 1em;
// margin-bottom: 1em;
// margin-left: 0;
// margin-right: 0; 	
// dt 	display: block; 	
// em 	font-style: italic; 	
// embed:focus 	outline: none; 	
// fieldset 	display: block;
// margin-left: 2px;
// margin-right: 2px;
// padding-top: 0.35em;
// padding-bottom: 0.625em;
// padding-left: 0.75em;
// padding-right: 0.75em;
// border: 2px groove (internal value); 	
// figcaption 	display: block; 	
// figure 	display: block;
// margin-top: 1em;
// margin-bottom: 1em;
// margin-left: 40px;
// margin-right: 40px; 	
// footer 	display: block; 	
// form 	display: block;
// margin-top: 0em; 	
// h1 	display: block;
// font-size: 2em;
// margin-top: 0.67em;
// margin-bottom: 0.67em;
// margin-left: 0;
// margin-right: 0;
// font-weight: bold; 	
// h2 	display: block;
// font-size: 1.5em;
// margin-top: 0.83em;
// margin-bottom: 0.83em;
// margin-left: 0;
// margin-right: 0;
// font-weight: bold; 	
// h3 	display: block;
// font-size: 1.17em;
// margin-top: 1em;
// margin-bottom: 1em;
// margin-left: 0;
// margin-right: 0;
// font-weight: bold; 	
// h4 	display: block;
// margin-top: 1.33em;
// margin-bottom: 1.33em;
// margin-left: 0;
// margin-right: 0;
// font-weight: bold; 	
// h5 	display: block;
// font-size: .83em;
// margin-top: 1.67em;
// margin-bottom: 1.67em;
// margin-left: 0;
// margin-right: 0;
// font-weight: bold; 	
// h6 	display: block;
// font-size: .67em;
// margin-top: 2.33em;
// margin-bottom: 2.33em;
// margin-left: 0;
// margin-right: 0;
// font-weight: bold; 	
// head 	display: none; 	
// header 	display: block; 	
// hr 	display: block;
// margin-top: 0.5em;
// margin-bottom: 0.5em;
// margin-left: auto;
// margin-right: auto;
// border-style: inset;
// border-width: 1px; 	
// html 	display: block; 	
// html:focus 	outline: none; 	
// i 	font-style: italic; 	
// iframe:focus 	outline: none; 	
// iframe[seamless] 	display: block; 	
// img 	display: inline-block; 	
// ins 	text-decoration: underline; 	
// kbd 	font-family: monospace; 	
// label 	cursor: default; 	
// legend 	display: block;
// padding-left: 2px;
// padding-right: 2px;
// border: none; 	
// li 	display: list-item; 	
// link 	display: none; 	
// map 	display: inline; 	
// mark 	background-color: yellow;
// color: black; 	
// menu 	display: block;
// list-style-type: disc;
// margin-top: 1em;
// margin-bottom: 1em;
// margin-left: 0;
// margin-right: 0;
// padding-left: 40px; 	
// nav 	display: block; 	
// object:focus 	outline: none; 	
// ol 	display: block;
// list-style-type: decimal;
// margin-top: 1em;
// margin-bottom: 1em;
// margin-left: 0;
// margin-right: 0;
// padding-left: 40px; 	
// output 	display: inline; 	
// p 	display: block;
// margin-top: 1em;
// margin-bottom: 1em;
// margin-left: 0;
// margin-right: 0; 	
// param 	display: none; 	
// pre 	display: block;
// font-family: monospace;
// white-space: pre;
// margin: 1em 0; 	
// q 	display: inline; 	
// q::before 	content: open-quote; 	
// q::after 	content: close-quote; 	
// rt 	line-height: normal; 	
// s 	text-decoration: line-through; 	
// samp 	font-family: monospace; 	
// script 	display: none; 	
// section 	display: block; 	
// small 	font-size: smaller; 	
// strike 	text-decoration: line-through; 	
// strong 	font-weight: bold; 	
// style 	display: none; 	
// sub 	vertical-align: sub;
// font-size: smaller; 	
// summary 	display: block; 	
// sup 	vertical-align: super;
// font-size: smaller; 	
// table 	display: table;
// border-collapse: separate;
// border-spacing: 2px;
// border-color: gray; 	
// tbody 	display: table-row-group;
// vertical-align: middle;
// border-color: inherit; 	
// td 	display: table-cell;
// vertical-align: inherit; 	
// tfoot 	display: table-footer-group;
// vertical-align: middle;
// border-color: inherit; 	
// th 	display: table-cell;
// vertical-align: inherit;
// font-weight: bold;
// text-align: center; 	
// thead 	display: table-header-group;
// vertical-align: middle;
// border-color: inherit; 	
// title 	display: none; 	
// tr 	display: table-row;
// vertical-align: inherit;
// border-color: inherit; 	
// u 	text-decoration: underline; 	
// ul 	display: block;
// list-style-type: disc;
// margin-top: 1em;
// margin-bottom: 1 em;
// margin-left: 0;
// margin-right: 0;
// padding-left: 40px; 	
// var 	font-style: italic; 	
// ]
