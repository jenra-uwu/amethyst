use std::{fmt::Display, collections::HashMap};

#[derive(Debug, Clone)]
enum MapValue {
    Unknown,
    Pointer(usize),
    // Struct(Vec<Option<usize>>),
}

impl Display for MapValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapValue::Unknown => write!(f, "-"),
            MapValue::Pointer(p) => write!(f, "p_{p}"),
            // MapValue::Struct(vs) => {
            //     write!(f, "(")?;

            //     if let Some(v) = vs.first() {
            //         match v {
            //             Some(p) => write!(f, "p_{p}")?,
            //             None => write!(f, "_")?,
            //         }
            //     }

            //     for v in vs[1..].iter() {
            //         match v {
            //             Some(p) => write!(f, ", p_{p}")?,
            //             None => write!(f, ", _")?,
            //         }
            //     }

            //     write!(f, ")")
            // }
        }
    }
}

#[derive(Debug, Clone)]
enum HeapPred {
    Empty,
    Map(usize, MapValue),
    Star(Vec<HeapPred>),
}

impl Display for HeapPred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeapPred::Empty => write!(f, "emp"),
            HeapPred::Map(p, v) => write!(f, "p_{p} |-> {v}"),
            HeapPred::Star(s) => {
                if let Some(p) = s.first() {
                    write!(f, "({p}")?;
                }

                for p in s[1..].iter() {
                    write!(f, " * {p}")?;
                }

                write!(f, ")")
            }
        }
    }
}

impl HeapPred {
    fn add_predicate(&mut self, pred: HeapPred) {
        match (self, pred) {
            (_, HeapPred::Empty) => (),
            (s @ HeapPred::Empty, pred) => *s = pred,

            (HeapPred::Star(ps), HeapPred::Star(ps_add)) => ps.extend(ps_add),
            (HeapPred::Star(ps), pred) => ps.push(pred),
            (s, pred @ HeapPred::Star(_)) => {
                let mut temp = pred;
                std::mem::swap(s, &mut temp);
                let HeapPred::Star(ps) = s
                else {
                    unreachable!();
                };
                ps.push(temp);
            }

            (s, pred) => {
                let mut temp = HeapPred::Star(vec![pred]);
                std::mem::swap(s, &mut temp);
                let HeapPred::Star(ps) = s
                else {
                    unreachable!();
                };
                ps.insert(0, temp);
            }
        }
    }

    fn remove_map(&mut self, ptr: usize) -> bool {
        match self {
            HeapPred::Star(ps) => {
                let mut i = None;
                for (j, p) in ps.iter_mut().enumerate() {
                    match p {
                        &mut HeapPred::Map(p_, _) => {
                            if ptr == p_ {
                                i = Some(j);
                                break;
                            }
                        }

                        p => {
                            if p.remove_map(ptr) {
                                return true;
                            }
                        }
                    }
                }

                if let Some(i) = i {
                    ps.remove(i);
                    true
                } else {
                    false
                }
            }

            &mut HeapPred::Map(p, _) if p == ptr => {
                *self = HeapPred::Empty;
                true
            }

            _ => false,
        }
    }

    fn unifiable(&self, other: &HeapPred) -> bool {
        match (self, other) {
            (HeapPred::Empty, HeapPred::Empty) => true,

            // TODO
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pointer(usize);

#[derive(Debug, Clone)]
pub struct Store {
    var_map: HashMap<String, usize>,
    pred: HeapPred,
    next_ptr: usize,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            var_map: HashMap::default(),
            pred: HeapPred::Empty,
            next_ptr: 1,
        }
    }
}

impl Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, sigma |= {}", self.var_map, self.pred)
    }
}

impl Store {
    pub fn alloc(&mut self) -> Pointer {
        let ptr = self.next_ptr;
        self.pred.add_predicate(HeapPred::Map(ptr, MapValue::Unknown));
        self.next_ptr += 1;
        Pointer(ptr)
    }

    pub fn dealloc(&mut self, ptr: Pointer) -> bool {
        self.pred.remove_map(ptr.0)
    }

    pub fn set_var(&mut self, var: &str, p: Pointer) -> Option<Pointer> {
        self.var_map.insert(var.to_owned(), p.0).map(Pointer)
    }

    pub fn get_var(&self, var: &str) -> Option<Pointer> {
        self.var_map.get(var).cloned().map(Pointer)
    }

    pub fn remove_var(&mut self, var: &str) -> Option<Pointer> {
        self.var_map.remove(var).map(Pointer)
    }

    // TEMP: we prolly don't need this once we actually have types
    pub fn is_empty(&self) -> bool {
        self.pred.unifiable(&HeapPred::Empty)
    }
}
