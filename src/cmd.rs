// trait Command {
// 	fn execute(&self, args: (), state: State);
// }
//
//
// // Use a Null struct to initialize the remote control.
// struct NullCommand;
// impl NullCommand {
// 	fn new() -> NullCommand {
// 		NullCommand
// 	}
// }
//
// impl Command for NullCommand {
// 	fn execute(&self, args: ()) {
// 		println!("Nothing to do!");
// 	}
// }
//
//
// pub struct CmdMaker;
//
// impl CmdMaker {
//     fn new() -> CmdMaker {
//         CmdMaker
//     }
//     // fn parse(&self, cmd: String) -> "something implementing Command trait"
// }
