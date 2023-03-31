use amethyst::lexer::Lexer;
use amethyst::{parse, typecheck};

fn main() {
    // let mut lexer = Lexer::new("forall (a: type) (b: type) let apply (f: a -> b) (x: a) -> b = f x\nforall (a: type) let id (x: a) -> a = x\nlet a = apply id 2\nlet b = apply id true");
    // let mut lexer = Lexer::new("forall (a: type) (b: type) type Pair = Pair a b\nforall (a: type) type Option = Some a | None\nlet f x = match x with | Some x to x + 1 | None to 0 end");
    // let mut lexer = Lexer::new("let apply f x = f x\nlet id x = x\nlet a = apply id 2\nlet b = apply id true");
    // let mut lexer = Lexer::new("type Nat = S Nat | Z\nlet f x = match x with | Z to 0 | S k to 1 + f k end");
    let mut lexer = Lexer::new(
        "
        let not x = if x then false else true

        forall (a: type)
        type Option
            = Some a
            | None

        type NoEq = Uwu | Owo

        class Eq e =
            let eq: e -> e -> bool
        end

        instance Eq i32 =
            let eq a b = a == b
        end

        instance Eq bool =
            let eq a b = if a then b else not b
        end

        let ne a b = not (eq a b)

        let a = eq 2 3
        let b = eq true false
    ",
    ); /*
        instance Eq i32 =
            let eq a b = a == b
        end

        instance Eq bool =
            let eq a b =
                if a then
                    b
                else if b then
                    false
                else true
        end

        forall (e: type) where Eq e
        instance Eq (Option e) =
            let eq a b =
                match Pair a b with
                | Pair (Some a) (Some b) to eq a b
                | Pair None None         to true
                | _                      to false
                end
        end
    ",
    );
    // */

    let mut asts = parse::parse(&mut lexer).expect("should work");
    for ast in asts.iter() {
        println!("{}", ast);
    }

    println!();

    typecheck::typecheck(&mut asts).expect("should work");

    for ast in asts {
        println!("{}", ast);
    }
}
