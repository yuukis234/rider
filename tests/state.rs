#[cfg(test)]
mod tests {
    use rider::libs::state::state::{Status};

    #[test]
    fn object_status() {
        let status = Status{hash:"test" ,state: 1.0};
        assert_eq!(status.getkey(), 1.0)
    }

    fn hash_status() {
        let status = Status{hash:"test" ,state: 1.0};
        assert_eq!(status.gethash(), "test");
    }
}
