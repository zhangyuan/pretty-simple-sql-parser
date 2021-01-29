use peg;

#[derive(Debug)]
pub struct SelectedColumn {
  name: String,
  alias: Option<String>
}

impl PartialEq for SelectedColumn {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.alias == other.alias
  }
}

#[derive(Debug)]
pub struct Table {
  name: String,
}

impl PartialEq for Table {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

#[derive(Debug)]
pub struct SelectFromTable {
  table: Option<Table>,
  columns: Vec<SelectedColumn>
}

impl PartialEq for SelectFromTable {
  fn eq(&self, other: &Self) -> bool {
    self.table == other.table && self.columns == other.columns
  }
}

#[derive(Debug)]
pub struct SelectToTable {
  columns: Vec<SelectedColumn>
}

peg::parser!{
  grammar list_parser() for str {
    rule number() -> u32
      = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub rule list() -> Vec<u32>
      = "[" l:number() ** "," "]" { l }
  }
}

peg::parser!{
  grammar sql_parser() for str {
    rule whitespace() = quiet!{[' ' | '\n' | '\t']+} / expected!("whitespace")

    rule column_name() -> String
      = t:$(['a'..='z' | 'A'..='Z' | '_']+ / ['*']) { t.parse().unwrap() }

    rule alias_name() -> String
      = t:$(['a'..='z' | 'A'..='Z' | '_']+ / ['*']) { t.parse().unwrap() }

    rule as_alias() -> String
      = whitespace()* "as" whitespace()* a:alias_name() { a.parse().unwrap() }

    rule selected_column() -> SelectedColumn
      = t:column_name() a:as_alias()? { SelectedColumn {
      name: t,
      alias: a
     }
    }

    rule selected_columns() -> Vec<SelectedColumn>
      = columns: selected_column() ** ("," whitespace()*) {
      columns
    }

    rule table_name() -> String
      = t:$(['a'..='z' | 'A'..='Z' | '_']+ ) { t.parse().unwrap() }

    rule from_table() -> Table
      = "FROM" whitespace() t:table_name() {
        Table {
          name: t
        }
      }

    pub rule sql() -> SelectFromTable
      = "SELECT" whitespace() s:selected_columns() whitespace()* t:from_table()? { SelectFromTable {
        table: t,
        columns: s
      }
    }
  }
}

fn main() {
}

#[cfg(test)]
mod tests {
  use super::*;

  fn build_table(name: &str) -> Table {
      Table {
        name: name.into()
      }
  }

  #[test]
  fn should_parse_list() {
    assert_eq!(list_parser::list("[1,1,2,3,5,8]"), Ok(vec![1, 1, 2, 3, 5, 8]));
  }

  #[test]
  fn should_parse_sql() {
    let x = sql_parser::sql("SELECT * FROM abc_");

    assert_eq!(x, Ok(SelectFromTable {
      table: Some(build_table("abc_")),
        columns: vec![
          SelectedColumn { name: String::from("*"),  alias: None }
        ]
      },
    ));
  }

  #[test]
  fn should_parse_sql2() {
    let x = sql_parser::sql("SELECT x FROM abc_");

    assert_eq!(x, Ok(SelectFromTable {
      table: Some(Table{
        name: String::from("abc_")
      }),
      columns: vec![SelectedColumn {
        name: String::from("x"),
        alias: None
      }]
    }));
  }

  #[test]
  fn should_parse_sql2_1() {
    let x = sql_parser::sql("SELECT x as y FROM abc_");

    assert_eq!(x, Ok(SelectFromTable {
      table: Some(build_table("abc_")),
      columns: vec![SelectedColumn {
        name: String::from("x"),
        alias: Some("y".into())
      }]
    }));
  }

  #[test]
  fn should_parse_sql3() {
    let x = sql_parser::sql("SELECT a,b,c FROM abc_");

    assert_eq!(x, Ok(SelectFromTable {
      table: Some(build_table("abc_")),
      columns: vec![
        SelectedColumn { name: String::from("a"), alias: None},
        SelectedColumn { name: String::from("b"), alias: None },
        SelectedColumn { name: String::from("c"), alias: None },
      ]
    }));
  }

  #[test]
  fn should_parse_sql3_1() {
    let x = sql_parser::sql("SELECT a, b FROM abc_");

    assert_eq!(x, Ok(SelectFromTable {
      table: Some(build_table("abc_")),
      columns: vec![
        SelectedColumn { name: String::from("a"), alias: None },
        SelectedColumn { name: String::from("b"), alias: None },
      ]
    }));
  }

  #[test]
  fn should_parse_sql4() {
    let x = sql_parser::sql("SELECT x  FROM abc_");
    assert_eq!(x, Ok(SelectFromTable {
      table: Some(build_table("abc_")),
      columns: vec![
        SelectedColumn { name: String::from("x"), alias: None }
      ]
    }));
  }

  #[test]
  fn should_parse_sql5() {
    let x = sql_parser::sql(r#"SELECT x,
    y  FROM abc_"#);
    assert_eq!(x, Ok(SelectFromTable {
      table: Some(build_table("abc_")),
      columns: vec![
        SelectedColumn { name: String::from("x"), alias: None },
        SelectedColumn { name: String::from("y"), alias: None }
      ]
    }));
  }

  #[test]
  fn should_parse_sql6() {
    let x = sql_parser::sql("SELECT x, \t y  FROM abc_");
    assert_eq!(x, Ok(SelectFromTable {
      table: Some(build_table("abc_")),
      columns: vec![
        SelectedColumn { name: String::from("x"), alias: None },
        SelectedColumn { name: String::from("y"), alias: None }
      ]
    }));
  }
}
