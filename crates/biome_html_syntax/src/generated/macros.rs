//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::HtmlSyntaxNode::kind(&node) {
                $crate::HtmlSyntaxKind::HTML_ASTRO_EXPRESSION => {
                    let $pattern = unsafe { $crate::HtmlAstroExpression::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ASTRO_EXPRESSION_ATTRIBUTE => {
                    let $pattern =
                        unsafe { $crate::HtmlAstroExpressionAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ASTRO_FRAGMENT => {
                    let $pattern = unsafe { $crate::HtmlAstroFragment::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ASTRO_FRAGMENT_CLOSE => {
                    let $pattern = unsafe { $crate::HtmlAstroFragmentClose::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ASTRO_FRAGMENT_OPEN => {
                    let $pattern = unsafe { $crate::HtmlAstroFragmentOpen::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ASTRO_FRONTMATTER_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::HtmlAstroFrontmatterElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ASTRO_SHORTHAND_ATTRIBUTE => {
                    let $pattern =
                        unsafe { $crate::HtmlAstroShorthandAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ASTRO_SPREAD_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::HtmlAstroSpreadAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ASTRO_TEMPLATE_LITERAL_ATTRIBUTE => {
                    let $pattern =
                        unsafe { $crate::HtmlAstroTemplateLiteralAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::HtmlAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE_INITIALIZER_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::HtmlAttributeInitializerClause::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE_NAME => {
                    let $pattern = unsafe { $crate::HtmlAttributeName::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_CDATA_SECTION => {
                    let $pattern = unsafe { $crate::HtmlCdataSection::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_CLOSING_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlClosingElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_COMMENT => {
                    let $pattern = unsafe { $crate::HtmlComment::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_CONTENT => {
                    let $pattern = unsafe { $crate::HtmlContent::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_DIRECTIVE => {
                    let $pattern = unsafe { $crate::HtmlDirective::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_OPENING_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlOpeningElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ROOT => {
                    let $pattern = unsafe { $crate::HtmlRoot::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_SELF_CLOSING_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlSelfClosingElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_STRING => {
                    let $pattern = unsafe { $crate::HtmlString::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_TAG_NAME => {
                    let $pattern = unsafe { $crate::HtmlTagName::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_BOGUS => {
                    let $pattern = unsafe { $crate::HtmlBogus::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_BOGUS_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::HtmlBogusAttribute::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_BOGUS_ELEMENT => {
                    let $pattern = unsafe { $crate::HtmlBogusElement::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ATTRIBUTE_LIST => {
                    let $pattern = unsafe { $crate::HtmlAttributeList::new_unchecked(node) };
                    $body
                }
                $crate::HtmlSyntaxKind::HTML_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::HtmlElementList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
