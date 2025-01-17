// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`DescribeBatchPredictions`](crate::operation::DescribeBatchPredictions)
pub struct DescribeBatchPredictionsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::describe_batch_predictions_input::Builder,
}

impl<C, M, R> DescribeBatchPredictionsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::describe_batch_predictions_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `limit`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.limit = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `results`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::DescribeBatchPredictionsPaginatorItems<C, M, R> {
        crate::paginator::DescribeBatchPredictionsPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::DescribeBatchPredictionsOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeBatchPredictionsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeBatchPredictionsInputOperationOutputAlias,
            crate::output::DescribeBatchPredictionsOutput,
            crate::error::DescribeBatchPredictionsError,
            crate::input::DescribeBatchPredictionsInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_describe_batch_predictions_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`DescribeDataSources`](crate::operation::DescribeDataSources)
pub struct DescribeDataSourcesPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::describe_data_sources_input::Builder,
}

impl<C, M, R> DescribeDataSourcesPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::describe_data_sources_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `limit`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.limit = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `results`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::DescribeDataSourcesPaginatorItems<C, M, R> {
        crate::paginator::DescribeDataSourcesPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::DescribeDataSourcesOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeDataSourcesError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeDataSourcesInputOperationOutputAlias,
            crate::output::DescribeDataSourcesOutput,
            crate::error::DescribeDataSourcesError,
            crate::input::DescribeDataSourcesInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_describe_data_sources_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`DescribeEvaluations`](crate::operation::DescribeEvaluations)
pub struct DescribeEvaluationsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::describe_evaluations_input::Builder,
}

impl<C, M, R> DescribeEvaluationsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::describe_evaluations_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `limit`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.limit = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `results`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::DescribeEvaluationsPaginatorItems<C, M, R> {
        crate::paginator::DescribeEvaluationsPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::DescribeEvaluationsOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeEvaluationsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeEvaluationsInputOperationOutputAlias,
            crate::output::DescribeEvaluationsOutput,
            crate::error::DescribeEvaluationsError,
            crate::input::DescribeEvaluationsInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_describe_evaluations_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`DescribeMLModels`](crate::operation::DescribeMLModels)
pub struct DescribeMlModelsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::describe_ml_models_input::Builder,
}

impl<C, M, R> DescribeMlModelsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::describe_ml_models_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `limit`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.limit = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `results`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::DescribeMlModelsPaginatorItems<C, M, R> {
        crate::paginator::DescribeMlModelsPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::DescribeMlModelsOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeMLModelsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeMlModelsInputOperationOutputAlias,
            crate::output::DescribeMlModelsOutput,
            crate::error::DescribeMLModelsError,
            crate::input::DescribeMlModelsInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_describe_ml_models_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Flattened paginator for `DescribeBatchPredictionsPaginator`
///
/// This is created with [`.items()`](DescribeBatchPredictionsPaginator::items)
pub struct DescribeBatchPredictionsPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(DescribeBatchPredictionsPaginator<C, M, R>);

impl<C, M, R> DescribeBatchPredictionsPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::BatchPrediction,
            aws_smithy_http::result::SdkError<crate::error::DescribeBatchPredictionsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeBatchPredictionsInputOperationOutputAlias,
            crate::output::DescribeBatchPredictionsOutput,
            crate::error::DescribeBatchPredictionsError,
            crate::input::DescribeBatchPredictionsInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_describe_batch_predictions_output_results(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}

/// Flattened paginator for `DescribeDataSourcesPaginator`
///
/// This is created with [`.items()`](DescribeDataSourcesPaginator::items)
pub struct DescribeDataSourcesPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(DescribeDataSourcesPaginator<C, M, R>);

impl<C, M, R> DescribeDataSourcesPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::DataSource,
            aws_smithy_http::result::SdkError<crate::error::DescribeDataSourcesError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeDataSourcesInputOperationOutputAlias,
            crate::output::DescribeDataSourcesOutput,
            crate::error::DescribeDataSourcesError,
            crate::input::DescribeDataSourcesInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_describe_data_sources_output_results(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}

/// Flattened paginator for `DescribeEvaluationsPaginator`
///
/// This is created with [`.items()`](DescribeEvaluationsPaginator::items)
pub struct DescribeEvaluationsPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(DescribeEvaluationsPaginator<C, M, R>);

impl<C, M, R> DescribeEvaluationsPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::Evaluation,
            aws_smithy_http::result::SdkError<crate::error::DescribeEvaluationsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeEvaluationsInputOperationOutputAlias,
            crate::output::DescribeEvaluationsOutput,
            crate::error::DescribeEvaluationsError,
            crate::input::DescribeEvaluationsInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_describe_evaluations_output_results(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}

/// Flattened paginator for `DescribeMlModelsPaginator`
///
/// This is created with [`.items()`](DescribeMlModelsPaginator::items)
pub struct DescribeMlModelsPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(DescribeMlModelsPaginator<C, M, R>);

impl<C, M, R> DescribeMlModelsPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::MlModel,
            aws_smithy_http::result::SdkError<crate::error::DescribeMLModelsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::DescribeMlModelsInputOperationOutputAlias,
            crate::output::DescribeMlModelsOutput,
            crate::error::DescribeMLModelsError,
            crate::input::DescribeMlModelsInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_describe_ml_models_output_results(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}
