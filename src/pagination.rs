//! pagination helpers
//!
//! generic paginator for connection-style graphql results.

use crate::error::{Error, Result};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

/// how many already-used cursors the stall guard keeps to compare against
const CURSOR_WINDOW: usize = 16;

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
    recent: VecDeque<C>,
    done: bool,
    pages_fetched: usize,
    max_pages: Option<usize>,
    _phantom: std::marker::PhantomData<(T, R)>,
}

/// boxed future used by [`DynPaginator`]
pub type BoxFutureResult<'a, R> = Pin<Box<dyn Future<Output = Result<R>> + 'a>>;

/// boxed fetch callback used by [`DynPaginator`]
pub type BoxFetch<'a, C, R> = Box<dyn FnMut(Option<C>) -> BoxFutureResult<'a, R> + 'a>;

/// boxed extract callback used by [`DynPaginator`]
pub type BoxExtract<'a, T, C, R> = Box<dyn FnMut(R) -> Result<EdgePage<T, C>> + 'a>;

/// type-erased paginator for ergonomic api surfaces that cannot expose closure types.
pub type DynPaginator<'a, T, C, R> =
    Paginator<T, C, R, BoxFetch<'a, C, R>, BoxFutureResult<'a, R>, BoxExtract<'a, T, C, R>>;

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
            recent: VecDeque::new(),
            done: false,
            pages_fetched: 0,
            max_pages: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// cap how many pages the walk may fetch
    ///
    /// unset by default. asking for a page beyond the cap yields
    /// [`Error::PaginationLimit`] instead of the page.
    pub fn with_max_pages(mut self, max_pages: usize) -> Self {
        self.max_pages = Some(max_pages);
        self
    }
}

impl<T, C, R, Fetch, Fut, Extract> Paginator<T, C, R, Fetch, Fut, Extract>
where
    C: Clone + PartialEq,
    Fetch: FnMut(Option<C>) -> Fut,
    Fut: Future<Output = Result<R>>,
    Extract: FnMut(R) -> Result<EdgePage<T, C>>,
{
    /// fetch the next page of results
    ///
    /// fails with [`Error::PaginationStalled`] if the page carries a cursor the
    /// walk already used within the last 16 fetches. that catches a server
    /// repeating one cursor and a server cycling through up to 16 of them; a
    /// cycle longer than the window is not detected and is bounded only by
    /// [`Paginator::with_max_pages`], which fails with
    /// [`Error::PaginationLimit`] once exceeded. both errors end the walk:
    /// later calls return `Ok(None)` without another fetch.
    pub async fn next_page(&mut self) -> Result<Option<Vec<T>>> {
        if self.done {
            return Ok(None);
        }
        if let Some(max_pages) = self.max_pages {
            if self.pages_fetched >= max_pages {
                self.done = true;
                return Err(Error::PaginationLimit { max_pages });
            }
        }

        let response = (self.fetch)(self.cursor.clone()).await?;
        let page = (self.extract)(response)?;
        self.pages_fetched += 1;

        if let Some(used) = self.cursor.clone() {
            if self.recent.len() == CURSOR_WINDOW {
                self.recent.pop_front();
            }
            self.recent.push_back(used);
        }

        if let Some(next) = page.next_cursor.as_ref() {
            if self.recent.contains(next) {
                self.done = true;
                return Err(Error::PaginationStalled {
                    pages: self.pages_fetched - 1,
                });
            }
        }

        self.cursor = page.next_cursor.clone();
        if self.cursor.is_none() {
            self.done = true;
        }

        Ok(Some(page.nodes))
    }

    /// fetch all pages and return a single collection
    ///
    /// unbounded unless [`Paginator::with_max_pages`] is set: the stall guard
    /// on [`Paginator::next_page`] ends a short cursor cycle, not a long one.
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
    async fn test_next_page_multi_page() {
        let pages = vec![
            EdgePage {
                nodes: vec![1, 2],
                next_cursor: Some("cur1".to_string()),
            },
            EdgePage {
                nodes: vec![3],
                next_cursor: Some("cur2".to_string()),
            },
            EdgePage {
                nodes: vec![4, 5],
                next_cursor: None,
            },
        ];
        let pages = Arc::new(Mutex::new(pages.into_iter()));

        let pages_fetch = pages.clone();
        let fetch = move |_cursor: Option<String>| {
            let pages = pages_fetch.clone();
            async move {
                let page = pages.lock().unwrap().next().unwrap();
                Ok(page)
            }
        };
        let extract = |page: EdgePage<i32, String>| Ok(page);

        let mut paginator = Paginator::new(fetch, extract);

        let p1 = paginator.next_page().await.unwrap().unwrap();
        assert_eq!(p1, vec![1, 2]);

        let p2 = paginator.next_page().await.unwrap().unwrap();
        assert_eq!(p2, vec![3]);

        let p3 = paginator.next_page().await.unwrap().unwrap();
        assert_eq!(p3, vec![4, 5]);

        assert!(paginator.next_page().await.unwrap().is_none());
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_next_page_single_page_then_done() {
        // (label, nodes of the only page)
        for (label, nodes) in [("empty", vec![]), ("non-empty", vec![42])] {
            let page_nodes = nodes.clone();
            let fetch = move |_: Option<()>| {
                let nodes = page_nodes.clone();
                async move {
                    Ok(EdgePage::<i32, ()> {
                        nodes,
                        next_cursor: None,
                    })
                }
            };
            let extract = |page: EdgePage<i32, ()>| Ok(page);

            let mut paginator = Paginator::new(fetch, extract);
            assert_eq!(
                paginator.next_page().await.unwrap(),
                Some(nodes),
                "{label}: first page"
            );
            assert_eq!(paginator.next_page().await.unwrap(), None, "{label}: done");
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_normal_multi_page_walk_unaffected() {
        let seen: Arc<Mutex<Vec<Option<String>>>> = Arc::new(Mutex::new(Vec::new()));
        let seen_fetch = seen.clone();

        let fetch = move |cursor: Option<String>| {
            let seen = seen_fetch.clone();
            async move {
                seen.lock().unwrap().push(cursor.clone());
                let page = match cursor.as_deref() {
                    None => EdgePage {
                        nodes: vec![1, 2],
                        next_cursor: Some("c1".to_string()),
                    },
                    Some("c1") => EdgePage {
                        nodes: vec![3],
                        next_cursor: Some("c2".to_string()),
                    },
                    Some("c2") => EdgePage {
                        nodes: vec![4, 5],
                        next_cursor: None,
                    },
                    other => panic!("unexpected cursor {other:?}"),
                };
                Ok(page)
            }
        };
        let extract = |page: EdgePage<i32, String>| Ok(page);

        let items = Paginator::new(fetch, extract).collect_all().await.unwrap();

        assert_eq!(items, vec![1, 2, 3, 4, 5]);
        assert_eq!(
            *seen.lock().unwrap(),
            vec![None, Some("c1".to_string()), Some("c2".to_string())]
        );
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_repeated_cursor_ends_the_walk() {
        let calls: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let calls_fetch = calls.clone();

        let fetch = move |_cursor: Option<String>| {
            let calls = calls_fetch.clone();
            async move {
                let mut count = calls.lock().unwrap();
                *count += 1;
                assert!(*count <= 8, "paginator kept fetching a repeated cursor");
                Ok(EdgePage {
                    nodes: vec![7],
                    next_cursor: Some("stuck".to_string()),
                })
            }
        };
        let extract = |page: EdgePage<i32, String>| Ok(page);

        let mut paginator = Paginator::new(fetch, extract);

        assert_eq!(paginator.next_page().await.unwrap().unwrap(), vec![7]);
        assert!(matches!(
            paginator.next_page().await.unwrap_err(),
            Error::PaginationStalled { pages: 1 }
        ));
        assert!(paginator.next_page().await.unwrap().is_none());
        assert_eq!(*calls.lock().unwrap(), 2);
    }

    async fn walk_cursor_cycle(
        period: usize,
        max_pages: Option<usize>,
        ceiling: u32,
    ) -> (Error, u32) {
        let calls: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let calls_fetch = calls.clone();

        let fetch = move |cursor: Option<String>| {
            let calls = calls_fetch.clone();
            async move {
                let mut count = calls.lock().unwrap();
                *count += 1;
                assert!(*count <= ceiling, "paginator kept walking a {period}-cycle");
                let next = match cursor {
                    None => 0,
                    Some(ref c) => {
                        (c.trim_start_matches('c').parse::<usize>().unwrap() + 1) % period
                    }
                };
                Ok(EdgePage {
                    nodes: vec![1],
                    next_cursor: Some(format!("c{next}")),
                })
            }
        };
        let extract = |page: EdgePage<i32, String>| Ok(page);

        let mut paginator = Paginator::new(fetch, extract);
        if let Some(max_pages) = max_pages {
            paginator = paginator.with_max_pages(max_pages);
        }
        let err = paginator.collect_all().await.unwrap_err();
        let count = *calls.lock().unwrap();
        (err, count)
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_cursor_cycles() {
        // (cycle period, max_pages, fetch ceiling, expected error, expected calls)
        for (period, max_pages, ceiling, expected, expected_calls) in [
            // repeated cursor
            (1, None, 8, Error::PaginationStalled { pages: 1 }, 2),
            // alternating cursors
            (2, None, 8, Error::PaginationStalled { pages: 2 }, 3),
            // cycle at the documented window boundary is caught
            (16, None, 20, Error::PaginationStalled { pages: 16 }, 17),
            // cycle past the window is bounded only by max_pages
            (
                17,
                Some(48),
                52,
                Error::PaginationLimit { max_pages: 48 },
                48,
            ),
        ] {
            let (err, calls) = walk_cursor_cycle(period, max_pages, ceiling).await;

            let same = match (&err, &expected) {
                (
                    Error::PaginationStalled { pages: got },
                    Error::PaginationStalled { pages: want },
                ) => got == want,
                (
                    Error::PaginationLimit { max_pages: got },
                    Error::PaginationLimit { max_pages: want },
                ) => got == want,
                _ => false,
            };
            assert!(same, "period {period}: got {err:?}, want {expected:?}");
            assert_eq!(calls, expected_calls, "period {period}: calls");
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_long_walk_of_distinct_cursors_never_stalls() {
        let total = CURSOR_WINDOW * 8;

        let fetch = move |cursor: Option<String>| async move {
            let n = match cursor {
                None => 0usize,
                Some(ref c) => c.trim_start_matches('c').parse::<usize>().unwrap(),
            };
            Ok(EdgePage {
                nodes: vec![n as i32],
                next_cursor: (n + 1 < total).then(|| format!("c{}", n + 1)),
            })
        };
        let extract = |page: EdgePage<i32, String>| Ok(page);

        let items = Paginator::new(fetch, extract).collect_all().await.unwrap();

        assert_eq!(items.len(), total);
        assert_eq!(items.first(), Some(&0));
        assert_eq!(items.last(), Some(&(total as i32 - 1)));
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_max_pages_bounds_an_endless_walk() {
        let calls: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let calls_fetch = calls.clone();

        let fetch = move |_cursor: Option<String>| {
            let calls = calls_fetch.clone();
            async move {
                let mut count = calls.lock().unwrap();
                *count += 1;
                assert!(*count <= 8, "paginator ignored max_pages");
                Ok(EdgePage {
                    nodes: vec![*count as i32],
                    next_cursor: Some(format!("cur{count}")),
                })
            }
        };
        let extract = |page: EdgePage<i32, String>| Ok(page);

        let err = Paginator::new(fetch, extract)
            .with_max_pages(3)
            .collect_all()
            .await
            .unwrap_err();

        assert!(matches!(err, Error::PaginationLimit { max_pages: 3 }));
        assert_eq!(*calls.lock().unwrap(), 3);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_max_pages_matching_page_count_succeeds() {
        let fetch = move |cursor: Option<String>| async move {
            let page = match cursor.as_deref() {
                None => EdgePage {
                    nodes: vec![1],
                    next_cursor: Some("c1".to_string()),
                },
                Some("c1") => EdgePage {
                    nodes: vec![2],
                    next_cursor: Some("c2".to_string()),
                },
                Some("c2") => EdgePage {
                    nodes: vec![3],
                    next_cursor: None,
                },
                other => panic!("unexpected cursor {other:?}"),
            };
            Ok(page)
        };
        let extract = |page: EdgePage<i32, String>| Ok(page);

        let items = Paginator::new(fetch, extract)
            .with_max_pages(3)
            .collect_all()
            .await
            .unwrap();

        assert_eq!(items, vec![1, 2, 3]);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_max_pages_zero_fetches_nothing() {
        let calls: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let calls_fetch = calls.clone();

        let fetch = move |_cursor: Option<String>| {
            let calls = calls_fetch.clone();
            async move {
                *calls.lock().unwrap() += 1;
                Ok(EdgePage {
                    nodes: vec![1],
                    next_cursor: None,
                })
            }
        };
        let extract = |page: EdgePage<i32, String>| Ok(page);

        let err = Paginator::new(fetch, extract)
            .with_max_pages(0)
            .collect_all()
            .await
            .unwrap_err();

        assert!(matches!(err, Error::PaginationLimit { max_pages: 0 }));
        assert_eq!(*calls.lock().unwrap(), 0);
    }
}
