use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pais {
    pub codigo: u32,
    pub descricao: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Municipio {
    pub codigo: u32,
    pub descricao: String,
    pub uf: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualificacaoSocio {
    pub codigo: u32,
    pub descricao: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NaturezaJuridica {
    pub codigo: u32,
    pub descricao: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cnae {
    pub codigo: u32,
    pub descricao: String,
}
