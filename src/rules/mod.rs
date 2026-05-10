mod adb_unknown_command;
mod ag_literal;
mod apt_get;
mod apt_get_search;
mod apt_invalid_operation;
mod apt_list_upgradable;
mod apt_upgrade;
mod aws_cli;
mod az_cli;
mod brew_cask_dependency;
mod brew_install;
mod brew_link;
mod brew_reinstall;
mod brew_uninstall;
mod brew_unknown_command;
mod brew_update_formula;
mod cargo;
mod cargo_no_command;
mod cat_dir;
mod cd_correction;
mod cd_cs;
mod cd_mkdir;
mod cd_parent;
mod chmod_x;
mod choco_install;
mod composer_not_command;
mod conda_mistype;
mod cp_create_destination;
mod cp_omitting_directory;
mod cpp11;
mod dirty_untar;
mod dirty_unzip;
mod django_south_ghost;
mod django_south_merge;
mod dnf_no_such_command;
mod docker_image_being_used_by_container;
mod docker_login;
mod docker_not_command;
mod dry;
mod fab_command_not_found;
mod fix_alt_space;
mod fix_file;
mod gem_unknown_command;
mod git_add;
mod git_add_force;
mod git_bisect_usage;
mod git_branch_0flag;
mod git_branch_delete;
mod git_branch_delete_checked_out;
mod git_branch_exists;
mod git_branch_list;
mod git_checkout;
mod git_clone_git_clone;
mod git_clone_missing;
mod git_commit_add;
mod git_commit_amend;
mod git_commit_reset;
mod git_diff_no_index;
mod git_diff_staged;
mod git_fix_stash;
mod git_flag_after_filename;
mod git_help_aliased;
mod git_hook_bypass;
mod git_lfs_mistype;
mod git_main_master;
mod git_merge;
mod git_merge_unrelated;
mod git_not_command;
mod git_pull;
mod git_pull_clone;
mod git_pull_uncommitted_changes;
mod git_push;
mod git_push_different_branch_names;
mod git_push_force;
mod git_push_pull;
mod git_push_without_commits;
mod git_rebase_merge_dir;
mod git_rebase_no_changes;
mod git_remote_delete;
mod git_remote_seturl_add;
mod git_rm_local_modifications;
mod git_rm_recursive;
mod git_rm_staged;
mod git_stash;
mod git_stash_pop;
mod git_tag_force;
mod git_two_dashes;
mod go_run;
mod go_unknown_command;
mod gradle_no_task;
mod gradle_wrapper;
mod grep_arguments_order;
mod grep_recursive;
mod grunt_task_not_found;
mod gulp_not_task;
mod has_exists_script;
mod heroku_multiple_apps;
mod heroku_not_command;
mod history;
mod hostscli;
mod ifconfig_device_not_found;
mod java;
mod javac;
mod lein_not_task;
mod ln_no_hard_link;
mod ln_s_order;
mod long_form_help;
mod ls_all;
mod ls_lah;
mod man;
mod man_no_space;
mod mercurial;
mod missing_space_before_subcommand;
mod mkdir_p;
mod mvn_no_command;
mod mvn_unknown_lifecycle_phase;
mod nixos_cmd_not_found;
mod no_such_file;
mod npm_missing_script;
mod npm_run_script;
mod npm_wrong_command;
mod omnienv_no_such_command;
mod open;
mod pacman;
mod pacman_invalid_option;
mod pacman_not_found;
mod path_from_history;
mod php_s;
mod pip_install;
mod pip_unknown_command;
mod port_already_in_use;
mod prove_recursively;
mod python_command;
mod python_execute;
mod python_module_error;
mod quotation_marks;
mod rails_migrations_pending;
mod react_native_command_unrecognized;
mod remove_shell_prompt_literal;
mod remove_trailing_cedilla;
mod rm_dir;
mod rm_root;
mod scm_correction;
mod sed_unterminated_s;
mod sl_ls;
mod ssh_known_hosts;
mod sudo;
mod sudo_command_from_user_path;
mod systemctl;
mod terraform_init;
mod terraform_no_command;
mod test.py;
mod tmux;
mod touch;
mod tsuru_login;
mod tsuru_not_command;
mod unknown_command;
mod unsudo;
mod vagrant_up;
mod whois;
mod workon_doesnt_exists;
mod wrong_hyphen_before_subcommand;
mod yarn_alias;
mod yarn_command_not_found;
mod yarn_command_replaced;
mod yarn_help;
mod yum_invalid_operation;

pub struct Command { pub text: String, pub output: String, pub exit_code: i32 }

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn matches(&self, cmd: &Command) -> bool;
    fn fix(&self, cmd: &Command) -> String;
    fn priority(&self) -> u8 { 50 }
}

pub fn get_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(adb_unknown_command::AdbUnknownCommand),
        Box::new(ag_literal::AgLiteral),
        Box::new(apt_get::AptGet),
        Box::new(apt_get_search::AptGetSearch),
        Box::new(apt_invalid_operation::AptInvalidOperation),
        Box::new(apt_list_upgradable::AptListUpgradable),
        Box::new(apt_upgrade::AptUpgrade),
        Box::new(aws_cli::AwsCli),
        Box::new(az_cli::AzCli),
        Box::new(brew_cask_dependency::BrewCaskDependency),
        Box::new(brew_install::BrewInstall),
        Box::new(brew_link::BrewLink),
        Box::new(brew_reinstall::BrewReinstall),
        Box::new(brew_uninstall::BrewUninstall),
        Box::new(brew_unknown_command::BrewUnknownCommand),
        Box::new(brew_update_formula::BrewUpdateFormula),
        Box::new(cargo::Cargo),
        Box::new(cargo_no_command::CargoNoCommand),
        Box::new(cat_dir::CatDir),
        Box::new(cd_correction::CdCorrection),
        Box::new(cd_cs::CdCs),
        Box::new(cd_mkdir::CdMkdir),
        Box::new(cd_parent::CdParent),
        Box::new(chmod_x::ChmodX),
        Box::new(choco_install::ChocoInstall),
        Box::new(composer_not_command::ComposerNotCommand),
        Box::new(conda_mistype::CondaMistype),
        Box::new(cp_create_destination::CpCreateDestination),
        Box::new(cp_omitting_directory::CpOmittingDirectory),
        Box::new(cpp11::Cpp11),
        Box::new(dirty_untar::DirtyUntar),
        Box::new(dirty_unzip::DirtyUnzip),
        Box::new(django_south_ghost::DjangoSouthGhost),
        Box::new(django_south_merge::DjangoSouthMerge),
        Box::new(dnf_no_such_command::DnfNoSuchCommand),
        Box::new(docker_image_being_used_by_container::DockerImageBeingUsedByContainer),
        Box::new(docker_login::DockerLogin),
        Box::new(docker_not_command::DockerNotCommand),
        Box::new(dry::Dry),
        Box::new(fab_command_not_found::FabCommandNotFound),
        Box::new(fix_alt_space::FixAltSpace),
        Box::new(fix_file::FixFile),
        Box::new(gem_unknown_command::GemUnknownCommand),
        Box::new(git_add::GitAdd),
        Box::new(git_add_force::GitAddForce),
        Box::new(git_bisect_usage::GitBisectUsage),
        Box::new(git_branch_0flag::GitBranch0flag),
        Box::new(git_branch_delete::GitBranchDelete),
        Box::new(git_branch_delete_checked_out::GitBranchDeleteCheckedOut),
        Box::new(git_branch_exists::GitBranchExists),
        Box::new(git_branch_list::GitBranchList),
        Box::new(git_checkout::GitCheckout),
        Box::new(git_clone_git_clone::GitCloneGitClone),
        Box::new(git_clone_missing::GitCloneMissing),
        Box::new(git_commit_add::GitCommitAdd),
        Box::new(git_commit_amend::GitCommitAmend),
        Box::new(git_commit_reset::GitCommitReset),
        Box::new(git_diff_no_index::GitDiffNoIndex),
        Box::new(git_diff_staged::GitDiffStaged),
        Box::new(git_fix_stash::GitFixStash),
        Box::new(git_flag_after_filename::GitFlagAfterFilename),
        Box::new(git_help_aliased::GitHelpAliased),
        Box::new(git_hook_bypass::GitHookBypass),
        Box::new(git_lfs_mistype::GitLfsMistype),
        Box::new(git_main_master::GitMainMaster),
        Box::new(git_merge::GitMerge),
        Box::new(git_merge_unrelated::GitMergeUnrelated),
        Box::new(git_not_command::GitNotCommand),
        Box::new(git_pull::GitPull),
        Box::new(git_pull_clone::GitPullClone),
        Box::new(git_pull_uncommitted_changes::GitPullUncommittedChanges),
        Box::new(git_push::GitPush),
        Box::new(git_push_different_branch_names::GitPushDifferentBranchNames),
        Box::new(git_push_force::GitPushForce),
        Box::new(git_push_pull::GitPushPull),
        Box::new(git_push_without_commits::GitPushWithoutCommits),
        Box::new(git_rebase_merge_dir::GitRebaseMergeDir),
        Box::new(git_rebase_no_changes::GitRebaseNoChanges),
        Box::new(git_remote_delete::GitRemoteDelete),
        Box::new(git_remote_seturl_add::GitRemoteSeturlAdd),
        Box::new(git_rm_local_modifications::GitRmLocalModifications),
        Box::new(git_rm_recursive::GitRmRecursive),
        Box::new(git_rm_staged::GitRmStaged),
        Box::new(git_stash::GitStash),
        Box::new(git_stash_pop::GitStashPop),
        Box::new(git_tag_force::GitTagForce),
        Box::new(git_two_dashes::GitTwoDashes),
        Box::new(go_run::GoRun),
        Box::new(go_unknown_command::GoUnknownCommand),
        Box::new(gradle_no_task::GradleNoTask),
        Box::new(gradle_wrapper::GradleWrapper),
        Box::new(grep_arguments_order::GrepArgumentsOrder),
        Box::new(grep_recursive::GrepRecursive),
        Box::new(grunt_task_not_found::GruntTaskNotFound),
        Box::new(gulp_not_task::GulpNotTask),
        Box::new(has_exists_script::HasExistsScript),
        Box::new(heroku_multiple_apps::HerokuMultipleApps),
        Box::new(heroku_not_command::HerokuNotCommand),
        Box::new(history::History),
        Box::new(hostscli::Hostscli),
        Box::new(ifconfig_device_not_found::IfconfigDeviceNotFound),
        Box::new(java::Java),
        Box::new(javac::Javac),
        Box::new(lein_not_task::LeinNotTask),
        Box::new(ln_no_hard_link::LnNoHardLink),
        Box::new(ln_s_order::LnSOrder),
        Box::new(long_form_help::LongFormHelp),
        Box::new(ls_all::LsAll),
        Box::new(ls_lah::LsLah),
        Box::new(man::Man),
        Box::new(man_no_space::ManNoSpace),
        Box::new(mercurial::Mercurial),
        Box::new(missing_space_before_subcommand::MissingSpaceBeforeSubcommand),
        Box::new(mkdir_p::MkdirP),
        Box::new(mvn_no_command::MvnNoCommand),
        Box::new(mvn_unknown_lifecycle_phase::MvnUnknownLifecyclePhase),
        Box::new(nixos_cmd_not_found::NixosCmdNotFound),
        Box::new(no_such_file::NoSuchFile),
        Box::new(npm_missing_script::NpmMissingScript),
        Box::new(npm_run_script::NpmRunScript),
        Box::new(npm_wrong_command::NpmWrongCommand),
        Box::new(omnienv_no_such_command::OmnienvNoSuchCommand),
        Box::new(open::Open),
        Box::new(pacman::Pacman),
        Box::new(pacman_invalid_option::PacmanInvalidOption),
        Box::new(pacman_not_found::PacmanNotFound),
        Box::new(path_from_history::PathFromHistory),
        Box::new(php_s::PhpS),
        Box::new(pip_install::PipInstall),
        Box::new(pip_unknown_command::PipUnknownCommand),
        Box::new(port_already_in_use::PortAlreadyInUse),
        Box::new(prove_recursively::ProveRecursively),
        Box::new(python_command::PythonCommand),
        Box::new(python_execute::PythonExecute),
        Box::new(python_module_error::PythonModuleError),
        Box::new(quotation_marks::QuotationMarks),
        Box::new(rails_migrations_pending::RailsMigrationsPending),
        Box::new(react_native_command_unrecognized::ReactNativeCommandUnrecognized),
        Box::new(remove_shell_prompt_literal::RemoveShellPromptLiteral),
        Box::new(remove_trailing_cedilla::RemoveTrailingCedilla),
        Box::new(rm_dir::RmDir),
        Box::new(rm_root::RmRoot),
        Box::new(scm_correction::ScmCorrection),
        Box::new(sed_unterminated_s::SedUnterminatedS),
        Box::new(sl_ls::SlLs),
        Box::new(ssh_known_hosts::SshKnownHosts),
        Box::new(sudo::Sudo),
        Box::new(sudo_command_from_user_path::SudoCommandFromUserPath),
        Box::new(systemctl::Systemctl),
        Box::new(terraform_init::TerraformInit),
        Box::new(terraform_no_command::TerraformNoCommand),
        Box::new(test.py::Test.py),
        Box::new(tmux::Tmux),
        Box::new(touch::Touch),
        Box::new(tsuru_login::TsuruLogin),
        Box::new(tsuru_not_command::TsuruNotCommand),
        Box::new(unknown_command::UnknownCommand),
        Box::new(unsudo::Unsudo),
        Box::new(vagrant_up::VagrantUp),
        Box::new(whois::Whois),
        Box::new(workon_doesnt_exists::WorkonDoesntExists),
        Box::new(wrong_hyphen_before_subcommand::WrongHyphenBeforeSubcommand),
        Box::new(yarn_alias::YarnAlias),
        Box::new(yarn_command_not_found::YarnCommandNotFound),
        Box::new(yarn_command_replaced::YarnCommandReplaced),
        Box::new(yarn_help::YarnHelp),
        Box::new(yum_invalid_operation::YumInvalidOperation),
    ]
}
