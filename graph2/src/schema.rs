use juniper::{EmptyMutation, RootNode};

struct Monkey {
    id: i32,
    name: String,
}

#[juniper::object(description = "A monkey")]
impl Monkey {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn members() -> Vec<Monkey> {
        vec![
            Monkey {
                id: 1,
                name: "Jerry".to_owned(),
            },
            Monkey {
                id: 2,
                name: "Dumdum".to_owned(),
            },
        ]
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
