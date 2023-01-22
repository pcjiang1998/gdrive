pub mod about;
pub mod account;
pub mod app_config;
pub mod common;
pub mod files;
pub mod hub;
pub mod md5_writer;
pub mod version;

use clap::{Parser, Subcommand};
use common::chunk_size::ChunkSize;
use files::download::ExistingFileAction;
use files::list::ListQuery;
use files::list::ListSortOrder;
use mime::Mime;
use std::{error::Error, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_version_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print information about gdrive
    About,

    /// Commands for managing accounts
    Account {
        #[command(subcommand)]
        command: AccountCommand,
    },

    /// Commands for managing files
    Files {
        #[command(subcommand)]
        command: FileCommand,
    },

    /// Print version information
    Version,
}

#[derive(Subcommand)]
enum AccountCommand {
    /// Add an account
    Add,

    /// List all accounts
    List,

    /// Print current account
    Current,

    /// Switch to a different account
    Switch {
        /// Account name
        account_name: String,
    },

    /// Remove an account
    Remove {
        /// Account name
        account_name: String,
    },

    /// Export account, this will create a zip file of the account which can be imported
    Export {
        /// Account name
        account_name: String,
    },

    /// Import account that was created with the export command
    Import {
        /// Path to archive
        file_path: PathBuf,
    },
}

#[derive(Subcommand)]
enum FileCommand {
    /// Print file info
    Info {
        /// File id
        file_id: String,

        /// Display size in bytes
        #[arg(long, default_value_t = false)]
        size_in_bytes: bool,
    },

    /// List files
    List {
        /// Max files to list
        #[arg(long, default_value_t = 30)]
        max: usize,

        /// Query. See https://developers.google.com/drive/search-parameters
        #[arg(long, default_value_t = ListQuery::default())]
        query: ListQuery,

        /// Order by. See https://developers.google.com/drive/api/v3/reference/files/list
        #[arg(long, default_value_t = ListSortOrder::default())]
        order_by: ListSortOrder,

        /// List files in a specific folder
        #[arg(long, value_name = "DIRECTORY_ID")]
        parent: Option<String>,
    },

    /// Download file
    Download {
        /// File id
        file_id: String,

        /// Overwrite existing files and folders
        #[arg(long)]
        overwrite: bool,

        /// Download directories
        #[arg(long)]
        recursive: bool,

        /// Path where the file/directory should be downloaded to
        #[arg(long, value_name = "PATH")]
        destination: Option<PathBuf>,
    },

    /// Upload file
    Upload {
        /// Path of file to upload
        file_path: PathBuf,

        /// Force mime type [default: auto-detect]
        #[arg(long, value_name = "MIME_TYPE")]
        mime: Option<Mime>,

        /// Upload to an existing directory
        #[arg(long, value_name = "DIRECTORY_ID")]
        parent: Option<Vec<String>>,

        /// Upload directories. Note that this will always create a new directory on drive and will not update existing directories with the same name
        #[arg(long)]
        recursive: bool,

        /// Set chunk size in MB, must be a power of two.
        #[arg(long, value_name = "1|2|4|8|16|32|64|128|256|512|1024|4096|8192", default_value_t = ChunkSize::default())]
        chunk_size: ChunkSize,

        /// Print errors occuring during chunk upload
        #[arg(long, value_name = "", default_value_t = false)]
        print_chunk_errors: bool,

        /// Print details about each chunk
        #[arg(long, value_name = "", default_value_t = false)]
        print_chunk_info: bool,

        /// Print only id of file/folder
        #[arg(long, default_value_t = false)]
        print_only_id: bool,
    },

    /// Update file. This will create a new version of the file. The older versions will typically be kept for 30 days.
    Update {
        /// File id of the file you want ot update
        file_id: String,

        /// Path of file to upload
        file_path: PathBuf,

        /// Force mime type [default: auto-detect]
        #[arg(long, value_name = "MIME_TYPE")]
        mime: Option<Mime>,

        /// Set chunk size in MB, must be a power of two.
        #[arg(long, value_name = "1|2|4|8|16|32|64|128|256|512|1024|4096|8192", default_value_t = ChunkSize::default())]
        chunk_size: ChunkSize,

        /// Print errors occuring during chunk upload
        #[arg(long, value_name = "", default_value_t = false)]
        print_chunk_errors: bool,

        /// Print details about each chunk
        #[arg(long, value_name = "", default_value_t = false)]
        print_chunk_info: bool,
    },

    /// Delete file
    Delete {
        /// File id
        file_id: String,

        /// Delete directory and all it's content
        #[arg(long)]
        recursive: bool,
    },

    /// Create directory
    Mkdir {
        /// Name
        name: String,

        /// Create in an existing directory
        #[arg(long, value_name = "DIRECTORY_ID")]
        parent: Option<Vec<String>>,

        /// Print only id of folder
        #[arg(long, default_value_t = false)]
        print_only_id: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::About => {
            // fmt
            about::about()
        }

        Command::Account { command } => {
            // fmt
            match command {
                AccountCommand::Add => {
                    // fmt
                    account::add().await.unwrap_or_else(handle_error)
                }

                AccountCommand::List => {
                    // fmt
                    account::list().unwrap_or_else(handle_error)
                }

                AccountCommand::Current => {
                    // fmt
                    account::current().unwrap_or_else(handle_error)
                }

                AccountCommand::Switch { account_name } => {
                    // fmt
                    account::switch(account::switch::Config { account_name })
                        .unwrap_or_else(handle_error)
                }

                AccountCommand::Remove { account_name } => {
                    // fmt
                    account::remove(account::remove::Config { account_name })
                        .unwrap_or_else(handle_error)
                }

                AccountCommand::Export { account_name } => {
                    // fmt
                    account::export(account::export::Config { account_name })
                        .unwrap_or_else(handle_error)
                }

                AccountCommand::Import { file_path } => {
                    // fmt
                    account::import(account::import::Config {
                        archive_path: file_path,
                    })
                    .unwrap_or_else(handle_error)
                }
            }
        }

        Command::Files { command } => {
            match command {
                FileCommand::Info {
                    file_id,
                    size_in_bytes,
                } => {
                    // fmt
                    files::info(files::info::Config {
                        file_id,
                        size_in_bytes,
                    })
                    .await
                    .unwrap_or_else(handle_error)
                }

                FileCommand::List {
                    max,
                    query,
                    order_by,
                    parent,
                } => {
                    let q = parent
                        .map(|folder_id| ListQuery::FilesInFolder { folder_id })
                        .unwrap_or(query);

                    files::list(files::list::Config {
                        query: q,
                        order_by,
                        max_files: max,
                    })
                    .await
                    .unwrap_or_else(handle_error)
                }

                FileCommand::Download {
                    file_id,
                    overwrite,
                    recursive,
                    destination,
                } => {
                    let existing_file_action = if overwrite {
                        ExistingFileAction::Overwrite
                    } else {
                        ExistingFileAction::Abort
                    };

                    files::download(files::download::Config {
                        file_id,
                        existing_file_action,
                        download_directories: recursive,
                        destination_root: destination,
                    })
                    .await
                    .unwrap_or_else(handle_error)
                }

                FileCommand::Upload {
                    file_path,
                    mime,
                    parent,
                    recursive,
                    chunk_size,
                    print_chunk_errors,
                    print_chunk_info,
                    print_only_id,
                } => {
                    // fmt
                    files::upload(files::upload::Config {
                        file_path,
                        mime_type: mime,
                        parents: parent,
                        chunk_size,
                        print_chunk_errors,
                        print_chunk_info,
                        upload_directories: recursive,
                        print_only_id,
                    })
                    .await
                    .unwrap_or_else(handle_error)
                }

                FileCommand::Update {
                    file_id,
                    file_path,
                    mime,
                    chunk_size,
                    print_chunk_errors,
                    print_chunk_info,
                } => {
                    // fmt
                    files::update(files::update::Config {
                        file_id,
                        file_path,
                        mime_type: mime,
                        chunk_size,
                        print_chunk_errors,
                        print_chunk_info,
                    })
                    .await
                    .unwrap_or_else(handle_error)
                }

                FileCommand::Delete { file_id, recursive } => {
                    // fmt
                    files::delete(files::delete::Config {
                        file_id,
                        delete_directories: recursive,
                    })
                    .await
                    .unwrap_or_else(handle_error)
                }

                FileCommand::Mkdir {
                    name,
                    parent,
                    print_only_id,
                } => {
                    // fmt
                    files::mkdir(files::mkdir::Config {
                        id: None,
                        name,
                        parents: parent,
                        print_only_id,
                    })
                    .await
                    .unwrap_or_else(handle_error)
                }
            }
        }

        Command::Version => {
            // fmt
            version::version()
        }
    }
}

fn handle_error(err: impl Error) {
    eprintln!("Error: {}", err);
    std::process::exit(1);
}
