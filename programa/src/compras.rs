use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct ListaCompras {
    pub codigo: i64,
    pub produto: Option<i64>,
    pub nome: String,
    pub quantidade: Option<f64>,
    pub unidade: String,
}


pub async fn calcular_lista_compras(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Reunir itens de pedido e criar lista de compras
    let _rows_affected = sqlx::query!(r#"insert into lista_compras (produto, quantidade, unidade)
    SELECT 
           produto,
           itenspedido.quantidade - produtos.quantidade,
           produtos.unidade
      FROM itenspedido
           JOIN
           produtos ON itenspedido.produto = produtos.codigo
           where itenspedido.status = 'NOVO' "#)
        .execute(pool)
        .await?;

    // Alterar para processado
    let _rows_affected = sqlx::query!(r#"update itenspedido set status = 'PROCESSADO' where status ='NOVO' "#)
    .execute(pool)
    .await?;

    // Adicionando um cabeçalho à tabela
    println!("Executou com sucesso ");
    

    Ok(())
}

  