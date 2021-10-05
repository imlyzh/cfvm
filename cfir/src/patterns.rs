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

impl_pattern!(
    FUNCTION_DECL,
    "('function-decl attrs name ret-type param-type)"
);
impl_pattern!(
    FUNCTION_DEF,
    "('function attrs name ret-type param-type lines ...)"
);

impl_pattern!(GLOBAL_CONST_DEF, "('const attrs global-name expr)");
impl_pattern!(
    GLOBAL_VARIABLE_DEF,
    "('var attrs global-name type expr ...)"
);

impl_pattern!(TYPE_DEF, "('type attrs name type)");

// datas

impl_pattern!(RECORD_DATA, "('record lines ...)");
impl_pattern!(RECORD_DATA_LINE, "(name expr ...)");
impl_pattern!(ARRAY_DATA, "('array exprs ...)");

impl_pattern!(VECTOR_DATA, "('vector exprs ...)");

impl_pattern!(BYTES_DATA, "('record lines ...)");

// attris

impl_pattern!(ATTRIS, "(exprs ...)");

// types

impl_pattern!(POINTER_TYPE, "('ptr type)");

impl_pattern!(PARAMS_TYPE, "(param-pair ...)");
impl_pattern!(PARAM_PAIR_TYPE, "(type name ...)");

impl_pattern!(FUNCTION_TYPE, "('fun ret params)");

impl_pattern!(RECORD_TYPE, "('record lines ...)");
impl_pattern!(ALIGNED_RECORD_TYPE, "('aligned-record lines ...)");
impl_pattern!(RECORD_LINE, "(type name ...)");
impl_pattern!(ARRAY_TYPE, "('array type number)");

impl_pattern!(VECTOR_TYPE, "('vector simple-type number)");

//

impl_pattern!(LABEL_DEF, "('label label-name)");

// terminator

impl_pattern!(RET, "('ret args ...)");
impl_pattern!(BRANCH, "('branch op cond then else)");
impl_pattern!(CONDS, "('cond pair)");
impl_pattern!(SWITCH, "('switch num pair)");
impl_pattern!(CALL, "('call name args ...)");
impl_pattern!(UNRECHABLE, "('unrechable)");

impl_pattern!(STORE_INST_DEF, "('store local-name name)");

impl_pattern!(V_STORE_INST_DEF, "('volatile-store local-name name)");

impl_pattern!(BIND_INST_DEF, "('let local-name inst)");

// operator

impl_pattern!(ALLOCA_INST, "('alloc type)");
impl_pattern!(GET_PTR_INST, "('get-ptr value)");
impl_pattern!(LOAD_INST, "('load type value)");
impl_pattern!(CAST_INST, "('cast type value)");
impl_pattern!(ADD_INST, "('add value0 value1)");
impl_pattern!(F_ADD_INST, "('fadd value0 value1)");
impl_pattern!(SUB_INST, "('sub value0 value1)");
impl_pattern!(F_SUB_INST, "('fsub value0 value1)");
impl_pattern!(MUL_INST, "('mul value0 value1)");
impl_pattern!(F_MUL_INST, "('fmul value0 value1)");
impl_pattern!(U_DIV_INST, "('udiv value0 value1)");
impl_pattern!(S_DIV_INST, "('sdiv value0 value1)");
impl_pattern!(U_REM_INST, "('urem value0 value1)");
impl_pattern!(S_REM_INST, "('srem value0 value1)");
impl_pattern!(F_REM_INST, "('frem value0 value1)");
impl_pattern!(SHL_INST, "('shl value0 value1)");
impl_pattern!(L_SHR_INST, "('lshr value0 value1)");
impl_pattern!(A_SHR_INST, "('ashr value0 value1)");
impl_pattern!(AND_INST, "('and value0 value1)");
impl_pattern!(OR_INST, "('or value0 value1)");
impl_pattern!(XOR_INST, "('xor value0 value1)");
impl_pattern!(GET_VALUE_INST, "('get-value value index-list)");
impl_pattern!(GET_ITEM_INST, "('get-item value, index-value)");
impl_pattern!(SET_VALUE_INST, "('set-value value index-list value1)");
impl_pattern!(SET_ITEM_INST, "('set-item value index-value value1)");
impl_pattern!(TRUNC_INST, "('trunc value int-type)");
impl_pattern!(ZEXT_INST, "('zext value int-type)");
impl_pattern!(SEXT_INST, "('sext value int-type)");
impl_pattern!(FTRUNC_INST, "('ftrunc value float-type)");
impl_pattern!(F_EXT_INST, "('fext value float-type)");
impl_pattern!(I_CMP_INST, "('icmp icmp-op value0 value1)");
impl_pattern!(F_CMP_INST, "('fcmp fcmp-op value0 value1)");
impl_pattern!(PHI_INST, "('phi phi-pairs ...)");
impl_pattern!(PHI_PAIR, "['value label]");
impl_pattern!(CALL_INST, "('call fun-value values ...)");
