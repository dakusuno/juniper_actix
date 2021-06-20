use juniper::{FieldResult, RootNode,EmptySubscription,graphql_object};
use diesel::pg::PgConnection;
use super::context::GraphQLContext;
use super::data::Todos;
use super::models::{CreateTodoInput, Todo};


pub struct Query;

#[graphql_object(Context=GraphQLContext)]
impl Query{
    #[graphql(name="allTodos")]
    pub fn all_todos(context:&GraphQLContext)->FieldResult<Vec<Todo>>{
        let conn:&PgConnection = &context.pool.get().unwrap();
        Todos::all_todos(conn)
    }
    #[graphql(name="doneTodos")]
    pub fn done_todos(context:&GraphQLContext)->FieldResult<Vec<Todo>>{
        let conn:&PgConnection=&context.pool.get().unwrap();
        Todos::done_todos(conn)
    }
    #[graphql(name="notDoneTodos")]
    pub fn not_done_todos(context:&GraphQLContext)->FieldResult<Vec<Todo>>{
        let conn:&PgConnection=&context.pool.get().unwrap();
        Todos::not_done_todos(conn)
    }
    #[graphql(name="getTodoById")]
    pub fn get_todo_by_id(context:&GraphQLContext,id: i32) -> FieldResult<Option<Todo>>{
        let conn : &PgConnection= &context.pool.get().unwrap();
        Todos::get_todo_by_id(conn,id)
    }
}

pub struct Mutation;
#[graphql_object(Context=GraphQLContext)]
impl Mutation{
    #[graphql(name="createTodo")]
    pub fn create_todo(
        context:&GraphQLContext,
        input:CreateTodoInput
    )->FieldResult<Todo>{
        let conn:&PgConnection = &context.pool.get().unwrap();
        Todos::create_todo(conn,input)
    }
    #[graphql(name = "markTodoAsDone")]
    pub fn mark_todo_as_done(context: &GraphQLContext, id: i32) -> FieldResult<Todo> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::mark_todo_as_done(conn, id)
    }

    #[graphql(name = "markTodoAsNotDone")]
    pub fn mark_todo_as_not_done(
        context: &GraphQLContext,
        id: i32,
    ) -> FieldResult<Todo> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::mark_todo_as_not_done(conn, id)
    }
}
pub type Schema = RootNode<'static,Query,Mutation,EmptySubscription<GraphQLContext>>;
pub fn create_schema() -> Schema{
    Schema::new(Query, Mutation,EmptySubscription::new())
}