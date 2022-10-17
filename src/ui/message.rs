// messages to send between the debug console, clickgui, and the main thread
pub enum Message {
    SpawnDebugConsole,
    SpawnGui,
    KillThread,
    RenderEvent,
}
