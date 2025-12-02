use serde::{Serialize, Deserialize};
use super::Daemon;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Procedure {
  Users(crate::x::users::procedures::Procedure)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcedureReturn {
  Users(crate::x::users::procedures::ProcedureReturn)
}


impl Procedure {
  pub async fn execute(self, daemon: &Daemon) -> ProcedureReturn {
    match self {
      Procedure::Users(it) => {
        todo!()
        // let now = daemon.state.clock.read().await.now();
        // let user_group_guard = daemon.state.users
        // ProcedureReturn::Users(
        //   it
        //     .execute(
        //       daemon.state.clock.read().await.now(), 
        //       &daemon.database, 
        //       &mut *daemon.state.users.write().await,
        //     )
        //     .await
        // )
      }
    }
  }
}