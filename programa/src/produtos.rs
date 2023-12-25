use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct Produto {
    pub nome: String,
    pub grupo: Option<i64>,
    pub codigo: i64,
}

pub async fn lista_produtos(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // Exemplo de consulta SQL
        let rows = sqlx::query_as!(Produto, r#"select nome, grupo, codigo from produtos
        "#)
            .fetch_all(pool)
            .await?;
    
        // Adicionando um cabeçalho à tabela
        println!("🧵 codigo       Nome         Grupo ");
        println!("=======================================================");
    
        // Preenchendo a tabela com os dados
        for lista in rows {
            println!("{:6} {:12} {:8}",
            lista.codigo,
            lista.nome,
            lista.grupo.clone().unwrap_or_default()
            );
        }
    
        Ok(())
    }
    