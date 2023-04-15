macro_rules! define_packets {
    (
        $(
            $(pub)? mod $direction:ident {
                $(
                    enum $packet_type:ident : $mode:ident {
                        $(
                            $packet_name:ident {
                                $(
                                    $field_name:ident : $field_type:ty
                                ),* $(,)?
                            }
                        ),* $(,)?
                    }
                )*
            }
        )*
    ) => {
        $(
            $(
                enum $packet_type {
                    $( $packet_name($direction::$mode::$packet_name) ),*
                }
            )*
        )*

        $(
            pub mod $direction {
                use super::*;
                $(
                    pub mod $mode {
                        use super::*;

                        $(
                            #[derive(Default)]
                            pub struct $packet_name {
                                $(
                                    pub $field_name : $field_type
                                ),*
                            }

                            impl $packet_name {
                                fn write_to(&self, writer: &mut impl std::io::Write) -> anyhow::Result<()> {
                                    $(
                                        self.$field_name.minecraft_write(writer)?;
                                    )*
                                    Ok(())
                                }

                                fn read_from(reader: &mut impl std::io::Read) -> anyhow::Result<Self> {
                                    let mut v: Self = Default::default();
                                    $(
                                        v.$field_name = <$field_type>::minecraft_read(reader)?;
                                    )*
                                    Ok(v)
                                }
                            }
                        )*
                    }
                )*
            }
        )*
    }
}
