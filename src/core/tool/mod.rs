use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::prompt::CoreMessage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolResultContent {}

pub trait ToolParameters {
    type Output;
}

pub struct Schema<T> {
    pub _type: std::marker::PhantomData<T>,
}

impl<T> ToolParameters for Schema<T> {
    type Output = T;
}

pub struct ToolExecutionOptions {
    pub tool_call_id: String,
    pub messages: Vec<CoreMessage>,
}

pub type ToolExecuteFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub struct Tool<P: ToolParameters, R> {
    pub parameters: P,
    pub description: Option<String>,
    pub execute: Option<Box<dyn Fn(P::Output, ToolExecutionOptions) -> ToolExecuteFuture<R>>>,
    pub tool_type: ToolType,
}

pub enum ToolType {
    Function,
    ProviderDefined {
        id: String,
        args: HashMap<String, serde_json::Value>,
    },
}

pub fn tool<P, R>(parameters: P, description: Option<String>) -> ToolBuilder<P, R>
where
    P: ToolParameters,
{
    ToolBuilder {
        parameters,
        description,
        experimental_to_tool_result_content: None,
        execute: None,
        tool_type: ToolType::Function,
    }
}

pub struct ToolBuilder<P: ToolParameters, R> {
    parameters: P,
    description: Option<String>,
    experimental_to_tool_result_content: Option<Box<dyn Fn(R) -> ToolResultContent>>,
    execute: Option<Box<dyn Fn(P::Output, ToolExecutionOptions) -> ToolExecuteFuture<R>>>,
    tool_type: ToolType,
}

impl<P: ToolParameters, R> ToolBuilder<P, R> {
    pub fn with_execute<F, Fut>(mut self, execute_fn: F) -> Self
    where
        F: Fn(P::Output, ToolExecutionOptions) -> Fut + 'static,
        Fut: Future<Output = R> + Send + 'static,
    {
        self.execute = Some(Box::new(move |args, opts| Box::pin(execute_fn(args, opts))));
        self
    }

    pub fn with_tool_result_content<F>(mut self, converter: F) -> Self
    where
        F: Fn(R) -> ToolResultContent + 'static,
    {
        self.experimental_to_tool_result_content = Some(Box::new(converter));
        self
    }

    pub fn provider_defined(
        mut self,
        id: String,
        args: HashMap<String, serde_json::Value>,
    ) -> Self {
        self.tool_type = ToolType::ProviderDefined { id, args };
        self
    }

    pub fn build(self) -> Tool<P, R> {
        Tool {
            parameters: self.parameters,
            description: self.description,
            execute: self.execute,
            tool_type: self.tool_type,
        }
    }
}
