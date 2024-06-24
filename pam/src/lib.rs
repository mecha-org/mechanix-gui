use std::{collections::HashMap, ffi::CStr, path::PathBuf};

use pam::{
    constants::{PamFlag, PamResultCode, PAM_PRELIM_CHECK, PAM_PROMPT_ECHO_OFF},
    conv::Conv,
    module::{PamHandle, PamHooks},
};
use std::process::Command;

pub mod entries;
pub mod exitcode;
mod pam;
pub mod passwords;

struct PasswordModule;
pam_hooks!(PasswordModule);

impl PamHooks for PasswordModule {
    fn sm_authenticate(
        pamh: &mut PamHandle,
        args: Vec<&std::ffi::CStr>,
        flags: PamFlag,
    ) -> PamResultCode {
        let args: Vec<_> = args.iter().map(|s| s.to_string_lossy()).collect();
        let args: HashMap<&str, &str> = args
            .iter()
            .map(|s| {
                let mut parts = s.splitn(2, '=');
                (parts.next().unwrap(), parts.next().unwrap_or(""))
            })
            .collect();
        let user = pam_try!(pamh.get_user(None));
        println!(
            "PasswordModule::sm_authenticate() args {:?} user {:?} ",
            args, user,
        );

        let conv = match pamh.get_item::<Conv>() {
            Ok(Some(conv)) => conv,
            Ok(None) => {
                unreachable!("No conv available");
            }
            Err(err) => {
                println!("Couldn't get pam_conv");
                return err;
            }
        };
        let password = pam_try!(conv.send(PAM_PROMPT_ECHO_OFF, "password"));
        let password = match password {
            Some(password) => Some(pam_try!(password.to_str(), PamResultCode::PAM_AUTH_ERR)),
            None => None,
        };
        println!("Got password");

        if password.is_none() {
            println!("password not entered");
            return PamResultCode::PAM_AUTH_ERR;
        }

        let password = password.unwrap();

        // if password.is_empty() {
        //     println!("password empty");
        //     return PamResultCode::PAM_AUTH_ERR;
        // }

        let command = "mechanix-chkpwd";
        let cargs = vec!["--username", &user, "--password", password];
        let output = match Command::new(command).args(cargs).output() {
            Ok(output) => output,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };

        println!("Exit status {:?}", output.status.code());

        let success = output.status.code() == Some(0);

        if !success {
            if output.status.code() == Some(exitcode::NOUSER) {
                return PamResultCode::PAM_USER_UNKNOWN;
            }

            return PamResultCode::PAM_AUTH_ERR;
        }

        PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("account management");
        PamResultCode::PAM_SUCCESS
    }

    fn sm_chauthtok(pamh: &mut PamHandle, args: Vec<&CStr>, flags: PamFlag) -> PamResultCode {
        println!("PasswordModule::sm_chauthtok() {:?}", flags);

        if flags == PAM_PRELIM_CHECK {
            println!("returning first call");
            return PamResultCode::PAM_SUCCESS;
        }

        let user = pam_try!(pamh.get_user(None));

        println!("user is {:?}", user);

        let conv = match pamh.get_item::<Conv>() {
            Ok(Some(conv)) => conv,
            Ok(None) => {
                unreachable!("No conv available");
            }
            Err(err) => {
                println!("Couldn't get pam_conv");
                return err;
            }
        };

        let old = pam_try!(conv.send(PAM_PROMPT_ECHO_OFF, "old"));
        let old = match old {
            Some(old) => Some(pam_try!(old.to_str(), PamResultCode::PAM_AUTH_ERR)),
            None => None,
        };
        let old = old.unwrap();
        println!("Got old ");

        let new = pam_try!(conv.send(PAM_PROMPT_ECHO_OFF, "new"));
        let new = match new {
            Some(new) => Some(pam_try!(new.to_str(), PamResultCode::PAM_AUTH_ERR)),
            None => None,
        };
        let new = new.unwrap();
        println!("Got new ");

        let command = "mechanix-setpwd";
        let cargs = vec!["--username", &user, "--old", &old, "--new", &new];
        let output = match Command::new(command).args(cargs).output() {
            Ok(output) => output,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };

        println!("Exit status {:?}", output.status.code());

        let success = output.status.code() == Some(0);

        if !success {
            if output.status.code() == Some(exitcode::NOUSER) {
                return PamResultCode::PAM_USER_UNKNOWN;
            }

            return PamResultCode::PAM_AUTH_ERR;
        }

        PamResultCode::PAM_SUCCESS
    }
}
