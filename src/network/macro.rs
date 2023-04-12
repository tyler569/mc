macro_rules! define_packets {
    (
        enum $name:ident {
            $(
                $packet_id:literal : $packet_name:ident {
                    $(
                        $field:ident : $field_type:ty $(count $field_count:literal)? $(if $field_condition:tt)?
                    ),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        $(
            struct $packet_name {
                $(
                    $field_name: $field_type
                ),*
            }

            impl $packet_name {
                fn write(&self, writer: &mut impl std::io::Write) -> anyhow::Result<()> {
                    $(
                        self.$field_name.minecraft_write(writer)?;
                    )*
                    Ok(())
                }

                fn read(reader: &mut impl std::io::Read) -> anyhow::Result<Self> {
                    $(
                        if (true && $field_condition)
                        <$field_type>::minecraft_read(reader)?;
                    )*
                }
            }
        )*
    };
}

define_packets! {
    enum ClientBountPlayPacket {
        10: EntityPositionAndRotation {
            entity_id: VarInt,
            x: i32,
            y: i32,
            z: i32,
            pitch: Angle,
            yaw: Angle,
        },
        11: SomethingOptional {
            thing_exists: bool,
            thing: Option<i32> if |self| self.thing_exists,
            thing_count: VarInt,
            things: Vec<i32> count |self| self.thing_count,
        }
    }
}
