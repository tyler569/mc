use uuid::Uuid;

mod varint;
mod packet;

use varint::{VarInt, VarLong};

pub struct Position {
    x: u32,
    y: u32,
    z: u32,
}

struct Slot {}
struct Identifier(String);
struct Chat(String);
struct Angle(i8);
struct Nbt {}
struct BitSet {}
struct CommandNode {}
struct Statistic {}
enum Either<T, U> {
    Left(T),
    Right(U),
}

struct EntityMetadata {
    // complicated.
}

struct EntityProperty {
    key: Identifier,
    value: f64,
    modifier_count: VarInt,
    modifiers: Vec<()>, // TODO
}

struct Recipe {
    // complicated
}

struct Tag {
    // complicated
}

enum BossBarAction {
    Add {
        title: String,
        health: f32,
        color: VarInt,
        division: VarInt,
        flags: u8,
    },
    Remove {},
    UpdateHealth {
        health: f32,
    },
    UpdateTitle {
        title: String,
    },
    UpdateStyle {
        color: VarInt,
        dividers: VarInt,
    },
    UpdateFlags {
        flags: u8,
    },
}

enum ServerBoundHandshakePacket {
    Handshake {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: VarInt,
    },
}

enum ServerBoundStatusPacket {
    Request {},
    Ping {
        payload: i64,
    },
}

enum ServerBoundLoginPacket {
    LoginStart {
        username: String,
    },
    EncryptionResponse {
        shared_secret: Vec<u8>,
        verify_token: Vec<u8>,
    },
    LoginPluginResponse {
        message_id: VarInt,
        success: bool,
        data: Vec<u8>,
    },
}

enum ServerBoundPlayPacket {
    TeleportConfirm {
        teleport_id: VarInt,
    },
    QueryBlockNbt {
        transaction_id: VarInt,
        location: Position,
    },
    SetDifficulty {
        new_difficulty: i8,
    },
    Chat {
        message: String,
    },
    ClientStatus {
        action_id: VarInt,
    },
    ClientSettings {
        locale: String,
        view_distance: i8,
        chat_mode: VarInt,
        chat_colors: bool,
        displayed_skin_parts: u8,
        main_hand: VarInt,
        enable_text_filtering: bool,
        allow_server_listings: bool,
    },
    TabComplete {
        transaction_id: VarInt,
        text: String,
    },
    ClickWindowButton {
        window_id: i8,
        button_id: i8,
    },
    ClickWindow {
        window_id: u8,
        state_id: VarInt,
        slot: i16,
        button: i8,
        mode: VarInt,
        slots: Vec<(i16, Slot)>,
        carried_item: Slot,
    },
    CloseWindow {
        window_id: u8,
    },
    PluginMessage {
        channel: Identifier,
        data: Vec<u8>,
    },
    EditBook {
        slot: VarInt,
        count: VarInt,
        entries: Vec<String>,
        title: Option<String>,
    },
    QueryEntityNbt {
        transaction_id: VarInt,
        entity_id: VarInt,
    },
    InteractEntity {
        entity_id: VarInt,
        entity_type: VarInt,
        position: Option<(f32, f32, f32)>,
        hand: Option<VarInt>,
        sneaking: bool,
    },
    GenerateStructure {
        location: Position,
        levels: VarInt,
        keep_jigsaws: bool,
    },
    KeepAlive {
        keep_alive_id: i64,
    },
    LockDifficulty {
        locked: bool,
    },
    PlayerPosition {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
    },
    PlayerPositionAndRotation {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    PlayerRotation {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    PlayerMovement {
        on_ground: bool,
    },
    VehicleMove {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
    },
    SteerBoat {
        left_paddle: bool,
        right_paddle: bool,
    },
    PickItem {
        slot: VarInt,
    },
    CraftRecipieRequest {
        window_id: i8,
        recipe: Identifier,
        make_all: bool,
    },
    PlayerAbilities {
        flags: u8,
    },
    PlayerDigging {
        status: VarInt, // enum
        location: Position,
        face: u8, // enum
    },
    EntityAction {
        entity_id: VarInt,
        action_id: VarInt, // enum
        jump_boost: VarInt,
    },
    SteerVehicle {
        sideways: f32,
        forward: f32,
        flags: u8,
    },
    Pong {
        id: i32,
    },
    SetRecipeBookState {
        book_id: VarInt, // enum
        book_open: bool,
        filter_active: bool,
    },
    SetDelayedRecipe {
        recipe_id: Identifier,
    },
    NameItem {
        item_name: String,
    },
    ResourcePackStatus {
        result: VarInt, // enum
    },
    AdvancementTab {
        action: VarInt,
        // enum
        tab_id: Option<Identifier>,
    },
    SelectTrade {
        selected_slot: VarInt,
    },
    SetBeaconEffect {
        primary_effect: VarInt, // potion ID
        secondary_effect: VarInt, // potion ID
    },
    HeldItemChange {
        slot: i16,
    },
    UpdateCommandBlock {
        location: Position,
        command: String,
        mode: VarInt, // enum
        flags: u8,
    },
    UpdateCommandBlockMinecart {
        entity_id: VarInt,
        command: String,
        track_output: bool,
    },
    CreativeInventoryAction {
        slot: i16,
        clicked_item: Slot,
    },
    UpdateJigsawBlock {
        location: Position,
        name: Identifier,
        target: Identifier,
        pool: Identifier,
        final_state: String,
        joint_type: String,
    },
    UpdateStructureBlock {
        location: Position,
        action: VarInt, // enum
        mode: VarInt, // enum
        name: String,
        offset_x: i8,
        offset_y: i8,
        offset_z: i8,
        size_x: i8,
        size_y: i8,
        size_z: i8,
        mirror: VarInt, // enum
        rotation: VarInt, // enum
        metadata: String,
        integrity: f32,
        seed: VarLong,
        flags: u8,
    },
    UpdateSign {
        location: Position,
        line1: String,
        line2: String,
        line3: String,
        line4: String,
    },
    Animation {
        hand: VarInt, // enum
    },
    Spectate {
        target_player: Uuid,
    },
    PlayerBlockPlacement {
        hand: VarInt, // enum
        location: Position,
        face: VarInt, // enum
        cursor_position: (f32, f32, f32),
        inside_block: bool,
    },
    UseItem {
        hand: VarInt, // enum
    },
}

enum ClientBoundHandshakePacket {}

enum ClientBoundStatusPacket {
    Response {
        json_data: String,
    },
    Pong {
        payload: i64,
    },
}

enum ClientBoundLoginPacket {
    Disconnect {
        reason: String,
    },
    EncryptionRequest {
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>,
    },
    LoginSuccess {
        uuid: Uuid,
        usename: String,
    },
    SetCompression {
        threshold: VarInt,
    },
    LoginPluginRequest {
        message_id: VarInt,
        channel: Identifier,
        data: Vec<u8>,
    },
}

enum ClientBoundPlayPacket {
    SpawnEntity {
        entity_id: VarInt,
        object_uuid: Uuid,
        object_type: VarInt, // enum
        x: f64,
        y: f64,
        z: f64,
        pitch: Angle,
        yaw: Angle,
        data: i32,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
    },
    SpawnExperienceOrb {
        entity_id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        count: i16,
    },
    SpawnLivingEntity {
        entity_id: VarInt,
        entity_uuid: Uuid,
        entity_type: VarInt,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
    },
    SpawnPainting {
        entity_id: VarInt,
        entity_uuid: Uuid,
        motive: VarInt, // painting ID (why motive?)
        location: Position,
        direction: u8, // enum
    },
    SpawnPlayer {
        entity_id: VarInt,
        player_uuid: Uuid,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle,
    },
    SkulkVibrationSignal {
        source_position: Position,
        destination_identifier: Identifier,
        destination: Either<Position, VarInt>,
        arrival_ticks: VarInt,
    },
    EntityAnimation {
        entity_id: VarInt,
        animation: u8, // animation ID
    },
    Statistics {
        statistics: Vec<Statistic>,
    },
    AcknowlegePlayerDigging {
        location: Position,
        block: VarInt,
        status: VarInt, // enum
        successful: bool,
    },
    BlockBreakAnimation {
        entity_id: VarInt,
        location: Position,
        destroy_stage: i8, // 0-9
    },
    BlockEntityData {
        location: Position,
        entity_type: VarInt, //
        nbt_data: Nbt,
    },
    BlockAction {
        location: Position,
        action_id: u8,
        action_param: u8,
        block_type: VarInt,
    },
    BlockChange {
        location: Position,
        block_id: VarInt,
    },
    BossBar {
        uuid: Uuid,
        action: BossBarAction,
    },
    ServerDifficulty {
        difficulty: u8, // enum
        difficulty_locked: bool,
    },
    ChatMessage {
        chat_json: String,
        position: u8, // enum
        sender: Uuid,
    },
    ClearTitles {
        reset: bool,
    },
    TabComplete {
        id: VarInt,
        start: VarInt,
        length: VarInt,
        count: VarInt,
        matches: Vec<(String, bool, Option<String>)>,
    },
    DeclareCommands {
        nodes: Vec<CommandNode>,
        root_index: VarInt,
    },
    CloseWindow {
        window_id: u8,
    },
    WindowItems {
        window_id: u8,
        state_id: VarInt, // enum
        slot_data: Vec<Slot>,
        carried_item: Slot,
    },
    WindowProperty {
        window_id: u8,
        property: i16,
        value: i16,
    },
    SetSlot {
        window_id: u8,
        state_id: VarInt,
        slot: i16,
        slot_data: Slot,
    },
    SetCooldown {
        item_id: VarInt,
        cooldown_ticks: VarInt,
    },
    PluginMessage {
        channel: Identifier,
        data: Vec<u8>,
    },
    NamedSoundEffect {
        sound_name: Identifier,
        sound_category: VarInt, // enum
        effect_position: (i32, i32, i32),
        volume: f32,
        pitch: f32,
    },
    Disconnect {
        reason: String,
    },
    EntityStatus {
        entity_id: i32,
        entity_status: i8, // enum
    },
    Explosion {
        x: f32,
        y: f32,
        z: f32,
        strength: f32,
        records: Vec<(u8, u8, u8)>,
        player_motion: (f32, f32, f32),
    },
    UnloacChunk {
        chunk: (i32, i32),
    },
    ChangeGameState {
        reason: u8,
        value: f32,
    },
    OpenHorseWindow {
        window_id: u8,
        slot_count: VarInt,
        entity_id: i32,
    },
    InitializeWorldBorder {
        x: f64,
        z: f64,
        old_diameter: f64,
        new_diameter: f64,
        speed: VarLong,
        portal_teleport_boundary: VarInt,
        warning_blocks: VarInt,
        warning_time: VarInt,
    },
    KeepAlive {
        keep_alive_id: i64,
    },
    ChunkDataAndUpdateLight {
        chunk_x: i32,
        chunk_z: i32,
        heightmaps: Nbt,
        size: VarInt,
        data: Vec<u8>,
        block_entities: Vec<(i8, i16, VarInt, Nbt)>,
        trust_edges: bool,
        sky_light_mask: BitSet,
        block_light_mask: BitSet,
        empty_sky_light_mask: BitSet,
        empty_block_light_mask: BitSet,
        sky_light_array: Vec<()>,
        block_light_array: Vec<()>,
    },
    Effect {
        effect_id: i32,
        location: Position,
        data: i32,
        disable_relative_volume: bool,
    },
    Particle {
        particle_id: i32,
        long_distance: bool,
        x: f64,
        y: f64,
        z: f64,
        offset_x: f32,
        offset_y: f32,
        offset_z: f32,
        particle_data: f32,
        particle_count: i32,
        data: ()
    },
    UpdateLight {
        chunk_x: VarInt,
        chunk_z: VarInt,
        trust_edges: bool,
        sky_light_mask: BitSet,
        block_light_mask: BitSet,
        empty_sky_light_mask: BitSet,
        empty_block_light_mask: BitSet,
        sky_light_array: Vec<()>,
        block_light_array: Vec<()>,
    },
    JoinGame {
        entity_id: i32,
        is_hardcore: bool,
        gamemode: u8,
        previous_gamemode: i8,
        world_count: VarInt,
        dimension_names: Vec<Identifier>,
        dimension_codec: Nbt,
        dimension: Nbt,
        dimension_name: Identifier,
        hashed_seed: i64,
        max_players: VarInt,
        view_distance: VarInt,
        simulation_distance: VarInt,
        reduced_debug_info: bool,
        enable_respawn_screen: bool,
        is_debug: bool,
        is_flat: bool,
    },
    MapData {
        map_id: VarInt,
        scale: i8,
        locked: bool,
        tracking_position: bool,
        icons: Vec<(VarInt, i8, i8, i8, Option<String>)>,
        columns: Option<u8>,
        rows: Option<u8>,
        x: Option<i8>,
        z: Option<i8>,
        length: Option<VarInt>,
        data: Option<Vec<u8>>,
    },
    TradeList {
        // TODO
    },
    EntityPosition {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        on_ground: bool,
    },
    EntityPositionAndRotation {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        yaw: Angle,
        pitch: Angle,
        on_ground: bool,
    },
    EntityRotation {
        yaw: Angle,
        pitch: Angle,
        on_ground: bool,
    },
    VehicleMove {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
    },
    OpenBook {
        hand: VarInt, // enum
    },
    OpenWindow {
        window_id: VarInt,
        window_type: VarInt,
        window_title: Chat,
    },
    OpenSignEditor {
        location: Position,
    },
    Ping {
        id: i32,
    },
    CraftRecipeResponse {
        window_id: i8,
        recipe: Identifier,
    },
    PlayerAbilities {
        flags: i8,
        flying_speed: f32,
        fov_modifier: f32,
    },
    EndCombatEvent {
        duration: VarInt,
        entity_id: i32,
    },
    EnterCombatEvent {
        player_id: VarInt,
        entity_id: VarInt,
        message: String,
    },
    PlayerInfo {
        // TODO
    },
    Face {
        feet_eyes: VarInt, // enum
        target_x: f64,
        target_y: f64,
        target_z: f64,
        entity_id: Option<VarInt>,
        entity_feet_eyes: Option<VarInt>, // enum
    },
    PlayerPositionAndLook {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        flags: i8,
        teleport_id: VarInt,
        dismount_vehicle: bool,
    },
    UnlockRecipes {
        action: VarInt,
        crafting_recipe_book_open: bool,
        crafting_recipe_book_filter_active: bool,
        smelting_recipe_book_open: bool,
        smelting_recipe_book_filter_active: bool,
        blast_furnace_recipe_book_open: bool,
        blast_furnace_recipe_book_filter_active: bool,
        smoker_recipe_book_open: bool,
        smoker_recipe_book_filter_active: bool,
        recipe_ids: Vec<VarInt>,
        recipe_ids_init: Vec<VarInt>, // only if action == 0
    },
    DestroyEntities {
        entity_ids: Vec<VarInt>,
    },
    RemoveEntityEffect {
        entity_id: VarInt,
        effect_id: VarInt,
    },
    ResourcePackSend {
        url: String,
        hash: String,
        forced: bool,
        has_prompt_message: bool,
        prompt_message: Option<String>,
    },
    Respawn {
        dimension: Nbt,
        dimension_name: Identifier,
        hashed_seed: i64,
        gamemode: u8,
        previous_gamemode: u8,
        is_debug: bool,
        is_flag: bool,
        copy_metadata: bool,
    },
    EntityHeadLook {
        entity_id: VarInt,
        head_yaw: Angle,
    },
    MultiBlockChange {
        chunk_section_position: i64,
        trust_light_edges: bool, // always inverse of previous UpdateLight packet
        blocks_array_size: VarInt,
        blocks: Vec<VarLong>,
    },
    SelectAdvancement {
        has_id: bool,
        identifier: Option<Identifier>,
    },
    ActionBar {
        action_bar_text: String,
    },
    WorldBorderCenter {
        x: f64,
        z: f64,
    },
    WorldBorderLerpSize {
        old_diameter: f64,
        new_diameter: f64,
        speed: VarLong,
    },
    WorldBorderSize {
        diameter: f64,
    },
    WorldBorderWarningDelay {
        warning_time: VarInt,
    },
    WorldBorderWarningReach {
        warning_blocks: VarInt,
    },
    Camera {
        camera_entity_id: VarInt,
    },
    HeldItemChange {
        slot: i8,
    },
    UpdateViewPosition {
        chunk_x: VarInt,
        chunk_z: VarInt,
    },
    UpdateViewDistance {
        view_distance: VarInt,
    },
    SpawnPosition {
        location: Position,
        angle: f32,
    },
    DisplayScoreboard {
        position: i8,
        score_name: String,
    },
    EntityMetadata {
        entity_id: VarInt,
        metadata: EntityMetadata,
    },
    AttachEntity {
        attached_entity_id: i32,
        holding_entity_id: i32,
    },
    EntityVelocity {
        entity_id: VarInt,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
    },
    EntityEquipment {
        entity_id: VarInt,
        equipment: Vec<(i8, Slot)>,
    },
    SetExperience {
        experience_bar: f32,
        level: VarInt,
        total_experience: VarInt,
    },
    UpdateHealth {
        health: f32,
        food: VarInt,
        food_saturation: f32,
    },
    ScoreboardObjective {
        objective_name: String,
        mode: i8,
        objective_value: Option<String>, // rest only if mode is 0 or 2
        objective_type: Option<VarInt>,
    },
    SetPassengers {
        entity_id: VarInt,
        passenger_count: VarInt,
        passengers: Vec<VarInt>,
    },
    Teams {
        // TODO
    },
    UpdateScore {
        entity_name: String,
        action: VarInt,
        objective_name: String,
        value: Option<VarInt>, // if action != 1
    },
    UpdateSimulationDistance {
        simulation_distance: VarInt,
    },
    SetTitleSubTitle {
        subtitle_text: String,
    },
    TimeUpdate {
        world_age: i64,
        time_of_day: i64,
    },
    SetTitleText {
        title_text: String,
    },
    SetTitleTimes {
        fade_in: i32,
        stay: i32,
        fade_out: i32,
    },
    EntitySoundEffect {
        sound_id: VarInt,
        sound_category: VarInt,
        entity_id: VarInt,
        volume: f32,
        pitch: f32,
    },
    SoundEffect {
        sound_id: VarInt,
        sound_category: VarInt,
        effect_position_x: i32,
        effect_position_y: i32,
        effect_position_z: i32,
        volume: f32,
        piitch: f32,
    },
    StopSound {
        flags: u8,
        source: Option<VarInt>, // only if flags is 1 or 3 (& 0x01)
        sound: Option<Identifier>, // only if flags is 2 or 3 (& 0x02)
    },
    PlayListHeaderAndFooter {
        header: String,
        footer: String,
    },
    NbtQueryResponse {
        transaction_id: VarInt,
        nbt: Nbt,
    },
    CollectItem {
        collected_entity_id: VarInt,
        collector_entity_id: VarInt,
        pickup_item_count: VarInt,
    },
    EntityTeleport {
        entity_id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle,
        on_ground: bool,
    },
    Advancements {
        // TODO
    },
    EntityProperties {
        entity_id: VarInt,
        property_count: VarInt,
        properties: Vec<EntityProperty>,
    },
    EntityEffect {
        entity_id: VarInt,
        effect_id: VarInt,
        amplifier: i16,
        duration: VarInt,
        flags: i8,
    },
    DeclareRecipes {
        recipe_count: VarInt,
        recipes: Vec<Recipe>,
    },
    Tags {
        tag_count: VarInt,
        tags: Vec<Tag>,
    }
}