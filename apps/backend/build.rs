use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/player/player.proto"], &["src/proto/"])?;
    prost_build::compile_protos(
        &[
            "src/proto/control/connect.proto",
            "src/proto/control/disconnect.proto",
            "src/proto/control/heartbeat.proto",
        ],
        &["src/proto/"],
    )?;
    prost_build::compile_protos(
        &[
            "src/proto/lobby/broadcast.proto",
            "src/proto/lobby/create.proto",
            "src/proto/lobby/join.proto",
            "src/proto/lobby/list.proto",
            "src/proto/lobby/lobby.proto",
            "src/proto/lobby/quit.proto",
            "src/proto/lobby/ready.proto",
        ],
        &["src/proto/"],
    )?;
    prost_build::compile_protos(
        &[
            "src/proto/game/board.proto",
            "src/proto/game/start.proto",
            "src/proto/game/tile.proto",
            "src/proto/game/broadcast.proto",
            "src/proto/game/finish_turn.proto",
            "src/proto/game/get_new_card.proto",
            "src/proto/game/set_tile.proto",
        ],
        &["src/proto/"],
    )?;
    prost_build::compile_protos(&["src/proto/error/error.proto"], &["src/proto/"])?;
    Ok(())
}
