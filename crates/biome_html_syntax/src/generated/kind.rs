//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum HtmlSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    L_ANGLE,
    R_ANGLE,
    SLASH,
    EQ,
    BANG,
    MINUS,
    COMMENT_START,
    COMMENT_END,
    CDATA_START,
    CDATA_END,
    FENCE,
    L_CURLY,
    R_CURLY,
    DOT_DOT_DOT,
    BACKTICK,
    NULL_KW,
    TRUE_KW,
    FALSE_KW,
    DOCTYPE_KW,
    HTML_KW,
    HTML_STRING_LITERAL,
    HTML_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    IDENT,
    HTML_IDENT,
    HTML_ROOT,
    HTML_DIRECTIVE,
    HTML_SELF_CLOSING_TAG,
    HTML_ELEMENT,
    HTML_ASTRO_FRONTMATTER_ELEMENT,
    HTML_ASTRO_EXPRESSION,
    HTML_ASTRO_FRAGMENT,
    HTML_ASTRO_FRAGMENT_OPEN,
    HTML_ASTRO_FRAGMENT_CLOSE,
    HTML_ASTRO_SHORTHAND_ATTRIBUTE,
    HTML_ASTRO_SPREAD_ATTRIBUTE,
    HTML_ASTRO_EXPRESSION_ATTRIBUTE,
    HTML_ASTRO_TEMPLATE_LITERAL_ATTRIBUTE,
    HTML_OPENING_ELEMENT,
    HTML_CLOSING_ELEMENT,
    HTML_SELF_CLOSING_ELEMENT,
    HTML_ATTRIBUTE,
    HTML_ATTRIBUTE_INITIALIZER_CLAUSE,
    HTML_STRING,
    HTML_TAG_NAME,
    HTML_ATTRIBUTE_NAME,
    HTML_ELEMENT_LIST,
    HTML_ATTRIBUTE_LIST,
    HTML_CONTENT,
    HTML_JS_CONTENT,
    HTML_TEMPLATE_LITERAL_CONTENT,
    HTML_COMMENT,
    HTML_CDATA_SECTION,
    HTML_BOGUS,
    HTML_BOGUS_ELEMENT,
    HTML_BOGUS_ATTRIBUTE,
    #[doc(hidden)]
    __LAST,
}
use self::HtmlSyntaxKind::*;
impl HtmlSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            L_ANGLE
                | R_ANGLE
                | SLASH
                | EQ
                | BANG
                | MINUS
                | COMMENT_START
                | COMMENT_END
                | CDATA_START
                | CDATA_END
                | FENCE
                | L_CURLY
                | R_CURLY
                | DOT_DOT_DOT
                | BACKTICK
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(self, HTML_STRING_LITERAL | HTML_LITERAL)
    }
    pub const fn is_list(self) -> bool {
        matches!(self, HTML_ELEMENT_LIST | HTML_ATTRIBUTE_LIST)
    }
    pub fn from_keyword(ident: &str) -> Option<Self> {
        let kw = match ident {
            "null" => NULL_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            "doctype" => DOCTYPE_KW,
            "html" => HTML_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            L_ANGLE => "<",
            R_ANGLE => ">",
            SLASH => "/",
            EQ => "=",
            BANG => "!",
            MINUS => "-",
            COMMENT_START => "<!--",
            COMMENT_END => "-->",
            CDATA_START => "<![CDATA[",
            CDATA_END => "]]>",
            FENCE => "---",
            L_CURLY => "{",
            R_CURLY => "}",
            DOT_DOT_DOT => "...",
            BACKTICK => "`",
            NULL_KW => "null",
            TRUE_KW => "true",
            FALSE_KW => "false",
            DOCTYPE_KW => "doctype",
            HTML_KW => "html",
            EOF => "EOF",
            HTML_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [<] => { $ crate :: HtmlSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: HtmlSyntaxKind :: R_ANGLE } ; [/] => { $ crate :: HtmlSyntaxKind :: SLASH } ; [=] => { $ crate :: HtmlSyntaxKind :: EQ } ; [!] => { $ crate :: HtmlSyntaxKind :: BANG } ; [-] => { $ crate :: HtmlSyntaxKind :: MINUS } ; [<!--] => { $ crate :: HtmlSyntaxKind :: COMMENT_START } ; [-->] => { $ crate :: HtmlSyntaxKind :: COMMENT_END } ; ["<![CDATA["] => { $ crate :: HtmlSyntaxKind :: CDATA_START } ; ["]]>"] => { $ crate :: HtmlSyntaxKind :: CDATA_END } ; [---] => { $ crate :: HtmlSyntaxKind :: FENCE } ; ['{'] => { $ crate :: HtmlSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: HtmlSyntaxKind :: R_CURLY } ; [...] => { $ crate :: HtmlSyntaxKind :: DOT_DOT_DOT } ; ['`'] => { $ crate :: HtmlSyntaxKind :: BACKTICK } ; [html_js_content] => { $ crate :: HtmlSyntaxKind :: HTML_JS_CONTENT } ; [html_template_literal_content] => { $ crate :: HtmlSyntaxKind :: HTML_TEMPLATE_LITERAL_CONTENT } ; [null] => { $ crate :: HtmlSyntaxKind :: NULL_KW } ; [true] => { $ crate :: HtmlSyntaxKind :: TRUE_KW } ; [false] => { $ crate :: HtmlSyntaxKind :: FALSE_KW } ; [doctype] => { $ crate :: HtmlSyntaxKind :: DOCTYPE_KW } ; [html] => { $ crate :: HtmlSyntaxKind :: HTML_KW } ; [ident] => { $ crate :: HtmlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: HtmlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: HtmlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: HtmlSyntaxKind :: HASH } ; }
