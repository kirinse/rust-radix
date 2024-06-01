use std::{iter::Map, marker::PhantomData};

pub fn create_context<ContextValueType>(
    root_component_name: &str,
    default_context: Option<ContextValueType>,
) {
    // TODO
}

pub struct CreateScope {}

pub struct ContextScope<ContextValueType> {
    default_contexts: Vec<ContextValueType>,
}

impl<ContextValueType> ContextScope<ContextValueType> {
    pub fn new() -> Self {
        ContextScope {
            default_contexts: vec![],
        }
    }

    pub fn create_context(
        &self,
        root_component_name: &str,
        default_context: Option<ContextValueType>,
    ) -> Context<ContextValueType> {
        Context::new()
    }

    pub fn create_scope(&self) {}
}

impl<ContextValueType> Default for ContextScope<ContextValueType> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Context<ContextValueType> {
    context_value_type: PhantomData<ContextValueType>,
}

impl<ContextValueType> Context<ContextValueType> {
    pub fn new() -> Self {
        Context {
            context_value_type: PhantomData,
        }
    }

    pub fn provider(&self) {}

    pub fn use_context(&self, consumer_name: &str, scope: Option<Scope<ContextValueType>>) {}
}

impl<ContextValueType> Default for Context<ContextValueType> {
    fn default() -> Self {
        Self::new()
    }
}

pub type Scope<C> = Map<String, C>;

pub fn create_context_scope<ContextValueType>(
    scope_name: &str,
    create_context_scope_deps: Option<Vec<CreateScope>>,
) -> ContextScope<ContextValueType> {
    ContextScope::new()
}

fn compose_context_scopes(scopes: Vec<CreateScope>) -> CreateScope {
    CreateScope {}
}
