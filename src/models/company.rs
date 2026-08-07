use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Empresa {
    pub cnpj_basico: u32,
    pub razao_social: String,
    pub natureza_juridica: u32,
    pub qualificacao_responsavel: u32,
    pub porte_empresa: f64,
    pub ente_federativo_responsavel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Estabelecimento {
    pub cnpj_basico: u32,
    pub cnpj_ordem: u32,
    pub cnpj_dv: u8,
    pub identificador_matriz_filial: u8,
    pub nome_fantasia: Option<String>,
    pub situacao_cadastral: u8,
    pub data_situacao_cadastral: Option<u32>, // YYYYMMDD as u32
    pub motivo_situacao_cadastral: Option<u32>,
    pub nome_cidade_exterior: Option<String>,
    pub pais: Option<u32>,
    pub data_inicio_atividade: Option<u32>, // YYYYMMDD as u32
    pub cnae_fiscal_principal: u32,
    pub cnae_fiscal_secundaria: Option<String>,
    pub tipo_logradouro: Option<String>,
    pub logradouro: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub bairro: Option<String>,
    pub cep: Option<String>,
    pub municipio: Option<u32>,
    pub telefone_1: Option<String>,
    pub telefone_2: Option<String>,
    pub correio_eletronico: Option<String>,
    pub situacao_especial: Option<String>,
    pub data_situacao_especial: Option<u32>, // YYYYMMDD as u32
}

impl Estabelecimento {
    pub fn formatted_cnpj(&self) -> String {
        format!(
            "{:08}/{:04}-{:02}",
            self.cnpj_basico, self.cnpj_ordem, self.cnpj_dv
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DadosSimples {
    pub cnpj_basico: u32,
    pub opcao_simples: Option<u8>,       // 1: Yes, 0: No, None: Other
    pub data_opcao_simples: Option<u32>, // YYYYMMDD
    pub data_exclusao_simples: Option<u32>, // YYYYMMDD
    pub opcao_mei: Option<u8>,           // 1: Yes, 0: No, None: Other
    pub data_opcao_mei: Option<u32>,     // YYYYMMDD
    pub data_exclusao_mei: Option<u32>,  // YYYYMMDD
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Socio {
    pub id: Option<i64>,
    pub cnpj_basico: u32,
    pub identificador_socio: u8,
    pub nome_socio_razao_social: String,
    pub cnpj_cpf_socio: Option<String>,
    pub qualificacao_socio: u32,
    pub data_entrada_sociedade: Option<u32>, // YYYYMMDD
    pub pais: Option<u32>,
    pub representante_legal: Option<String>,
    pub nome_representante: Option<String>,
    pub qualificacao_representante_legal: Option<u32>,
    pub faixa_etaria: u8,
}
