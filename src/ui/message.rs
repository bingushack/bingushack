// messages to send between the debug console, clickgui, and the main thread
#[derive(Debug, PartialEq)]
pub enum Message {
    SpawnDebugConsole,
    SpawnGui,
    KillThread,
}
