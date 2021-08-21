use lazy_static::*;
use sexpr_ir::syntax::sexpr::one_unit_parse;
use sexpr_process::pattern::ListPattern;


macro_rules! impl_pattern {
    ($name:ident, $e:expr) => {
        lazy_static! {
            pub static ref $name: ListPattern =
                ListPattern::from(&one_unit_parse($e, "<cfir>").unwrap()).unwrap();
        }
    };
}

impl_pattern!(MODULE, "('unnamed-module defines ...)");
impl_pattern!(NAMED_MODULE, "('module name defines ...)");

impl_pattern!(FUNCTION_DECL, "('function-decl name ret-type param-type)");
impl_pattern!(FUNCTION_DEF, "('function name ret-type param-type lines ...)");
impl_pattern!(PUBLIC_FUNCTION_DEF, "('pub-function name ret-type param-type lines ...)");

impl_pattern!(LABEL_DEF, "('label label-name)");

impl_pattern!(PARAMS_TYPE, "(param_pair ...)");
impl_pattern!(PARAM_PAIR_TYPE, "(type name ...)");

impl_pattern!(FUNCTION_TYPE, "('fun ret params)");

impl_pattern!(RECORD_TYPE, "('record lines ...)");
impl_pattern!(ALIGNED_RECORD_TYPE, "('aligned-record lines ...)");
impl_pattern!(RECORD_LINE, "(name type)");
impl_pattern!(ARRAY_TYPE, "('array type number)");

impl_pattern!(VECTOR_TYPE, "('vector simple-type number)");

impl_pattern!(TYPE_DEF, "('type-def name type)");
