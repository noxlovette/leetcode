use std::collections::VecDeque;

#[derive(Default)]
#[allow(dead_code)]
struct RecentCounter {
    queue: VecDeque<i32>,
}

#[allow(dead_code)]
impl RecentCounter {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_front(t);

        self.queue
            .iter()
            .filter(|el| (t - 3000..=t).contains(*el))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::RecentCounter;

    #[test]
    fn test1() {
        let mut c = RecentCounter::new();
        let first = c.ping(1);
        let second = c.ping(100);
        let third = c.ping(3001);
        let fourth = c.ping(3002);

        assert_eq!(first, 1);
        assert_eq!(second, 2);
        assert_eq!(third, 3);
        assert_eq!(fourth, 3);
    }
}
