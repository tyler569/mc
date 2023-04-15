    use super::*;
    use crate::network::Angle;

    #[test]
    fn test_write_u64() {
        let mut vec = vec![];
        64u64.minecraft_write(&mut vec);
        assert_eq!(vec, &[0, 0, 0, 0, 0, 0, 0, 64]);
    }

    #[test]
    fn test_write_string() {
        let mut vec = vec![];
        "Hello World".to_owned().minecraft_write(&mut vec);
        assert_eq!(vec, b"\x0bHello World");
    }

    #[test]
    fn test_write_position() {
        let mut vec = vec![];
        let position = Position { x: 1, y: 1, z: 1 };
        position.minecraft_write(&mut vec);
        assert_eq!(vec, &[0, 0, 0, 64, 0, 0, 16, 1]);
    }

    #[test]
    fn test_write_array() {
        let mut vec = vec![];
        let numbers = vec![2u16, 3u16, 4u16, 5u16];
        numbers.minecraft_write(&mut vec);
        assert_eq!(vec, vec![4, 0, 2, 0, 3, 0, 4, 0, 5]);
    }

    #[test]
    fn test_write_optional() {
        let mut vec = vec![];
        let mut maybe = Some(3i32);
        maybe.minecraft_write(&mut vec);
        assert_eq!(vec, &[0, 0, 0, 3]);

        vec.clear();
        maybe = None;
        maybe.minecraft_write(&mut vec);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_write_angle() {
        let mut vec = vec![];
        let angle = Angle(PI);
        angle.minecraft_write(&mut vec);
        assert_eq!(vec, &[128]);
    }

    #[test]
    fn test_write_uuid() {
        let mut vec = vec![];
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"testing");
        // 013fad7b-475f-55b4-b2b7-0da6c41293a8
        uuid.minecraft_write(&mut vec);
        assert_eq!(
            vec,
            &[1, 63, 173, 123, 71, 95, 85, 180, 178, 183, 13, 166, 196, 18, 147, 168]
        );
    }

    #[test]
    fn test_write_simple_nbt() {
        let mut vec = vec![];
        let mut nbt = Nbt(nbt::Blob::new());
        nbt.0.insert("Key", "Value");
        nbt.minecraft_write(&mut vec);
        assert_eq!(
            vec,
            &[10, 0, 0, 8, 0, 3, 75, 101, 121, 0, 5, 86, 97, 108, 117, 101, 0]
        );
    }

    #[test]
    fn test_write_slot() {
        let mut vec = vec![];
        let mut slot = Slot::Nothing;
        slot.minecraft_write(&mut vec);
        assert_eq!(vec, &[0]);

        vec.clear();
        slot = Slot::Item {
            id: VarInt(1),
            count: 1,
            nbt: None,
        };
        slot.minecraft_write(&mut vec);
        assert_eq!(vec, &[1, 1, 1, 0]);
    }
