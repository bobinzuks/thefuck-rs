pub mod adb_unknown_command;
pub mod ag_literal;
pub mod apt_get;
pub mod apt_get_search;
pub mod apt_invalid_operation;
pub mod apt_list_upgradable;
pub mod apt_upgrade;
pub mod aws_cli;
pub mod az_cli;
pub mod brew_cask_dependency;
pub mod brew_install;
pub mod brew_link;
pub mod brew_reinstall;
pub mod brew_uninstall;
pub mod brew_unknown_command;
pub mod brew_update_formula;
pub mod cargo;
pub mod cargo_no_command;
pub mod cat_dir;
pub mod cd_correction;
pub mod cd_cs;
pub mod cd_mkdir;
pub mod cd_parent;
pub mod chmod_x;
pub mod choco_install;
pub mod composer_not_command;
pub mod conda_mistype;
pub mod cp_create_destination;
pub mod cp_omitting_directory;
pub mod cpp11;
pub mod dirty_untar;
pub mod dirty_unzip;
pub mod django_south_ghost;
pub mod django_south_merge;
pub mod dnf_no_such_command;
pub mod docker_image_being_used_by_container;
pub mod docker_login;
pub mod docker_not_command;
pub mod dry;
pub mod fab_command_not_found;
pub mod fix_alt_space;
pub mod fix_file;
pub mod gem_unknown_command;
pub mod git_add;
pub mod git_add_force;
pub mod git_bisect_usage;
pub mod git_branch;
pub mod git_branch_0flag;
pub mod git_branch_delete;
pub mod git_branch_delete_checked_out;
pub mod git_branch_exists;
pub mod git_branch_list;
pub mod git_checkout;
pub mod git_clone_git_clone;
pub mod git_clone_missing;
pub mod git_commit_add;
pub mod git_commit_amend;
pub mod git_commit_reset;
pub mod git_diff_no_index;
pub mod git_diff_staged;
pub mod git_fix_stash;
pub mod git_flag_after_filename;
pub mod git_help_aliased;
pub mod git_hook_bypass;
pub mod git_lfs_mistype;
pub mod git_main_master;
pub mod git_merge;
pub mod git_merge_unrelated;
pub mod git_not_command;
pub mod git_pull;
pub mod git_pull_clone;
pub mod git_pull_uncommitted_changes;
pub mod git_push;
pub mod git_push_different_branch_names;
pub mod git_push_force;
pub mod git_push_pull;
pub mod git_push_without_commits;
pub mod git_rebase_merge_dir;
pub mod git_rebase_no_changes;
pub mod git_remote_delete;
pub mod git_remote_seturl_add;
pub mod git_rm_local_modifications;
pub mod git_rm_recursive;
pub mod git_rm_staged;
pub mod git_stash;
pub mod git_stash_pop;
pub mod git_tag_force;
pub mod git_two_dashes;
pub mod go_run;
pub mod go_unknown_command;
pub mod gradle_no_task;
pub mod gradle_wrapper;
pub mod grep_arguments_order;
pub mod grep_recursive;
pub mod grunt_task_not_found;
pub mod gulp_not_task;
pub mod has_exists_script;
pub mod heroku_multiple_apps;
pub mod heroku_not_command;
pub mod history;
pub mod hostscli;
pub mod ifconfig_device_not_found;
pub mod java;
pub mod javac;
pub mod lein_not_task;
pub mod ln_no_hard_link;
pub mod ln_s_order;
pub mod long_form_help;
pub mod ls_all;
pub mod ls_lah;
pub mod man;
pub mod man_no_space;
pub mod mercurial;
pub mod missing_space_before_subcommand;
pub mod mkdir_p;
pub mod mvn_no_command;
pub mod mvn_unknown_lifecycle_phase;
pub mod nixos_cmd_not_found;
pub mod no_command;
pub mod no_such_file;
pub mod npm_missing_script;
pub mod npm_run_script;
pub mod npm_wrong_command;
pub mod omnienv_no_such_command;
pub mod open;
pub mod pacman;
pub mod pacman_invalid_option;
pub mod pacman_not_found;
pub mod path_from_history;
pub mod php_s;
pub mod pip_install;
pub mod pip_unknown_command;
pub mod port_already_in_use;
pub mod prove_recursively;
pub mod python_command;
pub mod python_execute;
pub mod python_module_error;
pub mod quotation_marks;
pub mod rails_migrations_pending;
pub mod react_native_command_unrecognized;
pub mod remove_shell_prompt_literal;
pub mod remove_trailing_cedilla;
pub mod rm_dir;
pub mod rm_root;
pub mod scm_correction;
pub mod sed_unterminated_s;
pub mod sl_ls;
pub mod ssh_known_hosts;
pub mod sudo;
pub mod sudo_command_from_user_path;
pub mod switch_lang;
pub mod systemctl;
pub mod terraform_init;
pub mod terraform_no_command;
pub mod tmux;
pub mod touch;
pub mod tsuru_login;
pub mod tsuru_not_command;
pub mod unknown_command;
pub mod unsudo;
pub mod vagrant_up;
pub mod whois;
pub mod workon_doesnt_exists;
pub mod wrong_hyphen_before_subcommand;
pub mod yarn_alias;
pub mod yarn_command_not_found;
pub mod yarn_command_replaced;
pub mod yarn_help;
pub mod yum_invalid_operation;

pub struct Command {
    pub text: String,
    pub output: String,
    pub exit_code: i32,
}

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn matches(&self, cmd: &Command) -> bool;
    fn fix(&self, cmd: &Command) -> String;
    fn priority(&self) -> u8 { 50 }
}

pub fn get_rules() -> Vec<Box<dyn Rule>> {
    vec![
Box::new(adb_unknown_command::AdbUnknownCommand{}),
Box::new(ag_literal::AgLiteral{}),
Box::new(apt_get::AptGet{}),
Box::new(apt_get_search::AptGetSearch{}),
Box::new(apt_invalid_operation::AptInvalidOperation{}),
Box::new(apt_list_upgradable::AptListUpgradable{}),
Box::new(apt_upgrade::AptUpgrade{}),
Box::new(aws_cli::AwsCli{}),
Box::new(az_cli::AzCli{}),
Box::new(brew_cask_dependency::BrewCaskDependency{}),
Box::new(brew_install::BrewInstall{}),
Box::new(brew_link::BrewLink{}),
Box::new(brew_reinstall::BrewReinstall{}),
Box::new(brew_uninstall::BrewUninstall{}),
Box::new(brew_unknown_command::BrewUnknownCommand{}),
Box::new(brew_update_formula::BrewUpdateFormula{}),
Box::new(cargo::Cargo{}),
Box::new(cargo_no_command::CargoNoCommand{}),
Box::new(cat_dir::CatDir{}),
Box::new(cd_correction::CdCorrection{}),
Box::new(cd_cs::CdCs{}),
Box::new(cd_mkdir::CdMkdir{}),
Box::new(cd_parent::CdParent{}),
Box::new(chmod_x::ChmodX{}),
Box::new(choco_install::ChocoInstall{}),
Box::new(composer_not_command::ComposerNotCommand{}),
Box::new(conda_mistype::CondaMistype{}),
Box::new(cp_create_destination::CpCreateDestination{}),
Box::new(cp_omitting_directory::CpOmittingDirectory{}),
Box::new(cpp11::Cpp11{}),
Box::new(dirty_untar::DirtyUntar{}),
Box::new(dirty_unzip::DirtyUnzip{}),
Box::new(django_south_ghost::DjangoSouthGhost{}),
Box::new(django_south_merge::DjangoSouthMerge{}),
Box::new(dnf_no_such_command::DnfNoSuchCommand{}),
Box::new(docker_image_being_used_by_container::DockerImageBeingUsedByContainer{}),
Box::new(docker_login::DockerLogin{}),
Box::new(docker_not_command::DockerNotCommand{}),
Box::new(dry::Dry{}),
Box::new(fab_command_not_found::FabCommandNotFound{}),
Box::new(fix_alt_space::FixAltSpace{}),
Box::new(fix_file::FixFile{}),
Box::new(gem_unknown_command::GemUnknownCommand{}),
Box::new(git_add::GitAdd{}),
Box::new(git_add_force::GitAddForce{}),
Box::new(git_bisect_usage::GitBisectUsage{}),
Box::new(git_branch::GitBranch{}),
Box::new(git_branch_0flag::GitBranch0flag{}),
Box::new(git_branch_delete::GitBranchDelete{}),
Box::new(git_branch_delete_checked_out::GitBranchDeleteCheckedOut{}),
Box::new(git_branch_exists::GitBranchExists{}),
Box::new(git_branch_list::GitBranchList{}),
Box::new(git_checkout::GitCheckout{}),
Box::new(git_clone_git_clone::GitCloneGitClone{}),
Box::new(git_clone_missing::GitCloneMissing{}),
Box::new(git_commit_add::GitCommitAdd{}),
Box::new(git_commit_amend::GitCommitAmend{}),
Box::new(git_commit_reset::GitCommitReset{}),
Box::new(git_diff_no_index::GitDiffNoIndex{}),
Box::new(git_diff_staged::GitDiffStaged{}),
Box::new(git_fix_stash::GitFixStash{}),
Box::new(git_flag_after_filename::GitFlagAfterFilename{}),
Box::new(git_help_aliased::GitHelpAliased{}),
Box::new(git_hook_bypass::GitHookBypass{}),
Box::new(git_lfs_mistype::GitLfsMistype{}),
Box::new(git_main_master::GitMainMaster{}),
Box::new(git_merge::GitMerge{}),
Box::new(git_merge_unrelated::GitMergeUnrelated{}),
Box::new(git_not_command::GitNotCommand{}),
Box::new(git_pull::GitPull{}),
Box::new(git_pull_clone::GitPullClone{}),
Box::new(git_pull_uncommitted_changes::GitPullUncommittedChanges{}),
Box::new(git_push::GitPush{}),
Box::new(git_push_different_branch_names::GitPushDifferentBranchNames{}),
Box::new(git_push_force::GitPushForce{}),
Box::new(git_push_pull::GitPushPull{}),
Box::new(git_push_without_commits::GitPushWithoutCommits{}),
Box::new(git_rebase_merge_dir::GitRebaseMergeDir{}),
Box::new(git_rebase_no_changes::GitRebaseNoChanges{}),
Box::new(git_remote_delete::GitRemoteDelete{}),
Box::new(git_remote_seturl_add::GitRemoteSeturlAdd{}),
Box::new(git_rm_local_modifications::GitRmLocalModifications{}),
Box::new(git_rm_recursive::GitRmRecursive{}),
Box::new(git_rm_staged::GitRmStaged{}),
Box::new(git_stash::GitStash{}),
Box::new(git_stash_pop::GitStashPop{}),
Box::new(git_tag_force::GitTagForce{}),
Box::new(git_two_dashes::GitTwoDashes{}),
Box::new(go_run::GoRun{}),
Box::new(go_unknown_command::GoUnknownCommand{}),
Box::new(gradle_no_task::GradleNoTask{}),
Box::new(gradle_wrapper::GradleWrapper{}),
Box::new(grep_arguments_order::GrepArgumentsOrder{}),
Box::new(grep_recursive::GrepRecursive{}),
Box::new(grunt_task_not_found::GruntTaskNotFound{}),
Box::new(gulp_not_task::GulpNotTask{}),
Box::new(has_exists_script::HasExistsScript{}),
Box::new(heroku_multiple_apps::HerokuMultipleApps{}),
Box::new(heroku_not_command::HerokuNotCommand{}),
Box::new(history::History{}),
Box::new(hostscli::Hostscli{}),
Box::new(ifconfig_device_not_found::IfconfigDeviceNotFound{}),
Box::new(java::Java{}),
Box::new(javac::Javac{}),
Box::new(lein_not_task::LeinNotTask{}),
Box::new(ln_no_hard_link::LnNoHardLink{}),
Box::new(ln_s_order::LnSOrder{}),
Box::new(long_form_help::LongFormHelp{}),
Box::new(ls_all::LsAll{}),
Box::new(ls_lah::LsLah{}),
Box::new(man::Man{}),
Box::new(man_no_space::ManNoSpace{}),
Box::new(mercurial::Mercurial{}),
Box::new(missing_space_before_subcommand::MissingSpaceBeforeSubcommand{}),
Box::new(mkdir_p::MkdirP{}),
Box::new(mvn_no_command::MvnNoCommand{}),
Box::new(mvn_unknown_lifecycle_phase::MvnUnknownLifecyclePhase{}),
Box::new(nixos_cmd_not_found::NixosCmdNotFound{}),
Box::new(no_command::NoCommand{}),
Box::new(no_such_file::NoSuchFile{}),
Box::new(npm_missing_script::NpmMissingScript{}),
Box::new(npm_run_script::NpmRunScript{}),
Box::new(npm_wrong_command::NpmWrongCommand{}),
Box::new(omnienv_no_such_command::OmnienvNoSuchCommand{}),
Box::new(open::Open{}),
Box::new(pacman::Pacman{}),
Box::new(pacman_invalid_option::PacmanInvalidOption{}),
Box::new(pacman_not_found::PacmanNotFound{}),
Box::new(path_from_history::PathFromHistory{}),
Box::new(php_s::PhpS{}),
Box::new(pip_install::PipInstall{}),
Box::new(pip_unknown_command::PipUnknownCommand{}),
Box::new(port_already_in_use::PortAlreadyInUse{}),
Box::new(prove_recursively::ProveRecursively{}),
Box::new(python_command::PythonCommand{}),
Box::new(python_execute::PythonExecute{}),
Box::new(python_module_error::PythonModuleError{}),
Box::new(quotation_marks::QuotationMarks{}),
Box::new(rails_migrations_pending::RailsMigrationsPending{}),
Box::new(react_native_command_unrecognized::ReactNativeCommandUnrecognized{}),
Box::new(remove_shell_prompt_literal::RemoveShellPromptLiteral{}),
Box::new(remove_trailing_cedilla::RemoveTrailingCedilla{}),
Box::new(rm_dir::RmDir{}),
Box::new(rm_root::RmRoot{}),
Box::new(scm_correction::ScmCorrection{}),
Box::new(sed_unterminated_s::SedUnterminatedS{}),
Box::new(sl_ls::SlLs{}),
Box::new(ssh_known_hosts::SshKnownHosts{}),
Box::new(sudo::Sudo{}),
Box::new(sudo_command_from_user_path::SudoCommandFromUserPath{}),
Box::new(switch_lang::SwitchLang{}),
Box::new(systemctl::Systemctl{}),
Box::new(terraform_init::TerraformInit{}),
Box::new(terraform_no_command::TerraformNoCommand{}),
Box::new(tmux::Tmux{}),
Box::new(touch::Touch{}),
Box::new(tsuru_login::TsuruLogin{}),
Box::new(tsuru_not_command::TsuruNotCommand{}),
Box::new(unknown_command::UnknownCommand{}),
Box::new(unsudo::Unsudo{}),
Box::new(vagrant_up::VagrantUp{}),
Box::new(whois::Whois{}),
Box::new(workon_doesnt_exists::WorkonDoesntExists{}),
Box::new(wrong_hyphen_before_subcommand::WrongHyphenBeforeSubcommand{}),
Box::new(yarn_alias::YarnAlias{}),
Box::new(yarn_command_not_found::YarnCommandNotFound{}),
Box::new(yarn_command_replaced::YarnCommandReplaced{}),
Box::new(yarn_help::YarnHelp{}),
Box::new(yum_invalid_operation::YumInvalidOperation{}),
    ]
}
