use crate::results::CAPID_GRAPHDB;
use crate::FromTable;
use crate::{
    generated::{DeleteGraphArgs, QueryGraphArgs, ResultSet},
    Result, OP_DELETE, OP_QUERY,
};
use actor_core::{deserialize, serialize};
use wascc_actor::untyped::{self, UntypedHostBinding};

#[doc(hidden)]
pub struct GraphHostBindingBuilder {
    binding: String,
}

impl GraphHostBindingBuilder {
    pub fn graph(&self, graph: &str) -> GraphHostBinding {
        GraphHostBinding {
            hostbinding: untyped::host(&self.binding),
            graph_name: graph.to_string(),
        }
    }
}

/// Represents an abstraction around a graph database host binding
pub struct GraphHostBinding {
    hostbinding: UntypedHostBinding,
    graph_name: String,
}

/// Requests a named host binding. Named bindings are used when the potential
/// exists to bind to more than one of the same capability in a single host
pub fn host(binding: &str) -> GraphHostBindingBuilder {
    GraphHostBindingBuilder {
        binding: binding.to_string(),
    }
}

/// Requests the default named host binding. Unless you know you're going to
/// have multiple graph providers for the same actor, you should use the
/// default binding
pub fn default() -> GraphHostBindingBuilder {
    GraphHostBindingBuilder {
        binding: "default".to_string(),
    }
}

impl GraphHostBinding {
    /// Executes a query against the host graph. For this provider, we assume the query is a Cypher query
    /// but it could be Gremlin or GraphQL, etc, depending on the capability provider satisfying `wasmcloud:graphdb`. This
    /// can be used to perform mutations if you also return data from the mutation query
    pub fn query<T: FromTable>(&self, query: &str) -> Result<T> {
        self.query_with_statistics(query).map(|(value, _)| value)
    }

    /// The same as [`query`](#method.query), but returns statistics from the query like execution time and nodes/relations affected, etc.
    pub fn query_with_statistics<T: FromTable>(&self, query: &str) -> Result<(T, Vec<String>)> {
        let result_set = self.get_result_set(query).map_err(|e| format!("{}", e))?;
        let value = T::from_table(&result_set).map_err(|e| format!("{}", e))?;
        Ok((value, result_set.statistics))
    }

    /// Executes the given query without returning any values
    ///
    /// If you want to mutate the graph and retrieve values using one query, use [`query`](#method.query) instead.
    pub fn mutate(&mut self, query: &str) -> Result<()> {
        self.mutate_with_statistics(query).map(|_| ())
    }

    /// Same as [`mutate`](#method.mutate), but returns statistics about the query.
    pub fn mutate_with_statistics(&mut self, query: &str) -> Result<Vec<String>> {
        let result_set = self.get_result_set(query).map_err(|e| format!("{}", e))?;
        Ok(result_set.statistics)
    }

    /// Deletes the entire graph from the database.
    ///
    /// This is a potentially very destructive function. Use with care.    
    pub fn delete(self) -> Result<()> {
        let delreq = DeleteGraphArgs {
            graph_name: self.graph_name.to_string(),
        };
        self.hostbinding
            .call(CAPID_GRAPHDB, OP_DELETE, serialize(&delreq)?)?;
        Ok(())
    }

    /// Returns the name of the graph
    pub fn name(&self) -> &str {
        &self.graph_name
    }

    fn get_result_set(&self, query: &str) -> Result<ResultSet> {
        let query = QueryGraphArgs {
            graph_name: self.graph_name.to_string(),
            query: query.to_string(),
        };
        let res = self
            .hostbinding
            .call(CAPID_GRAPHDB, OP_QUERY, serialize(&query)?)?;
        Ok(deserialize(&res)?)
    }
}
