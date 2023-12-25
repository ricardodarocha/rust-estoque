use sqlx::{sqlite::SqlitePool, Pool, Sqlite};


use crate::{compras::ListaCompras};

pub async fn setup_database() -> Result<Pool<Sqlite>, sqlx::Error> {
    let database_url = "sqlite://dados.db";
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); 
    let pool = SqlitePool::connect(database_url).await?;
    Ok(pool)
}

pub async fn lista_compras(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Exemplo de consulta SQL

/*
    pub produto: Option<i64>,
    pub nome: Option<String>,
    pub quantidade: Option<f64>,
    pub unidade: Option<String>,
    pub codigo: Option<i64>,
} */


    let rows = sqlx::query_as!(ListaCompras, r#"select 
    lista_compras.codigo, 
    lista_compras.produto,
    produtos.nome, 
    sum(lista_compras.quantidade) as quantidade, 
    produtos.unidade
    from lista_compras
    join produtos on produtos.codigo = lista_compras.produto
    group by produto
    "#)
        .fetch_all(pool)
        .await?;

    // Adicionando um cabeçalho à tabela
    println!("🎁 Produto                      Quantidade  Unidade ");
    println!("=======================================================");

    // Preenchendo a tabela com os dados
    for lista in rows {
        println!("{:10} {:18} {:12} {:8}",
        lista.produto.unwrap_or_default(),
        lista.nome,
            lista.quantidade.unwrap_or_default(),
            lista.unidade.clone()
        );
    }

    Ok(())
}

