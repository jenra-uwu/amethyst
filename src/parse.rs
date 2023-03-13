use std::fmt::Display;

use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum BaseType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Named(String, Vec<Type>, Vec<Ast>),
}

impl Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseType::Bool => write!(f, "bool"),
            BaseType::I8 => write!(f, "i8"),
            BaseType::I16 => write!(f, "i16"),
            BaseType::I32 => write!(f, "i32"),
            BaseType::I64 => write!(f, "i64"),
            BaseType::U8 => write!(f, "u8"),
            BaseType::U16 => write!(f, "u16"),
            BaseType::U32 => write!(f, "u32"),
            BaseType::U64 => write!(f, "u64"),
            BaseType::F32 => write!(f, "f32"),
            BaseType::F64 => write!(f, "f64"),
            BaseType::Named(name, tparams, vparams) => {
                write!(f, "{}", name)?;
                for tparam in tparams {
                    write!(f, " {}", tparam)?;
                }
                for vparam in vparams {
                    write!(f, " {}", vparam)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Unknown,
    Base(BaseType),
    Func(Box<Type>, Box<Type>),
    Refined(BaseType, Box<Ast>),
    Generic(String),
    TypeVar(usize),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unknown => write!(f, "<unknown>"),
            Type::Base(b) => write!(f, "{}", b),
            Type::Func(a, r) => write!(f, "({} -> {})", a, r),
            Type::Refined(t, v) => write!(f, "{{ {} : {} }}", t, v),
            Type::Generic(g) => write!(f, "{}", g),
            Type::TypeVar(v) => write!(f, "${}", v),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    LogicalAnd,
    LogicalOr,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::Le => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::Ge => write!(f, ">="),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::Ne => write!(f, "!="),
            BinaryOp::LogicalAnd => write!(f, "&&"),
            BinaryOp::LogicalOr => write!(f, "||"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ast {
    Integer(u128),
    Bool(bool),
    Symbol(String),

    Binary {
        op: BinaryOp,
        left: Box<Ast>,
        right: Box<Ast>,
    },

    FuncCall {
        func: Box<Ast>,
        args: Vec<Ast>,
    },

    Let {
        mutable: bool,
        symbol: String,
        args: Vec<(String, Type)>,
        ret_type: Type,
        value: Box<Ast>,
        context: Box<Ast>,
    },

    TopLet {
        symbol: String,
        generics: Vec<String>,
        dep_values: Vec<(String, Type)>,
        args: Vec<(String, Type)>,
        ret_type: Type,
        value: Box<Ast>,
    },

    If {
        cond: Box<Ast>,
        then: Box<Ast>,
        elsy: Box<Ast>,
    },

    DatatypeDefinition {
        name: String,
        generics: Vec<String>,
        dep_values: Vec<(String, Type)>,
        variants: Vec<(String, Vec<(Option<String>, Type)>)>,
    },
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ast::Integer(n) => write!(f, "{}", n),
            Ast::Bool(b) => write!(f, "{}", b),
            Ast::Symbol(s) => write!(f, "{}", s),
            Ast::Binary { op, left, right } => write!(f, "({} {} {})", left, op, right),

            Ast::FuncCall { func, args } => {
                write!(f, "{}", func)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                Ok(())
            }

            Ast::Let { mutable, symbol, args, ret_type, value, context } => {
                write!(f, "let ")?;
                if *mutable {
                    write!(f, "mut ")?;
                }
                write!(f, "{}", symbol)?;
                for (arg, type_) in args {
                    write!(f, " ({}: {})", arg, type_)?;
                }

                if args.is_empty() {
                    write!(f, ": {}", ret_type)?;
                } else {
                    write!(f, " -> {}", ret_type)?;
                }

                write!(f, " = {} in {}", value, context)
            }

            Ast::TopLet { symbol, generics, dep_values, args, ret_type, value } => {
                if !generics.is_empty() || !dep_values.is_empty() {
                    write!(f, "forall")?;
                    for generic in generics {
                        write!(f, " ({}: type)", generic)?;
                    }
                    for (value, type_) in dep_values {
                        write!(f, " ({}: {})", value, type_)?;
                    }
                    writeln!(f)?;
                }

                write!(f, "let {}", symbol)?;
                for (arg, type_) in args {
                    write!(f, " ({}: {})", arg, type_)?;
                }

                if args.is_empty() {
                    write!(f, ": {}", ret_type)?;
                } else {
                    write!(f, " -> {}", ret_type)?;
                }

                write!(f, " = {}", value)
            }

            Ast::If { cond, then, elsy } => write!(f, "(if {} then {} else {})", cond, then, elsy),

            Ast::DatatypeDefinition { name, generics, dep_values, variants } => {
                if !generics.is_empty() || !dep_values.is_empty() {
                    write!(f, "forall")?;
                    for generic in generics {
                        write!(f, " ({}: type)", generic)?;
                    }
                    for (value, type_) in dep_values {
                        write!(f, " ({}: {})", value, type_)?;
                    }
                    writeln!(f)?;
                }

                writeln!(f, "type {}", name)?;
                let mut first = true;
                for (constructor, fields) in variants {
                    if first {
                        write!(f, "    = {}", constructor)?;
                        first = false;
                    } else {
                        write!(f, "    | {}", constructor)?;
                    }

                    for (name, type_) in fields {
                        if let Some(name) = name {
                            write!(f, " ({}: {})", name, type_)?;
                        } else {
                            write!(f, " {}", type_)?;
                        }
                    }
                    writeln!(f)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    NotStarted,
    InvalidValue,
    UnclosedBracket,
    InvalidInfix,
    InvalidLet,
    InvalidType,
    InvalidArg,
    InvalidIf,
    InvalidTypeDef,
}

macro_rules! try_token {
    ($lexer: expr, $pat: pat) => {{
        let token = $lexer.peek();
        if matches!(token, $pat) {
            $lexer.lex()
        } else {
            None
        }
    }}
}

macro_rules! try_clean {
    ($parse: expr, $lexer: expr, $state: ident, $error: expr) => {
        match $parse {
            Ok(v) => v,
            e @ Err(_) => {
                $lexer.pop($state);
                return e;
            }
        }
    }
}

fn parse_base_type(lexer: &mut Lexer<'_>, generics: &[String]) -> Result<Type, ParseError> {
    let state = lexer.push();
    match lexer.lex() {
        Some(Token::Symbol("bool")) => Ok(Type::Base(BaseType::Bool)),
        Some(Token::Symbol("i8")) => Ok(Type::Base(BaseType::I8)),
        Some(Token::Symbol("i16")) => Ok(Type::Base(BaseType::I16)),
        Some(Token::Symbol("i32")) => Ok(Type::Base(BaseType::I32)),
        Some(Token::Symbol("i64")) => Ok(Type::Base(BaseType::I64)),
        Some(Token::Symbol("u8")) => Ok(Type::Base(BaseType::U8)),
        Some(Token::Symbol("u16")) => Ok(Type::Base(BaseType::U16)),
        Some(Token::Symbol("u32")) => Ok(Type::Base(BaseType::U32)),
        Some(Token::Symbol("u64")) => Ok(Type::Base(BaseType::U64)),
        Some(Token::Symbol("f32")) => Ok(Type::Base(BaseType::F32)),
        Some(Token::Symbol("f64")) => Ok(Type::Base(BaseType::F64)),
        Some(Token::Symbol(s)) if generics.iter().any(|v| v.as_str() == s) => Ok(Type::Generic(s.to_string())),

        Some(Token::LParen) => {
            let type_ = parse_type(lexer, generics)?;
            if let Some(Token::RParen) = lexer.lex() {
                Ok(type_)
            } else {
                Err(ParseError::UnclosedBracket)
            }
        }

        Some(Token::LBrace) => {
            lexer.pop(state);
            parse_refinement_type(lexer, generics)
        }

        _ => {
            lexer.pop(state);
            Err(ParseError::NotStarted)
        }
    }
}

fn parse_child_type(lexer: &mut Lexer<'_>, generics: &[String]) -> Result<Type, ParseError> {
    match parse_base_type(lexer, generics) {
        v @ Ok(_) => return v,
        Err(ParseError::NotStarted) => (),
        e @ Err(_) => return e,
    }

    if let Some(Token::Symbol(s)) = try_token!(lexer, Some(Token::Symbol(_))) {
        Ok(Type::Base(BaseType::Named(s.to_string(), Vec::new(), Vec::new())))
    } else {
        Err(ParseError::InvalidType)
    }
}

fn parse_parametarised_type(lexer: &mut Lexer<'_>, generics: &[String]) -> Result<Type, ParseError> {
    match parse_base_type(lexer, generics) {
        v @ Ok(_) => return v,
        Err(ParseError::NotStarted) => (),
        e @ Err(_) => return e,
    }

    match lexer.lex() {
        Some(Token::Symbol(s)) => {
            let mut parameters = Vec::new();
            let mut state = lexer.push();
            while let Ok(param) = parse_child_type(lexer, generics) {
                parameters.push(param);
                state = lexer.push();
            }
            lexer.pop(state);

            Ok(Type::Base(BaseType::Named(s.to_string(), parameters, Vec::new())))
        }

        _ => Err(ParseError::InvalidType)
    }
}

fn parse_refinement_type(lexer: &mut Lexer<'_>, generics: &[String]) -> Result<Type, ParseError> {
    if try_token!(lexer, Some(Token::LBrace)).is_some() {
        let base = match parse_parametarised_type(lexer, generics)? {
            Type::Base(v) => v,
            _ => return Err(ParseError::InvalidType),
        };

        if try_token!(lexer, Some(Token::Colon)).is_none() {
            return Err(ParseError::UnclosedBracket);
        }

        let constraint = parse_expr(lexer)?;

        if try_token!(lexer, Some(Token::RBrace)).is_none() {
            return Err(ParseError::UnclosedBracket);
        }

        Ok(Type::Refined(base, Box::new(constraint)))
    } else {
        Err(ParseError::NotStarted)
    }
}

fn parse_func_type(lexer: &mut Lexer<'_>, generics: &[String]) -> Result<Type, ParseError> {
    let mut top = parse_parametarised_type(lexer, generics)?;
    let mut nodes = Vec::new();
    while try_token!(lexer, Some(Token::Arrow)).is_some() {
        nodes.push(top);
        top = parse_parametarised_type(lexer, generics)?;
    }
    for node in nodes.into_iter().rev() {
        top = Type::Func(Box::new(node), Box::new(top));
    }

    Ok(top)
}

fn parse_type(lexer: &mut Lexer<'_>, generics: &[String]) -> Result<Type, ParseError> {
    parse_func_type(lexer, generics)
}

fn parse_value(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    match lexer.lex() {
        Some(Token::Integer(n)) => Ok(Ast::Integer(n)),
        Some(Token::Bool(b)) => Ok(Ast::Bool(b)),
        Some(Token::Symbol(s)) => Ok(Ast::Symbol(s.to_string())),
        Some(Token::LParen) => {
            let value = parse_expr(lexer)?;
            if let Some(Token::RParen) = lexer.lex() {
                Ok(value)
            } else {
                Err(ParseError::UnclosedBracket)
            }
        }

        _ => Err(ParseError::InvalidValue),
    }
}

fn parse_func_call(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    let func = parse_value(lexer)?;
    let mut args = Vec::new();
    let mut state = lexer.push();
    while let Ok(arg) = parse_value(lexer) {
        args.push(arg);
        state = lexer.push();
    }
    lexer.pop(state);

    if args.is_empty() {
        Ok(func)
    } else {
        Ok(Ast::FuncCall {
            func: Box::new(func),
            args,
        })
    }
}

fn parse_muldiv(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    let state = lexer.push();
    let mut top = parse_func_call(lexer)?;
    while let Some(token) = try_token!(lexer, Some(Token::Astrisk | Token::Slash | Token::Percent)) {
        let right = try_clean!(parse_func_call(lexer), lexer, state, ParseError::InvalidInfix);
        top = Ast::Binary {
            op: match token {
                Token::Astrisk => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            },
            left: Box::new(top),
            right: Box::new(right),
        };
    }

    Ok(top)
}

fn parse_addsub(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    let state = lexer.push();
    let mut top = parse_muldiv(lexer)?;
    while let Some(token) = try_token!(lexer, Some(Token::Plus | Token::Minus)) {
        let right = try_clean!(parse_muldiv(lexer), lexer, state, ParseError::InvalidInfix);
        top = Ast::Binary {
            op: match token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            },
            left: Box::new(top),
            right: Box::new(right),
        };
    }

    Ok(top)
}

fn parse_compare(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    let state = lexer.push();
    let mut top = parse_addsub(lexer)?;
    while let Some(token) = try_token!(lexer, Some(Token::Lt | Token::Le | Token::Gt | Token::Ge | Token::Eq | Token::Ne)) {
        let right = try_clean!(parse_addsub(lexer), lexer, state, ParseError::InvalidInfix);
        top = Ast::Binary {
            op: match token {
                Token::Lt => BinaryOp::Lt,
                Token::Le => BinaryOp::Le,
                Token::Gt => BinaryOp::Gt,
                Token::Ge => BinaryOp::Ge,
                Token::Eq => BinaryOp::Eq,
                Token::Ne => BinaryOp::Ne,
                _ => unreachable!(),
            },
            left: Box::new(top),
            right: Box::new(right),
        };
    }

    Ok(top)
}

fn parse_and(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    let state = lexer.push();
    let mut top = parse_compare(lexer)?;
    while try_token!(lexer, Some(Token::LogicalAnd)).is_some() {
        let right = try_clean!(parse_compare(lexer), lexer, state, ParseError::InvalidInfix);
        top = Ast::Binary {
            op: BinaryOp::LogicalAnd,
            left: Box::new(top),
            right: Box::new(right),
        };
    }

    Ok(top)
}

fn parse_or(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    let state = lexer.push();
    let mut top = parse_and(lexer)?;
    while try_token!(lexer, Some(Token::LogicalOr)).is_some() {
        let right = try_clean!(parse_and(lexer), lexer, state, ParseError::InvalidInfix);
        top = Ast::Binary {
            op: BinaryOp::LogicalAnd,
            left: Box::new(top),
            right: Box::new(right),
        };
    }

    Ok(top)
}

fn parse_argument(lexer: &mut Lexer<'_>, allow_type: bool, generics: &[String]) -> Result<(String, Option<Type>), ParseError> {
    if let Some(Token::Symbol(s)) = try_token!(lexer, Some(Token::Symbol(_))) {
        Ok((s.to_string(), None))
    } else if try_token!(lexer, Some(Token::LParen)).is_some() {
        if let Some(Token::Symbol(arg)) = try_token!(lexer, Some(Token::Symbol(_))) {
            if try_token!(lexer, Some(Token::Colon)).is_none() {
                return Err(ParseError::InvalidArg);
            }

            let type_ = if allow_type && try_token!(lexer, Some(Token::Type)).is_some() {
                None
            } else {
                Some(parse_type(lexer, generics)?)
            };
            if try_token!(lexer, Some(Token::RParen)).is_none() {
                return Err(ParseError::InvalidArg);
            }

            Ok((arg.to_string(), type_))
        } else {
            Err(ParseError::InvalidArg)
        }
    } else {
        Err(ParseError::NotStarted)
    }
}

fn parse_if(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    if try_token!(lexer, Some(Token::If)).is_some() {
        let cond = parse_or(lexer)?;
        if try_token!(lexer, Some(Token::Then)).is_none() {
            return Err(ParseError::InvalidIf);
        }

        let then = parse_expr(lexer)?;
        if try_token!(lexer, Some(Token::Else)).is_none() {
            return Err(ParseError::InvalidIf);
        }

        let elsy = parse_expr(lexer)?;

        Ok(Ast::If {
            cond: Box::new(cond),
            then: Box::new(then),
            elsy: Box::new(elsy),
        })
    } else {
        Err(ParseError::NotStarted)
    }
}

fn parse_let(lexer: &mut Lexer<'_>, top_level: bool, generics: &[String]) -> Result<Ast, ParseError> {
    if let Some(Token::Let) = lexer.peek() {
        lexer.lex();
        let mutable = matches!(lexer.peek(), Some(Token::Mut));
        if mutable {
            lexer.lex();
        }

        let symbol = match try_token!(lexer, Some(Token::Symbol(_))) {
            Some(Token::Symbol(v)) => v.to_string(),
            _ => return Err(ParseError::InvalidLet),
        };

        let mut args = Vec::new();

        // TODO: args annotated with types
        loop {
            match parse_argument(lexer, false, generics) {
                Ok((arg, Some(t))) => args.push((arg, t)),
                Ok((arg, None)) => args.push((arg, Type::Unknown)),
                Err(ParseError::NotStarted) => break,
                Err(e) => return Err(e),
            }
        }

        let ret_type = if if args.is_empty() { try_token!(lexer, Some(Token::Colon)) } else { try_token!(lexer, Some(Token::Arrow)) }.is_some() {
            parse_type(lexer, generics)?
        } else {
            Type::Unknown
        };

        if try_token!(lexer, Some(Token::Equals)).is_none() {
            return Err(ParseError::InvalidLet);
        }

        let value = parse_expr(lexer)?;
        if try_token!(lexer, Some(Token::In)).is_none() {
            if top_level && !mutable {
                return Ok(Ast::TopLet {
                    symbol,
                    generics: generics.to_vec(),
                    dep_values: Vec::new(), // TODO
                    args,
                    ret_type,
                    value: Box::new(value),
                });
            } else {
                return Err(ParseError::InvalidLet);
            }
        } else if top_level {
            return Err(ParseError::InvalidLet);
        }

        let context = parse_expr(lexer)?;
        Ok(Ast::Let {
            mutable,
            symbol,
            args,
            ret_type,
            value: Box::new(value),
            context: Box::new(context),
        })
    } else {
        Err(ParseError::NotStarted)
    }
}

fn parse_type_def_variant(lexer: &mut Lexer<'_>, generics: &[String]) -> Result<(String, Vec<(Option<String>, Type)>), ParseError> {
    let name = match try_token!(lexer, Some(Token::Symbol(_))) {
        Some(Token::Symbol(v)) => v.to_string(),
        _ => return Err(ParseError::InvalidTypeDef),
    };

    let mut fields = Vec::new();
    loop {
        let t = match parse_base_type(lexer, generics) {
            Ok(v) => v,
            Err(ParseError::NotStarted) => break,
            Err(e) => return Err(e),
        };

        fields.push((None, t));
    }

    Ok((name, fields))
}

fn parse_type_def(lexer: &mut Lexer<'_>, generics: &[String]) -> Result<Ast, ParseError> {
    if try_token!(lexer, Some(Token::Type)).is_some() {
        let name = match try_token!(lexer, Some(Token::Symbol(_))) {
            Some(Token::Symbol(v)) => v.to_string(),
            _ => return Err(ParseError::InvalidTypeDef),
        };

        if try_token!(lexer, Some(Token::Equals)).is_none() {
            return Err(ParseError::InvalidLet);
        }

        let mut variants = vec![parse_type_def_variant(lexer, generics)?];
        while try_token!(lexer, Some(Token::Pipe)).is_some() {
            variants.push(parse_type_def_variant(lexer, generics)?);
        }

        Ok(Ast::DatatypeDefinition {
            name,
            generics: generics.to_vec(),
            dep_values: Vec::new(),
            variants,
        })
    } else {
        Err(ParseError::NotStarted)
    }
}

// TODO: dependent values
fn parse_forall(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    if try_token!(lexer, Some(Token::Forall)).is_some() {
        let mut generics = Vec::new();
        loop {
            match parse_argument(lexer, true, &[]) {
                Ok((generic, None)) => generics.push(generic.to_string()),
                Ok(_) => todo!(),
                Err(ParseError::NotStarted) => break,
                Err(e) => return Err(e),
            }
        }

        match parse_let(lexer, true, &generics) {
            v @ Ok(_) => return v,
            Err(ParseError::NotStarted) => (),
            e => return e,
        }

        parse_type_def(lexer, &generics)
    } else {
        Err(ParseError::NotStarted)
    }
}

fn parse_expr(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    match parse_let(lexer, false, &[]) {
        v @ Ok(_) => return v,
        Err(ParseError::NotStarted) => (),
        e => return e,
    }
    match parse_if(lexer) {
        v @ Ok(_) => return v,
        Err(ParseError::NotStarted) => (),
        e => return e,
    }
    parse_or(lexer)
}

fn parse_top(lexer: &mut Lexer<'_>) -> Result<Ast, ParseError> {
    match parse_forall(lexer) {
        v @ Ok(_) => return v,
        Err(ParseError::NotStarted) => (),
        e @ Err(_) => return e,
    }

    match parse_type_def(lexer, &[]) {
        v @ Ok(_) => return v,
        Err(ParseError::NotStarted) => (),
        e @ Err(_) => return e,
    }

    parse_let(lexer, true, &[])
}

pub fn parse(lexer: &mut Lexer<'_>) -> Result<Vec<Ast>, ParseError> {
    let mut asts = Vec::new();
    while lexer.peek().is_some() {
        asts.push(parse_top(lexer)?);
    }
    Ok(asts)
}
