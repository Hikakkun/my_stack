

#[cfg(test)]
mod tests {
    use my_stack::stack::{Stack, StackError};

    #[test]
    fn test_stack_push() {
        let mut stack: Stack<i32> = Stack::new(3);

        let test_case = [1, 2, 3];

        test_case.iter().for_each(|v|{
            stack.push(*v).unwrap();
            assert_eq!(stack.top(), Some(*v));
        });
        assert_eq!(stack.push(10), Err(StackError::Overflow));
    }

    #[test]
    fn test_stack_pop_top() {
        let mut stack: Stack<i32> = Stack::new(3);

        let test_case = [1, 2, 3];

        test_case.iter().for_each(|v|{
            stack.push(*v).unwrap();
        });
        assert_eq!(stack.pop(), Ok(3));
        assert_eq!(stack.top(), Some(2));
        assert_eq!(stack.pop(), Ok(2));
        assert_eq!(stack.top(), Some(1));
        assert_eq!(stack.pop(), Ok(1));
        assert_eq!(stack.top(), None);
        assert_eq!(stack.pop(), Err(StackError::Underflow));
    }

    #[test]
    fn test_stack_empty_size() {
        let mut stack: Stack<i32> = Stack::new(3);

        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
        
        stack.push(1).unwrap();
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 1);
    }
}