mod lexer;
mod parser;

fn main() {
    let input = "Animal behavior from an ethological perspective, with
emphasis on the mechanisms underlying a variety of behaviors. The material is
intended to complement that of ZOOL 371. Prerequisite or corequisite: ZOOL 241
or 242 or PHYSL 210, or 212 or 214. Offered in alternate years.";
    let requirements = parser::Parser::from(input).parse();
    dbg!(input);
    dbg!(requirements);
}
