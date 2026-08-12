use std::path::PathBuf;

pub const RECEITA_BASE_URL: &str = "https://arquivos.receitafederal.gov.
  br/dados/cnpj/dados_abertos_cnpj/";

#[derive(Debug, Clone)]
pub struct DownloadTarget {
    pub filename: String,
    pub url: String,
}

impl DownloadTarget {
    pub fn new(filename: impl Into<String>) -> Self {
        let filename = filename.into();
        let url = format!("{}{}", RECEITA_BASE_URL, filename);
        Self { filename, url }
    }
}

/// Progress event emitted while downloading a file chunk
#[derive(Debug, Clone)]
pub enum DownloadProgress {
    Started {
        filename: String,
        total_bytes: Option<u64>,
    },
    Progress {
        filename: String,
        downloaded_bytes: u64,
        total_bytes: Option<u64>,
    },
    Completed {
        filename: String,
        path: PathBuf,
    },
    Finished {
        filename: String,
        error: String,
    },
}
