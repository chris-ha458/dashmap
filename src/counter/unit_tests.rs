use crate::DashCounter;

    #[test]
    fn test_basic() {
        let set = DashCounter::new();

        set.insert(0);

        assert_eq!(set.get(&0).as_deref(), Some(&0));
    }

    #[test]
    fn test_default() {
        let set: DashCounter<u32> = DashCounter::default();

        set.insert(0);

        assert_eq!(set.get(&0).as_deref(), Some(&0));
    }

    #[test]
    fn test_multiple_hashes() {
        let set = DashCounter::<u32>::default();

        for i in 0..100 {
            assert!(set.insert(i));
        }

        for i in 0..100 {
            assert!(!set.insert(i));
        }

        for i in 0..100 {
            assert_eq!(Some(i), set.remove(&i));
        }

        for i in 0..100 {
            assert_eq!(None, set.remove(&i));
        }
    }