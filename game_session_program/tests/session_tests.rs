use gstd::ActorId;
use gtest::{Program, System};
use game_session_io::*;

const USER: u64 = 10;
const WORDLE_PROGRAM_ID: u64 = 2;

#[test]
fn test_game_start_and_lose() {
    let system = System::new();
    system.init_logger();

    let session_program: Program = Program::from_file(
        &system,
        "./wasm/wasm32-unknown-unknown/debug/game_session_program.opt.wasm"
    );

    let wordle_program: Program = Program::from_file(
        &system,
        "./wasm/wasm32-unknown-unknown/debug/wordle_program.opt.wasm"
    );

    let init_wordle_program_result = wordle_program.send_bytes(USER, []);
    assert!(!init_wordle_program_result.main_failed());

    let wordle_program_address: ActorId = WORDLE_PROGRAM_ID.into();

    let init_session_program_result = session_program.send(USER, wordle_program_address);
    assert!(!init_session_program_result.main_failed());

    let start_result = session_program.send(USER, Action::StartGame);
    assert!(!start_result.main_failed());

    let state: GameStatus = session_program.read_state(()).unwrap();
    assert_eq!(state, GameStatus::InProgress {
        attempts: 0,
        start_height: 0
    });

   let check_word_result = session_program.send(USER, Action::CheckWord { word: "apple".to_string() });
   assert!(!check_word_result.main_failed());

   for _ in 0..6 {
       session_program.send(USER, Action::CheckWord { word: "apple".to_string() });
   }
   let final_state: GameStatus = session_program.read_state(()).unwrap();
   assert_eq!(final_state, GameStatus::Finished(GameResult::Lose));
}


#[test]
fn test_game_timeout() {
    let system = System::new();
    system.init_logger();

    let session_program: Program = Program::from_file(
        &system,
        "./target/wasm32-unknown-unknown/debug/game_session_program.opt.wasm"
    );

    let wordle_program: Program = Program::from_file(
        &system,
        "./target/wasm32-unknown-unknown/debug/wordle_program.opt.wasm"
    );

    let init_wordle_program_result = wordle_program.send_bytes(USER, []);
    assert!(!init_wordle_program_result.main_failed());

    let wordle_program_address: ActorId = WORDLE_PROGRAM_ID.into();

    let init_session_program_result = session_program.send(USER, wordle_program_address);
    assert!(!init_session_program_result.main_failed());

    let start_result = session_program.send(USER, Action::StartGame);
    assert!(!start_result.main_failed());

    let state: GameStatus = session_program.read_state(()).unwrap();
    assert_eq!(state, GameStatus::InProgress {
        attempts: 0,
        start_height: 0
    });

   let check_word_result = session_program.send(USER, Action::CheckWord { word: "apple".to_string() });
   assert!(!check_word_result.main_failed());

   system.spend_blocks(200);
   let timeout_state: GameStatus = session_program.read_state(()).unwrap();
   assert_eq!(timeout_state, GameStatus::Finished(GameResult::TimeOut));
}


#[test]
fn test_game_win() {
    let system = System::new();
    system.init_logger();

    let session_program: Program = Program::from_file(
        &system,
        "./target/wasm32-unknown-unknown/debug/game_session_program.opt.wasm"
    );

    let wordle_program: Program = Program::from_file(
        &system,
        "./target/wasm32-unknown-unknown/debug/wordle_program.opt.wasm"
    );

    let init_wordle_program_result = wordle_program.send_bytes(USER, []);
    assert!(!init_wordle_program_result.main_failed());

    let wordle_program_address: ActorId = WORDLE_PROGRAM_ID.into();

    let init_session_program_result = session_program.send(USER, wordle_program_address);
    assert!(!init_session_program_result.main_failed());

    let start_result = session_program.send(USER, Action::StartGame);
    assert!(!start_result.main_failed());

    let state: GameStatus = session_program.read_state(()).unwrap();
    assert_eq!(state, GameStatus::InProgress {
        attempts: 0,
        start_height: 0
    });

   let check_word_result = session_program.send(USER, Action::CheckWord { word: "apple".to_string() });
   assert!(!check_word_result.main_failed());

   let check_word_result = session_program.send(USER, Action::CheckWord { word: "human".to_string() });
   //assert!(!check_word_result.main_failed());

   let check_word_result = session_program.send(USER, Action::CheckWord { word: "horse".to_string() });
    //assert!(!check_word_result.main_failed());

   let check_word_result = session_program.send(USER, Action::CheckWord { word: "house".to_string() });
   //assert!(!check_word_result.main_failed());

   let final_state: GameStatus = session_program.read_state(()).unwrap();
   assert_eq!(final_state, GameStatus::Finished(GameResult::Win));
}