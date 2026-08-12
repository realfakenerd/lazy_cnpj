use std::{
    fs,
    path::PathBuf,
    sync::{Arc, mpsc},
};

use reqwest::Client;
use tokio::sync::Semaphore;

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

pub struct Downloader {
    client: Client,
    output_dir: PathBuf,
    max_concurrent: usize,
}

impl Downloader {
    pub fn new<P: AsRef<Path>>(output_dir: P, max_concurrent: usize) -> Self {
        Self {
            client: Client::builder()
                .user_agent("lazy_cnpj/0.1.0")
                .build()
                .unwrap_or_default(),
            output_dir: output_dir.as_ref().to_path_buf(),
            max_concurrent: max_concurrent.max(1),
        }
    }

    /// Downloads multiple targets concurrently, emitting status updates over `progress_tx`.
    pub async fn download_batch(
        &self,
        targets: Vec<DownloadTarget>,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<Vec<PathBuf>, String> {
        fs::create_dir_all(&self.output_dir)
            .await
            .map_err(|e| format!("Failed to create download directory: {}", e))?;

        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let mut tasks = Vec::new();

        for target in targets {
            let client = self.client.clone();
            let dest_path = self.output_dir.join(&target.filename);
            let tx = progress_tx.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let task = tokio::spawn(async move {
                let _permit = permit;
                let res = download_single_file(&client, &target, &dest_path, tx).await;
                res.map(|_| dest_path)
            });

            tasks.push(task);
        }

        let mut downloaded_files = Vec::new();
        for task in tasks {
            match task.await {
                Ok(Ok(path)) => downloaded_files.push(path),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(format!("Task execution panicked: {}", e)),
            }
        }

        Ok(downloaded_files)
    }
}

async fn download_single_file(
    client: &Client,
    target: &DownloadTarget,
    dest_path: &Path,
    progress_tx: mpsc::Sender<DownloadProgress>,
) -> Result<(), String> {
    let response = client
        .get(&target.url)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed for {}: {}", target.filename, e))?;

    if !response.status().is_success() {
        let err_msg = format!("HTTP error {} for {}", response.status(), target.filename);
        let _ = progress_tx
            .send(DownloadProgress::Failed {
                filename: target.filename.clone(),
                error: err_msg.clone(),
            })
            .await;
        return Err(err_msg);
    }

    let total_bytes = response.content_length();
    let _ = progress_tx
        .send(DownloadProgress::Started {
            filename: target.filename.clone(),
            total_bytes,
        })
        .await;

    let mut file = File::create(dest_path)
        .await
        .map_err(|e| format!("Failed to create destination file {:?}: {}", dest_path, e))?;

    let mut stream = response.bytes_stream();
    let mut downloaded_bytes: u64 = 0;

    while let Some(chunk_res) = stream.next().await {
        let chunk = chunk_res.map_err(|e| {
            format!(
                "Stream read error for {}:
{}",
                target.filename, e
            )
        })?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Disk write error for {}: {}", target.filename, e))?;

        downloaded_bytes += chunk.len() as u64;
        let _ = progress_tx
            .send(DownloadProgress::Progress {
                filename: target.filename.clone(),
                downloaded_bytes,
                total_bytes,
            })
            .await;
    }

    file.flush()
        .await
        .map_err(|e| format!("Failed to flush file {}: {}", target.filename, e))?;

    let _ = progress_tx
        .send(DownloadProgress::Completed {
            filename: target.filename.clone(),
            path: dest_path.to_path_buf(),
        })
        .await;

    Ok(())
}
pub fn default_cnpj_targets() -> Vec<DownloadTarget> {
    let mut targets = Vec::new();

    // Empresas0 to Empresas9
    for i in 0..=9 {
        targets.push(DownloadTarget::new(format!("Empresas{}.zip", i)));
    }
    // Estabelecimentos0 to Estabelecimentos9
    for i in 0..=9 {
        targets.push(DownloadTarget::new(format!("Estabelecimentos{}.zip", i)));
    }
    // Socios0 to Socios9
    for i in 0..=9 {
        targets.push(DownloadTarget::new(format!("Socios{}.zip", i)));
    }
    // Simples
    targets.push(DownloadTarget::new("Simples.zip"));

    // Lookups
    targets.push(DownloadTarget::new("Cnaes.zip"));
    targets.push(DownloadTarget::new("Motivos.zip"));
    targets.push(DownloadTarget::new("Municipios.zip"));
    targets.push(DownloadTarget::new("Naturezas.zip"));
    targets.push(DownloadTarget::new("Paises.zip"));
    targets.push(DownloadTarget::new("Qualificacoes.zip"));

    targets
}
