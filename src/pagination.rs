//! pagination helpers
//!
//! generic paginator for connection-style graphql results.

use crate::error::Result;
use std::future::Future;

/// a single page of connection results
#[derive(Debug, Clone)]
pub struct EdgePage<T, C> {
    /// node payloads for this page
    pub nodes: Vec<T>,
    /// next cursor (if any)
    pub next_cursor: Option<C>,
}

/// generic paginator for connection-style data
pub struct Paginator<T, C, R, Fetch, Fut, Extract>
where
    C: Clone,
    Fetch: FnMut(Option<C>) -> Fut,
    Fut: Future<Output = Result<R>>,
    Extract: FnMut(R) -> Result<EdgePage<T, C>>,
{
    fetch: Fetch,
    extract: Extract,
    cursor: Option<C>,
    done: bool,
    _phantom: std::marker::PhantomData<(T, R)>,
}

impl<T, C, R, Fetch, Fut, Extract> Paginator<T, C, R, Fetch, Fut, Extract>
where
    C: Clone,
    Fetch: FnMut(Option<C>) -> Fut,
    Fut: Future<Output = Result<R>>,
    Extract: FnMut(R) -> Result<EdgePage<T, C>>,
{
    /// create a new paginator
    pub fn new(fetch: Fetch, extract: Extract) -> Self {
        Self {
            fetch,
            extract,
            cursor: None,
            done: false,
            _phantom: std::marker::PhantomData,
        }
    }

    /// fetch the next page of results
    pub async fn next_page(&mut self) -> Result<Option<Vec<T>>> {
        if self.done {
            return Ok(None);
        }

        let response = (self.fetch)(self.cursor.clone()).await?;
        let page = (self.extract)(response)?;
        self.cursor = page.next_cursor.clone();
        if self.cursor.is_none() {
            self.done = true;
        }

        Ok(Some(page.nodes))
    }

    /// fetch all pages and return a single collection
    pub async fn collect_all(mut self) -> Result<Vec<T>> {
        let mut items = Vec::new();
        while let Some(page) = self.next_page().await? {
            items.extend(page);
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_pagination_collect_all() {
        let state: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let state_fetch = state.clone();

        let fetch = move |cursor: Option<String>| {
            let state = state_fetch.clone();
            async move {
                let mut count = state.lock().unwrap();
                *count += 1;
                if cursor.is_none() {
                    Ok(EdgePage {
                        nodes: vec![1, 2],
                        next_cursor: Some("next".to_string()),
                    })
                } else {
                    Ok(EdgePage {
                        nodes: vec![3],
                        next_cursor: None,
                    })
                }
            }
        };

        let extract = |page: EdgePage<i32, String>| Ok(page);

        let paginator = Paginator::new(fetch, extract);
        let items = paginator.collect_all().await.unwrap();
        assert_eq!(items, vec![1, 2, 3]);
        assert_eq!(*state.lock().unwrap(), 2);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_pagination_next_page_done() {
        let fetch = |_: Option<()>| async {
            Ok(EdgePage::<i32, ()> {
                nodes: vec![42],
                next_cursor: None,
            })
        };
        let extract = |page: EdgePage<i32, ()>| Ok(page);

        let mut paginator = Paginator::new(fetch, extract);
        let page = paginator.next_page().await.unwrap();
        assert_eq!(page.unwrap(), vec![42]);
        let none = paginator.next_page().await.unwrap();
        assert!(none.is_none());
    }
}
