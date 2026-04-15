/// Rate limiting middleware for axum 0.7.
///
/// Uses axum::middleware::from_fn_with_state pattern instead of tower layers
/// to avoid Buffer error-type incompatibility (Buffer<T>::Error = BoxError,
/// which is not Into<Infallible> as axum requires).
///
/// Applied limits:
///   - /api/auth/* : 5 requests / 60 seconds
///   - /api/ze-guide/* : 20 requests / 60 seconds
///   - General API: 120 requests / 60 seconds

use axum::{extract::State, http::StatusCode, middleware::Next, response::Response};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// Shared state for a sliding-window rate limiter.
/// Arc<Mutex<...>> makes it Clone and Send + Sync.
#[derive(Clone, Debug)]
pub struct RateLimiter {
    state: Arc<Mutex<VecDeque<Instant>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(VecDeque::new())),
            max_requests,
            window,
        }
    }

    fn is_allowed(&self) -> bool {
        let mut timestamps = self.state.lock().expect("rate limiter lock poisoned");
        let now = Instant::now();
        // Evict timestamps outside the window
        while let Some(&front) = timestamps.front() {
            if now.duration_since(front) > self.window {
                timestamps.pop_front();
            } else {
                break;
            }
        }
        if timestamps.len() < self.max_requests {
            timestamps.push_back(now);
            true
        } else {
            false
        }
    }
}

/// Middleware function — use with `middleware::from_fn_with_state(limiter, rate_limit_fn)`.
pub async fn rate_limit_fn(
    State(limiter): State<RateLimiter>,
    req: axum::extract::Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if limiter.is_allowed() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}

pub fn auth_limiter() -> RateLimiter {
    RateLimiter::new(5, Duration::from_secs(60))
}

pub fn ze_guide_limiter() -> RateLimiter {
    RateLimiter::new(20, Duration::from_secs(60))
}

pub fn api_limiter() -> RateLimiter {
    RateLimiter::new(120, Duration::from_secs(60))
}
