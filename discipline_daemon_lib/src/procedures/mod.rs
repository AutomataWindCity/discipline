use std::future::Future;
use serde::{Deserialize, Serialize};
use crate::x::{rules, users, block_info_access};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Procedure {
  AddUser(users::procedures::AddUser),
  DeleteUser(users::procedures::DeleteUser),
  SetUserName(users::procedures::SetUserName),
  
  UserDeviceAccessRegulationAddRule(rules::procedures::user_device_access_regulation_rule_group::AddRule),
  UserDeviceAccessRegulationDeleteRule(rules::procedures::user_device_access_regulation_rule_group::DeleteRule),
  UserDeviceAccessRegulationActivateRule(rules::procedures::user_device_access_regulation_rule_group::ActivateRule),
  UserDeviceAccessRegulationDeactivateRule(rules::procedures::user_device_access_regulation_rule_group::DeactivateRule),

  UserInfoAccessRegulationAddVault(block_info_access::procedures::AddVault),
  UserInfoAccessRegulationDeleteVault(block_info_access::procedures::DeleteVault),
  UserInfoAccessRegulationSetVaultName(block_info_access::procedures::SetVaultName),
  UserInfoAccessRegulationAddDatum(block_info_access::procedures::AddDatum),
  UserInfoAccessRegulationDeleteDatum(block_info_access::procedures::DeleteDatum),


  // UserAccountAccessRegulationAddRule(rules::procedures::user_account_access_regulation_rule_group::AddRule),
  // UserAccountAccessRegulationDeleteRule(rules::procedures::user_account_access_regulation_rule_group::DeleteRule),
  // UserAccountAccessRegulationActivateRule(rules::procedures::user_account_access_regulation_rule_group::ActivateRule),
  // UserAccountAccessRegulationDeactivateRule(rules::procedures::user_account_access_regulation_rule_group::DeactivateRule),

  // UserInternetAccessRegulationAddRule(rules::procedures::user_internet_access_regulation_rule_group::AddRule),
  // UserInternetAccessRegulationDeleteRule(rules::procedures::user_internet_access_regulation_rule_group::DeleteRule),
  // UserInternetAccessRegulationActivateRule(rules::procedures::user_internet_access_regulation_rule_group::ActivateRule),
  // UserInternetAccessRegulationDeactivateRule(rules::procedures::user_internet_access_regulation_rule_group::DeactivateRule),
}

pub trait Sender: Sized {
  fn send(self, value: impl Serialize) -> impl Future<Output = ()> + Send;
}

impl Procedure {
  // pub async fn execute(self, daemon: &Daemon, sender: impl Sender) {
  //   match self {
  //     Procedure::AddUser($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::DeleteUser($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::SetUserName($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::UserDeviceAccessRegulationAddRule($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::UserDeviceAccessRegulationDeleteRule($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::UserDeviceAccessRegulationActivateRule($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::UserDeviceAccessRegulationDeactivateRule($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //   }
  // }

  // pub async fn execute_and_serialize(self, daemon: &Daemon, sender: impl Sender) {
  //   match self {
  //     Procedure::AddUser($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::DeleteUser($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::SetUserName($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::UserDeviceAccessRegulationAddRule($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::UserDeviceAccessRegulationDeleteRule($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::UserDeviceAccessRegulationActivateRule($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //     Procedure::UserDeviceAccessRegulationDeactivateRule($identifier) => {
  //       sender.send(it.execute(daemon).await).await;
  //     }
  //   }
  // }

}

#[macro_export]
macro_rules! match_procedure {
  ($procedure: expr, $identifier: ident => $body: block) => {
    match $procedure {
      Procedure::AddUser($identifier) => {
        $body
      }
      Procedure::DeleteUser($identifier) => {
        $body
      }
      Procedure::SetUserName($identifier) => {
        $body
      }
      Procedure::UserDeviceAccessRegulationAddRule($identifier) => {
        $body
      }
      Procedure::UserDeviceAccessRegulationDeleteRule($identifier) => {
        $body
      }
      Procedure::UserDeviceAccessRegulationActivateRule($identifier) => {
        $body
      }
      Procedure::UserDeviceAccessRegulationDeactivateRule($identifier) => {
        $body
      }
      Procedure::UserInfoAccessRegulationAddVault($identifier) => {
        $body
      }
      Procedure::UserInfoAccessRegulationDeleteVault($identifier) => {
        $body
      }
      Procedure::UserInfoAccessRegulationSetVaultName($identifier) => {
        $body
      }
      Procedure::UserInfoAccessRegulationAddDatum($identifier) => {
        $body
      }
      Procedure::UserInfoAccessRegulationDeleteDatum($identifier) => {
        $body
      }
    }
  };
}